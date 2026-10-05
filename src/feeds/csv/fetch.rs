use std::fs;
use std::path::{Path, PathBuf};
use crate::feeds::Bar;
use anyhow::{bail, Context, Result};
use reqwest::StatusCode;
use super::YFinance;

/// Directory (relative to the current working directory) where csv files are stored.
const DATA_DIR: &str = "data";

/// Longest symbol we accept. Real tickers are far shorter, this only bounds the input.
const MAX_SYMBOL_LEN: usize = 20;

/// Header of the csv we write. The default header from yahoo finance has capital letters
/// like Date, Open, High .....  Most scenarios need the lower case, so we use:
/// date,open,high,low,close,adj_close,volume
const CSV_HEADER: [&str; 7] = ["date", "open", "high", "low", "close", "adj_close", "volume"];

/// Header of the csv returned by yahoo finance. Used to tell a real csv from an error page.
const YAHOO_HEADER: [&str; 7] = ["Date", "Open", "High", "Low", "Close", "Adj Close", "Volume"];

/// Currently, the period is from 2018/01/01:00:00:00 to 2024/01/01:00:00:00 (UTC+8)
/// TODO: add more configuration in the future
const YAHOO_PERIOD1: i64 = 1514736000;
const YAHOO_PERIOD2: i64 = 1704038400;

/// The symbol ends up in a file path (`data/{symbol}.csv`) and in a request url, so only allow
/// the characters real tickers are made of: ASCII letters/digits and `. ^ = _ -`
/// (e.g. `SPY`, `002714.SZ`, `BRK-B`, `^GSPC`, `EURUSD=X`).
/// This rejects path separators, `..`, `?`, `#`, `%`, whitespace and so on.
pub fn validate_symbol(symbol: &str) -> Result<()> {
    let valid = !symbol.is_empty()
        && symbol.len() <= MAX_SYMBOL_LEN
        && !symbol.starts_with('.')
        && symbol.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '^' | '=' | '_' | '-'));
    if !valid {
        bail!(
            "invalid symbol {symbol:?}: expected 1-{MAX_SYMBOL_LEN} characters from [A-Za-z0-9.^=_-] \
             and not starting with '.'"
        );
    }
    Ok(())
}

/// Path of the csv file for `symbol`: `data/{symbol}.csv`. The symbol is validated first.
pub fn csv_path(symbol: &str) -> Result<PathBuf> {
    validate_symbol(symbol)?;
    Ok(Path::new(DATA_DIR).join(format!("{symbol}.csv")))
}

fn read_rows(symbol: &str) -> Result<Vec<YFinance>> {
    let path = csv_path(symbol)?;
    let file = fs::File::open(&path).with_context(|| {
        format!("cannot open {}, download it first with `golden csv --symbol {symbol}`", path.display())
    })?;
    let rows = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file)
        .deserialize::<YFinance>()
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("cannot parse {}", path.display()))?;
    if rows.is_empty() {
        bail!(
            "{} has no data rows, download it again with `golden csv --symbol {symbol}`",
            path.display()
        );
    }
    Ok(rows)
}

// https://docs.rs/csv/latest/csv/struct.Reader.html
pub fn get_bar_from_csv(symbol: &str) -> Result<Vec<Bar>> {
    // `golden backtest` downloads the csv when it does not exist yet.
    Ok(read_rows(symbol)?
        .into_iter()
        .map(|record| Bar {
            date: record.date,
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            // leave volume, wap and count blank
            volume: 0.0,
            wap: 0.0,
            count: 0,
        })
        .collect())
}

pub fn get_close_price_from_csv(symbol: &str) -> Result<Vec<f64>> {
    Ok(read_rows(symbol)?.into_iter().map(|record| record.close).collect())
}

/// First characters of a response body, for error messages.
fn snippet(body: &str) -> String {
    let s: String = body.chars().take(200).collect();
    s.trim().replace(['\r', '\n'], " ")
}

