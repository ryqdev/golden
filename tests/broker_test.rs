#[cfg(test)]
mod tests {
    use golden::broker::backtest::backtest::{Action, Order};

    #[test]
    fn test_order_default() {
        let order = Order::default();

        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Default order should have None action"),
        }
    }

    #[test]
    fn test_order_buy_creation() {
        let order = Order {
            action: Action::Buy,
            size: 100.0,
        };

        match order.action {
            Action::Buy => {
                assert_eq!(order.size, 100.0);
            },
            _ => panic!("Order should have Buy action"),
        }
    }

    #[test]
    fn test_order_sell_creation() {
        let order = Order {
            action: Action::Sell,
            size: 50.0,
        };

        match order.action {
            Action::Sell => {
                assert_eq!(order.size, 50.0);
            },
            _ => panic!("Order should have Sell action"),
        }
    }

    #[test]
    fn test_order_clone() {
        let order = Order {
            action: Action::Buy,
            size: 100.0,
        };

        let cloned_order = order.clone();

        match (&order.action, &cloned_order.action) {
            (Action::Buy, Action::Buy) => {
                assert_eq!(order.size, cloned_order.size);
            },
            _ => panic!("Cloned order should match original"),
        }
    }

    #[test]
    fn test_action_default() {
        let action = Action::default();

        match action {
            Action::None => {},
            _ => panic!("Default action should be None"),
        }
    }

    #[test]
    fn test_action_copy() {
        let action1 = Action::Buy;
        let action2 = action1;

        match (action1, action2) {
            (Action::Buy, Action::Buy) => {},
            _ => panic!("Copied action should match original"),
        }
    }

    #[test]
    fn test_order_none_creation() {
        let order = Order {
            action: Action::None,
            size: 0.0,
        };

        match order.action {
            Action::None => {
                assert_eq!(order.size, 0.0);
            },
            _ => panic!("Order should have None action"),
        }
    }

    #[test]
    fn test_order_debug_format() {
        let order = Order {
            action: Action::Buy,
            size: 100.0,
        };

        let debug_str = format!("{:?}", order);
        assert!(debug_str.contains("Order"));
    }

    #[test]
    fn test_action_clone() {
        let action1 = Action::Sell;
        let action2 = action1.clone();

        match (action1, action2) {
            (Action::Sell, Action::Sell) => {},
            _ => panic!("Cloned action should match original"),
        }
    }

    #[test]
    fn test_action_debug_format() {
        let action = Action::Buy;
        let debug_str = format!("{:?}", action);
        assert!(debug_str.contains("Buy"));
    }

    #[test]
    fn test_action_variants() {
        let none = Action::None;
        let buy = Action::Buy;
        let sell = Action::Sell;

        match none {
            Action::None => {},
            _ => panic!("Should be None variant"),
        }

        match buy {
            Action::Buy => {},
            _ => panic!("Should be Buy variant"),
        }

        match sell {
            Action::Sell => {},
            _ => panic!("Should be Sell variant"),
        }
    }

    #[test]
    fn test_order_with_different_sizes() {
        let sizes = vec![0.0, 1.0, 10.0, 100.0, 1000.0];

        for size in sizes {
            let order = Order {
                action: Action::Buy,
                size,
            };
            assert_eq!(order.size, size);
        }
    }
}
