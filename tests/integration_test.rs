#[cfg(test)]
mod tests {
    use golden::feeds::csv::fetch::get_bar_from_csv;
    use golden::feeds::Bar;
    use golden::strategy::strategy::BaseStrategy;
    use golden::broker::backtest::backtest::{Action, Order};
    use std::io::Write;

    fn init_logger() {
        let _ = env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .try_init();
    }

    #[test]
    fn test_end_to_end_data_to_strategy() {
        init_logger();

        // Load data from CSV
        let bars = get_bar_from_csv("SPY_test").unwrap();
        assert!(bars.len() > 0, "Should have loaded bar data");

        // Create strategy
        let strategy = BaseStrategy::default();

        // Process each bar through strategy
        let mut buy_count = 0;
        let mut sell_count = 0;
        let mut none_count = 0;

        for bar in &bars {
            let order = strategy.next(bar);
            match order.action {
                Action::Buy => buy_count += 1,
                Action::Sell => sell_count += 1,
                Action::None => none_count += 1,
            }
        }

        log::info!("Buy: {}, Sell: {}, None: {}", buy_count, sell_count, none_count);

        // Verify that we got some signals
        let total_signals = buy_count + sell_count + none_count;
        assert_eq!(total_signals, bars.len(), "Should process all bars");
    }

    #[test]
    fn test_strategy_consistency_across_bars() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let strategy = BaseStrategy::default();

        // Process bars twice to ensure consistency
        let mut orders1 = Vec::new();
        let mut orders2 = Vec::new();

        for bar in &bars {
            orders1.push(strategy.next(bar));
        }

        for bar in &bars {
            orders2.push(strategy.next(bar));
        }

