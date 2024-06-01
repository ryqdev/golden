
#[derive(Default)]
pub(crate) struct BacktestBroker {
    pub cash: Vec<f64>,
    pub position: Vec<f64>,
    pub net_assets: Vec<f64>,
    pub order: Vec<Order>
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Order {
    pub action: Action,
    pub(crate) size: f64,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Action {
    #[default]
    None,
    Buy,
    Sell,
}
