
#[derive(Default)]
pub(crate) struct BacktestBroker {
    pub cash: Vec<f64>,
    pub position: Vec<f64>,
    pub net_assets: Vec<f64>,
    pub order: Vec<Order>
}

#[derive(Default, Clone, Debug)]
pub struct Order {
    pub action: Action,
    pub size: f64,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Action {
    #[default]
    None,
    Buy,
    Sell,
}