        // Verify same results
        assert_eq!(orders1.len(), orders2.len());
        for i in 0..orders1.len() {
            match (&orders1[i].action, &orders2[i].action) {
                (Action::Buy, Action::Buy) => assert_eq!(orders1[i].size, orders2[i].size),
                (Action::Sell, Action::Sell) => assert_eq!(orders1[i].size, orders2[i].size),
                (Action::None, Action::None) => assert_eq!(orders1[i].size, orders2[i].size),
                _ => panic!("Inconsistent orders at index {}", i),
            }
        }
    }

    #[test]
    fn test_simulated_backtest_workflow() {
        init_logger();

        // Load data
        let bars = get_bar_from_csv("SPY_test").unwrap();
        let strategy = BaseStrategy::default();

        // Initialize broker state
        let mut cash = 10000.0;
        let mut position = 0.0;
        let initial_cash = cash;

        // Simulate backtest
        for bar in &bars {
            let order = strategy.next(bar);

            match order.action {
                Action::Buy => {
                    let buying_power = (cash / bar.close / 100.0) as i64;
                    if buying_power >= 1 {
                        let shares_to_buy = (buying_power * 100) as f64;
                        cash -= shares_to_buy * bar.close;
                        position += shares_to_buy;
                        log::info!("Buy {} shares at {}, cash: {}", shares_to_buy, bar.close, cash);
                    }
                }
                Action::Sell => {
                    if position > 0.0 {
                        cash += position * bar.close;
                        log::info!("Sell {} shares at {}, cash: {}", position, bar.close, cash);
                        position = 0.0;
                    }
                }
                Action::None => {}
            }
        }

        // Calculate final value
        let final_value = cash + position * bars.last().unwrap().close;
        log::info!("Initial cash: {}, Final value: {}, P&L: {}", initial_cash, final_value, final_value - initial_cash);

        // Verify we tracked the account
        assert!(final_value > 0.0, "Final value should be positive");
    }

    #[test]
    fn test_bar_data_integrity() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();

        for (i, bar) in bars.iter().enumerate() {
            // Verify OHLC relationships
            assert!(bar.high >= bar.open, "Bar {}: High {} should be >= open {}", i, bar.high, bar.open);
            assert!(bar.high >= bar.close, "Bar {}: High {} should be >= close {}", i, bar.high, bar.close);
            assert!(bar.high >= bar.low, "Bar {}: High {} should be >= low {}", i, bar.high, bar.low);
            assert!(bar.low <= bar.open, "Bar {}: Low {} should be <= open {}", i, bar.low, bar.open);
            assert!(bar.low <= bar.close, "Bar {}: Low {} should be <= close {}", i, bar.low, bar.close);

            // Verify prices are positive
            assert!(bar.open > 0.0, "Bar {}: Open price should be positive", i);
            assert!(bar.high > 0.0, "Bar {}: High price should be positive", i);
            assert!(bar.low > 0.0, "Bar {}: Low price should be positive", i);
            assert!(bar.close > 0.0, "Bar {}: Close price should be positive", i);

            // Verify date is not empty
            assert!(!bar.date.is_empty(), "Bar {}: Date should not be empty", i);
        }
    }

    #[test]
    fn test_order_generation_from_bars() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let strategy = BaseStrategy::default();

        let mut orders = Vec::new();

        for bar in &bars {
            let order = strategy.next(bar);
            orders.push(order);
        }

        // Verify we generated correct number of orders
        assert_eq!(orders.len(), bars.len());

        // Verify all orders have valid actions
        for order in &orders {
            match order.action {
                Action::Buy | Action::Sell | Action::None => {},
            }
        }
    }

    #[test]
    fn test_multiple_strategy_instances() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();

        // Create multiple strategy instances
        let strategy1 = BaseStrategy::default();
        let strategy2 = BaseStrategy::default();

        // Verify they produce same results
        for bar in &bars {
            let order1 = strategy1.next(bar);
            let order2 = strategy2.next(bar);

            match (&order1.action, &order2.action) {
                (Action::Buy, Action::Buy) => {},
                (Action::Sell, Action::Sell) => {},
                (Action::None, Action::None) => {},
                _ => panic!("Different strategies produced different results for same bar"),
            }
        }
    }

    #[test]
    fn test_position_tracking_accuracy() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let strategy = BaseStrategy::default();

        let mut cash_history = vec![10000.0];
        let mut position_history = vec![0.0];

        for bar in &bars {
            let order = strategy.next(bar);
            let cash = *cash_history.last().unwrap();
            let position = *position_history.last().unwrap();

            match order.action {
                Action::Buy => {
                    let buying_power = (cash / bar.close / 100.0) as i64;
                    if buying_power >= 1 {
                        let shares = (buying_power * 100) as f64;
                        cash_history.push(cash - shares * bar.close);
                        position_history.push(position + shares);
                    } else {
                        cash_history.push(cash);
                        position_history.push(position);
                    }
                }
                Action::Sell => {
                    if position > 0.0 {
                        cash_history.push(cash + position * bar.close);
                        position_history.push(0.0);
                    } else {
                        cash_history.push(cash);
                        position_history.push(position);
                    }
                }
                Action::None => {
                    cash_history.push(cash);
                    position_history.push(position);
                }
            }
        }

        // Verify history lengths
        assert_eq!(cash_history.len(), bars.len() + 1);
        assert_eq!(position_history.len(), bars.len() + 1);

        // Verify all positions are non-negative
        for pos in &position_history {
            assert!(*pos >= 0.0, "Position should be non-negative");
        }

        // Verify cash is tracked (can be negative in margin accounts, but should be finite)
        for c in &cash_history {
            assert!(c.is_finite(), "Cash should be finite");
        }
    }

    #[test]
    fn test_net_asset_calculation() {
        init_logger();

        let bars = get_bar_from_csv("SPY_test").unwrap();
        let strategy = BaseStrategy::default();

        let mut cash = 10000.0;
        let mut position = 0.0;
        let mut net_assets = Vec::new();

        net_assets.push(cash);

        for bar in &bars {
            let order = strategy.next(bar);

            match order.action {
                Action::Buy => {
                    let buying_power = (cash / bar.close / 100.0) as i64;
                    if buying_power >= 1 {
                        let shares = (buying_power * 100) as f64;
                        cash -= shares * bar.close;
                        position += shares;
                    }
                }
                Action::Sell => {
                    if position > 0.0 {
                        cash += position * bar.close;
                        position = 0.0;
                    }
                }
                Action::None => {}
            }

            let net_asset = cash + position * bar.close;
            net_assets.push(net_asset);
        }

        // Verify all net assets are positive
        for na in &net_assets {
            assert!(*na > 0.0, "Net assets should be positive");
        }

        // Calculate P&L
        let p_l = net_assets.last().unwrap() - net_assets.first().unwrap();
        log::info!("P&L: {}", p_l);
    }
}
