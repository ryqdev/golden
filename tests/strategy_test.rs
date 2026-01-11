#[cfg(test)]
mod tests {
    use golden::feeds::Bar;
    use golden::strategy::strategy::BaseStrategy;
    use golden::broker::backtest::backtest::Action;

    fn create_test_bar(date: &str, open: f64, high: f64, low: f64, close: f64) -> Bar {
        Bar {
            date: date.to_string(),
            open,
            high,
            low,
            close,
            volume: 0.0,
            wap: 0.0,
            count: 0,
        }
    }

    #[test]
    fn test_strategy_buy_signal() {
        let strategy = BaseStrategy::default();

        // Create a bar where close is significantly higher than open (>3% threshold)
        // close > open * 1.03 should trigger buy
        let bar = create_test_bar("2024-01-01", 100.0, 105.0, 99.0, 104.0);

        let order = strategy.next(&bar);

        match order.action {
            Action::Buy => {
                assert_eq!(order.size, 1000.0);
            },
            _ => panic!("Expected Buy action for close > open * 1.03"),
        }
    }

    #[test]
    fn test_strategy_sell_signal() {
        let strategy = BaseStrategy::default();

        // Create a bar where close is lower than open (>1% threshold)
        // close < open * 0.99 should trigger sell
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 98.0, 98.5);

        let order = strategy.next(&bar);

        match order.action {
            Action::Sell => {
                assert_eq!(order.size, 1000.0);
            },
            _ => panic!("Expected Sell action for close < open * 0.99"),
        }
    }

    #[test]
    fn test_strategy_no_action_within_threshold() {
        let strategy = BaseStrategy::default();

        // Create a bar where close is within threshold (between 0.99 and 1.03 of open)
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 99.0, 101.0);

        let order = strategy.next(&bar);

        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Expected None action for close within threshold"),
        }
    }

    #[test]
    fn test_strategy_boundary_buy() {
        let strategy = BaseStrategy::default();

        // Test exactly at buy threshold: close = open * 1.03
        let bar = create_test_bar("2024-01-01", 100.0, 103.1, 99.0, 103.0);

        let order = strategy.next(&bar);

        // At exactly 1.03, should be None (need > 1.03)
        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Expected None action at exact boundary"),
        }
    }

    #[test]
    fn test_strategy_boundary_sell() {
        let strategy = BaseStrategy::default();

        // Test exactly at sell threshold: close = open * 0.99
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 98.0, 99.0);

        let order = strategy.next(&bar);

        // At exactly 0.99, should be None (need < 0.99)
        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Expected None action at exact boundary"),
        }
    }

    #[test]
    fn test_strategy_strong_buy() {
        let strategy = BaseStrategy::default();

        // Test a very strong buy signal (close much higher than open)
        let bar = create_test_bar("2024-01-01", 100.0, 110.0, 99.0, 108.0);

        let order = strategy.next(&bar);

        match order.action {
            Action::Buy => {
                assert_eq!(order.size, 1000.0);
            },
            _ => panic!("Expected Buy action for strong upward movement"),
        }
    }

    #[test]
    fn test_strategy_strong_sell() {
        let strategy = BaseStrategy::default();

        // Test a very strong sell signal (close much lower than open)
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 95.0, 95.5);

        let order = strategy.next(&bar);

        match order.action {
            Action::Sell => {
                assert_eq!(order.size, 1000.0);
            },
            _ => panic!("Expected Sell action for strong downward movement"),
        }
    }

    #[test]
    fn test_strategy_zero_price_change() {
        let strategy = BaseStrategy::default();

        // Test when close equals open
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 99.0, 100.0);

        let order = strategy.next(&bar);

        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Expected None action when close equals open"),
        }
    }

    #[test]
    fn test_strategy_consistency() {
        let strategy = BaseStrategy::default();

        // Test that same input produces same output (idempotency)
        let bar = create_test_bar("2024-01-01", 100.0, 105.0, 99.0, 104.0);

        let order1 = strategy.next(&bar);
        let order2 = strategy.next(&bar);

        match (&order1.action, &order2.action) {
            (Action::Buy, Action::Buy) => {
                assert_eq!(order1.size, order2.size);
            },
            _ => panic!("Strategy should produce consistent results for same input"),
        }
    }

    #[test]
    fn test_strategy_default_creation() {
        let strategy = BaseStrategy::default();

        // Verify strategy can be created with default
        let bar = create_test_bar("2024-01-01", 100.0, 101.0, 99.0, 100.5);
        let _order = strategy.next(&bar);
    }
}
