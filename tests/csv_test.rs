#[cfg(test)]
mod tests {
    use golden::feeds::csv::fetch::get_bar_from_csv;
    use std::io::Write;

    #[test]
    fn get_bar_from_csv_test() {
        /// Unfortunately, Rust don't have a built-in tool for initialization before `cargo test`

        // init log config
        env_logger::Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} [{}] - {}",
                    record.file().unwrap_or("unknown_file"),
                    record.line().unwrap_or(0),
                    record.level(),
                    record.args()
                )
            })
            .filter_level(log::LevelFilter::Info)
            .init();

        // sample data from /data/SPY_test.csv
        assert_eq!(302.4599914550781, get_bar_from_csv("SPY_test").unwrap()[0].open);
        assert_eq!(304.32000732421875, get_bar_from_csv("SPY_test").unwrap()[0].close);
    }
}
