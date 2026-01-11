#[cfg(test)]
mod tests {
    use golden::cmds::{parse_config, strategy_mapping};
    use std::fs;
    use std::io::Write as IoWrite;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_config_valid() {
        // Create a temporary config file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        writeln!(temp_file, "symbol = \"AAPL\"").unwrap();
        writeln!(temp_file, "time = \"Default\"").unwrap();
        writeln!(temp_file, "strategy = \"BaseStrategy\"").unwrap();
        writeln!(temp_file, "cash = 10000.0").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Should successfully parse valid config");

        let config_data = result.unwrap();
        assert_eq!(config_data.config.broker, "Backtest");
        assert_eq!(config_data.config.symbol, "AAPL");
        assert_eq!(config_data.config.time, "Default");
        assert_eq!(config_data.config.strategy, "BaseStrategy");
        assert_eq!(config_data.config.cash, 10000.0);
    }

    #[test]
    fn test_parse_config_with_underscore_in_number() {
        // Test that Rust number formatting with underscores works
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        writeln!(temp_file, "symbol = \"SPY\"").unwrap();
        writeln!(temp_file, "time = \"Default\"").unwrap();
        writeln!(temp_file, "strategy = \"Test\"").unwrap();
        writeln!(temp_file, "cash = 100_000").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Should parse config with underscore in number");

        let config_data = result.unwrap();
        assert_eq!(config_data.config.cash, 100000.0);
    }

    #[test]
    fn test_parse_config_nonexistent_file() {
        let result = parse_config("/nonexistent/path/to/config.toml");
        assert!(result.is_err(), "Should return error for nonexistent file");
    }

    #[test]
    fn test_parse_config_invalid_toml() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "this is not valid toml {{{{").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_err(), "Should return error for invalid TOML");
    }

    #[test]
    fn test_parse_config_missing_fields() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        // Missing required fields
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_err(), "Should return error for missing required fields");
    }

    #[test]
    fn test_parse_config_with_comments() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        writeln!(temp_file, "# This is a comment").unwrap();
        writeln!(temp_file, "symbol = \"AAPL\"").unwrap();
        writeln!(temp_file, "#symbol = \"TSLA\"  # Commented out alternative").unwrap();
        writeln!(temp_file, "time = \"Default\"").unwrap();
        writeln!(temp_file, "strategy = \"Test\"").unwrap();
        writeln!(temp_file, "cash = 50000.0").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Should parse config with comments");

        let config_data = result.unwrap();
        assert_eq!(config_data.config.symbol, "AAPL");
        assert_eq!(config_data.config.cash, 50000.0);
    }

    #[test]
    fn test_parse_config_different_symbols() {
        let symbols = vec!["AAPL", "SPY", "002714.SZ", "002142.SZ", "TSLA"];

        for symbol in symbols {
            let mut temp_file = NamedTempFile::new().unwrap();
            writeln!(temp_file, "[config]").unwrap();
            writeln!(temp_file, "broker = \"Backtest\"").unwrap();
            writeln!(temp_file, "symbol = \"{}\"", symbol).unwrap();
            writeln!(temp_file, "time = \"Default\"").unwrap();
            writeln!(temp_file, "strategy = \"Test\"").unwrap();
            writeln!(temp_file, "cash = 10000.0").unwrap();
            temp_file.flush().unwrap();

            let result = parse_config(temp_file.path().to_str().unwrap());
            assert!(result.is_ok(), "Should parse config with symbol {}", symbol);

            let config_data = result.unwrap();
            assert_eq!(config_data.config.symbol, symbol);
        }
    }

    #[test]
    fn test_parse_config_various_cash_amounts() {
        let cash_amounts = vec![1000.0, 10000.0, 50000.0, 100000.0, 1000000.0];

        for cash in cash_amounts {
            let mut temp_file = NamedTempFile::new().unwrap();
            writeln!(temp_file, "[config]").unwrap();
            writeln!(temp_file, "broker = \"Backtest\"").unwrap();
            writeln!(temp_file, "symbol = \"SPY\"").unwrap();
            writeln!(temp_file, "time = \"Default\"").unwrap();
            writeln!(temp_file, "strategy = \"Test\"").unwrap();
            writeln!(temp_file, "cash = {}", cash).unwrap();
            temp_file.flush().unwrap();

            let result = parse_config(temp_file.path().to_str().unwrap());
            assert!(result.is_ok(), "Should parse config with cash {}", cash);

            let config_data = result.unwrap();
            assert_eq!(config_data.config.cash, cash);
        }
    }

    #[test]
    fn test_strategy_mapping() {
        // Test that strategy_mapping returns a BaseStrategy
        let strategy = strategy_mapping("Test");

        // We can't directly compare strategies, but we can test that it returns successfully
        // and creates a valid BaseStrategy object
        let _ = format!("{:?}", strategy);
    }

    #[test]
    fn test_strategy_mapping_different_names() {
        let strategy_names = vec!["Test", "BaseStrategy", "Custom", "Default"];

        for name in strategy_names {
            let strategy = strategy_mapping(name);
            // Verify strategy can be created for any name
            let _ = format!("{:?}", strategy);
        }
    }

    #[test]
    fn test_parse_config_from_actual_file() {
        // Test parsing the actual config.toml file if it exists
        if fs::metadata("config.toml").is_ok() {
            let result = parse_config("config.toml");
            assert!(result.is_ok(), "Should successfully parse config.toml");

            let config_data = result.unwrap();
            assert!(!config_data.config.broker.is_empty());
            assert!(!config_data.config.symbol.is_empty());
            assert!(config_data.config.cash > 0.0);
        }
    }

    #[test]
    fn test_parse_config_zero_cash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        writeln!(temp_file, "symbol = \"SPY\"").unwrap();
        writeln!(temp_file, "time = \"Default\"").unwrap();
        writeln!(temp_file, "strategy = \"Test\"").unwrap();
        writeln!(temp_file, "cash = 0.0").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Should parse config with zero cash");

        let config_data = result.unwrap();
        assert_eq!(config_data.config.cash, 0.0);
    }

    #[test]
    fn test_parse_config_negative_cash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "[config]").unwrap();
        writeln!(temp_file, "broker = \"Backtest\"").unwrap();
        writeln!(temp_file, "symbol = \"SPY\"").unwrap();
        writeln!(temp_file, "time = \"Default\"").unwrap();
        writeln!(temp_file, "strategy = \"Test\"").unwrap();
        writeln!(temp_file, "cash = -1000.0").unwrap();
        temp_file.flush().unwrap();

        let result = parse_config(temp_file.path().to_str().unwrap());
        assert!(result.is_ok(), "Should parse config with negative cash");

        let config_data = result.unwrap();
        assert_eq!(config_data.config.cash, -1000.0);
    }
}
