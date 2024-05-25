use super::Broker;

#[derive(Default, Clone, Debug)]
pub struct BackTestBroker{
    pub(crate) cash: Vec<f64>,
    pub position: Vec<f64>,
    pub(crate) net_assets: Vec<f64>,
}

// impl Broker for BackTestBroker {
    // fn set_cash(&mut self, cash: f64) {
    //     self.cash = cash
    // }
    // fn connect(){}
// }