/// Parse the csv returned by yahoo finance.
///
/// Fails (instead of returning an empty list) if the body is not a yahoo finance csv, e.g. the
/// "Too Many Requests" text or a json error, or if it contains no usable row at all.
/// Rows whose fields are `null` (yahoo returns them for days without data) are skipped.
fn parse_yahoo_csv(body: &str) -> Result<Vec<YFinance>> {
    let mut reader = csv::ReaderBuilder::new().from_reader(body.as_bytes());

    let header_ok = reader.headers().map_or(false, |h| {
        h.len() == YAHOO_HEADER.len() && h.iter().zip(YAHOO_HEADER).all(|(a, b)| a.trim().eq_ignore_ascii_case(b))
    });
    if !header_ok {
        bail!("unexpected response from yahoo finance (not a csv): {}", snippet(body));
    }

    let mut rows = Vec::new();
    for record in reader.records() {
        let record = record.context("invalid csv from yahoo finance")?;
        let line = record.position().map_or(0, |p| p.line());
        if record.iter().any(|field| field.trim().eq_ignore_ascii_case("null")) {
            log::warn!("skip row without data at line {line}: {:?}", record);
            continue;
        }
        rows.push(
            record
                .deserialize::<YFinance>(None)
                .with_context(|| format!("invalid row at line {line} from yahoo finance: {:?}", record))?,
        );
    }

    if rows.is_empty() {
        bail!("yahoo finance returned no data rows");
    }
    Ok(rows)
}

/// Write `rows` to `path`, creating the parent directory if needed.
/// Writes to a temporary file first, so a failure never leaves a half-written or empty csv
/// in place of an existing good one.
fn save_csv_file(path: &Path, rows: &[YFinance]) -> Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("cannot create directory {}", dir.display()))?;
    }

    let tmp = path.with_extension("csv.tmp");
    let written = (|| -> Result<()> {
        let mut wtr = csv::WriterBuilder::new().from_path(&tmp)?;
        wtr.write_record(CSV_HEADER)?;
        for record in rows {
            wtr.write_record(&[
                &record.date,
                &record.open.to_string(),
                &record.high.to_string(),
                &record.low.to_string(),
                &record.close.to_string(),
                &record.adj_close.to_string(),
                &record.volume.to_string(),
            ])?;
        }
        wtr.flush()?;
        drop(wtr);
        fs::rename(&tmp, path)?;
        Ok(())
    })();

    if written.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    written.with_context(|| format!("cannot write {}", path.display()))
}

/// Turn a yahoo finance response into rows and, only if that succeeded, save them to `save_to`.
/// A failed response never touches an existing file.
fn handle_yahoo_response(
    symbol: &str,
    status: StatusCode,
    body: &str,
    save_to: Option<&Path>,
) -> Result<Vec<YFinance>> {
    if !status.is_success() {
        bail!("yahoo finance returned HTTP {status} for {symbol}: {}", snippet(body));
    }
    let rows = parse_yahoo_csv(body).with_context(|| format!("cannot download data for {symbol}"))?;
    if let Some(path) = save_to {
        save_csv_file(path, &rows)?;
    }
    Ok(rows)
}

/// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
/// Example url to download historial csv data: https://query1.finance.yahoo.com/v7/finance/download/TLT?period1=345479400&period2=1717257709&interval=1d&events=history&includeAdjustedClose=true
///
/// It works well in brower `but` met Status 429 with `reqwest` GET RESTful request.
///
/// Solution: https://stackoverflow.com/questions/78111453/yahoo-finance-api-file-get-contents-429-too-many-requests
/// Add User-Agent to solve `429` problem
///
/// Returns an error, and leaves any existing `data/{symbol}.csv` untouched, if the request fails
/// or the response is not a valid csv with data.
pub async fn get_bar_from_yahoo(symbol: &str, save_csv: bool) -> Result<Vec<YFinance>> {
    validate_symbol(symbol)?;
    let save_to = if save_csv { Some(csv_path(symbol)?) } else { None };

    let url = format!("https://query1.finance.yahoo.com/v7/finance/download/{symbol}?period1={YAHOO_PERIOD1}&period2={YAHOO_PERIOD2}&interval=1d&events=history&includeAdjustedClose=true");

    let client = reqwest::Client::builder()
        .user_agent("curl/7.68.0")
        .build()?;

    let response = client.get(url).send().await?;
    let status = response.status();
    log::info!("Status Code: {}", status);
    let response_body = response.text().await?;
    log::debug!("Response body: {}", response_body);

    handle_yahoo_response(symbol, status, &response_body, save_to.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD_BODY: &str = "Date,Open,High,Low,Close,Adj Close,Volume\n\
        2023-01-03,10.5,11,10,10.8,10.7,1000\n\
        2023-01-04,10.8,11.2,10.6,11,10.9,2000\n";

    /// A fresh, empty directory under the system temp dir.
    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("golden-test-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn validate_symbol_accepts_real_tickers() {
        for s in ["SPY", "AAPL", "002714.SZ", "BRK-B", "^GSPC", "EURUSD=X", "SPY_test", "A.B.C"] {
            assert!(validate_symbol(s).is_ok(), "{s} should be valid");
        }
    }

    #[test]
    fn validate_symbol_rejects_paths_and_urls() {
        for s in [
            "", ".", "..", "../x", "../../etc/passwd", "a/b", "a\\b", "/abs", "a?b=c", "a#b", "a%2fb",
            "a b", "a\nb", ".hidden", "股票", &"A".repeat(MAX_SYMBOL_LEN + 1),
        ] {
            assert!(validate_symbol(s).is_err(), "{s:?} should be rejected");
        }
    }

    #[test]
    fn csv_path_is_inside_data_dir() {
        assert_eq!(csv_path("SPY").unwrap(), Path::new("data").join("SPY.csv"));
        assert!(csv_path("../SPY").is_err());
    }

    #[test]
    fn read_functions_reject_bad_symbol() {
        assert!(get_bar_from_csv("../Cargo").is_err());
        assert!(get_close_price_from_csv("../Cargo").is_err());
    }

    #[test]
    fn parse_good_csv() {
        let rows = parse_yahoo_csv(GOOD_BODY).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].date, "2023-01-03");
        assert_eq!(rows[1].close, 11.0);
        assert_eq!(rows[1].volume, 2000);
    }

    #[test]
    fn parse_skips_null_rows() {
        let body = format!("{GOOD_BODY}2023-01-05,null,null,null,null,null,null\n");
        assert_eq!(parse_yahoo_csv(&body).unwrap().len(), 2);
    }

    #[test]
    fn parse_rejects_non_csv_bodies() {
        for body in [
            "Too Many Requests",
            "404 Not Found: No data found, symbol may be delisted",
            "{\"finance\":{\"error\":{\"code\":\"Unauthorized\"}}}",
            "",
        ] {
            assert!(parse_yahoo_csv(body).is_err(), "{body:?} should be rejected");
        }
    }

    #[test]
    fn parse_rejects_header_only_and_all_null() {
        let header = "Date,Open,High,Low,Close,Adj Close,Volume\n";
        assert!(parse_yahoo_csv(header).is_err());
        assert!(parse_yahoo_csv(&format!("{header}2023-01-05,null,null,null,null,null,null\n")).is_err());
    }

    #[test]
    fn parse_reports_bad_row_instead_of_panicking() {
        let body = format!("{GOOD_BODY}2023-01-05,abc,1,1,1,1,1\n");
        let err = format!("{:#}", parse_yahoo_csv(&body).unwrap_err());
        assert!(err.contains("line 4"), "{err}");
    }

    #[test]
    fn save_creates_missing_directory_and_roundtrips() {
        let dir = temp_dir("save");
        let path = dir.join("nested").join("SPY.csv");
        let rows = parse_yahoo_csv(GOOD_BODY).unwrap();

        save_csv_file(&path, &rows).unwrap();

        let saved = fs::read_to_string(&path).unwrap();
        assert!(saved.starts_with("date,open,high,low,close,adj_close,volume\n2023-01-03,10.5,11,10,10.8,10.7,1000\n"));
        assert!(!path.with_extension("csv.tmp").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn failed_response_does_not_overwrite_existing_file() {
        let dir = temp_dir("keep");
        let path = dir.join("SPY.csv");
        fs::write(&path, "date,open,high,low,close,adj_close,volume\n2023-01-03,1,1,1,1,1,1\n").unwrap();
        let before = fs::read(&path).unwrap();

        for (status, body) in [
            (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests"),
            (StatusCode::NOT_FOUND, "404 Not Found: No data found, symbol may be delisted"),
            (StatusCode::OK, "Too Many Requests"),
            (StatusCode::OK, "Date,Open,High,Low,Close,Adj Close,Volume\n"),
        ] {
            assert!(handle_yahoo_response("SPY", status, body, Some(&path)).is_err());
            assert_eq!(fs::read(&path).unwrap(), before, "{status} {body:?} must not touch the file");
        }
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn good_response_is_saved_only_when_asked() {
        let dir = temp_dir("good");

        let not_saved = dir.join("not_saved").join("SPY.csv");
        let rows = handle_yahoo_response("SPY", StatusCode::OK, GOOD_BODY, None).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(!not_saved.parent().unwrap().exists());

        let saved = dir.join("saved").join("SPY.csv");
        handle_yahoo_response("SPY", StatusCode::OK, GOOD_BODY, Some(&saved)).unwrap();
        assert!(saved.exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn error_message_names_status_and_symbol() {
        let err = handle_yahoo_response("SPY", StatusCode::TOO_MANY_REQUESTS, "Too Many Requests", None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("429") && err.contains("SPY") && err.contains("Too Many Requests"), "{err}");
    }
}
