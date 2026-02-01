#[cfg(test)]
mod tests {
    use golden::feeds::csv::fetch::{get_bar_from_csv, get_close_price_from_csv};
    use golden::feeds::Bar;
    use std::io::Write;

    fn init_logger() {
        let _ = env_logger::Builder::new()
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
            .try_init();
    }

    #[test]
    fn test_get_bar_from_csv_basic() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();

        // Test first bar values
        assert_eq!(302.4599914550781, bars[0].open);
        assert_eq!(304.32000732421875, bars[0].close);
        assert_eq!(302.19000244140625, bars[0].low);
        assert_eq!(304.5199890136719, bars[0].high);
    }

    #[test]
    fn test_get_bar_from_csv_structure() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();

        // Verify we have data
        assert!(bars.len() > 0, "Should have at least one bar");

        // Verify Bar struct fields are accessible
        let first_bar = &bars[0];
        assert!(!first_bar.date.is_empty(), "Date should not be empty");
        assert!(first_bar.open > 0.0, "Open price should be positive");
        assert!(first_bar.high >= first_bar.open, "High should be >= open");
        assert!(first_bar.low <= first_bar.open, "Low should be <= open");
        assert!(first_bar.close > 0.0, "Close price should be positive");
    }

    #[test]
    fn test_get_bar_from_csv_ohlc_validation() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();

        // Validate OHLC relationship for all bars
        for (i, bar) in bars.iter().enumerate() {
            assert!(bar.high >= bar.open, "Bar {}: High should be >= open", i);
            assert!(bar.high >= bar.close, "Bar {}: High should be >= close", i);
            assert!(bar.high >= bar.low, "Bar {}: High should be >= low", i);
            assert!(bar.low <= bar.open, "Bar {}: Low should be <= open", i);
            assert!(bar.low <= bar.close, "Bar {}: Low should be <= close", i);
        }
    }

    #[test]
    fn test_get_bar_from_csv_nonexistent_file() {
        init_logger();

        let result = get_bar_from_csv("NONEXISTENT_SYMBOL");
        assert!(result.is_err(), "Should return error for nonexistent file");
    }

    #[test]
    fn test_get_close_price_from_csv() {
        init_logger();

        let close_prices = get_close_price_from_csv("SPY_test").unwrap();

        // Verify we have data
        assert!(close_prices.len() > 0, "Should have at least one close price");

        // Test first close price
        assert_eq!(304.32000732421875, close_prices[0]);

        // Verify all prices are positive
        for (i, &price) in close_prices.iter().enumerate() {
            assert!(price > 0.0, "Close price {} should be positive", i);
        }
    }

    #[test]
    fn test_get_close_price_consistency() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let close_prices = get_close_price_from_csv("SPY_test").unwrap();

        // Verify lengths match
        assert_eq!(bars.len(), close_prices.len(),
                   "Number of bars should match number of close prices");

        // Verify close prices match
        for (i, (bar, &close_price)) in bars.iter().zip(close_prices.iter()).enumerate() {
            assert_eq!(bar.close, close_price,
                      "Close price mismatch at index {}", i);
        }
    }

    #[test]
    fn test_bar_clone() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let first_bar = &bars[0];

        // Test that Bar can be cloned
        let cloned_bar = first_bar.clone();
        assert_eq!(first_bar.date, cloned_bar.date);
        assert_eq!(first_bar.open, cloned_bar.open);
        assert_eq!(first_bar.close, cloned_bar.close);
    }
}
