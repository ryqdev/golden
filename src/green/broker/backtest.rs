use super::Broker;

pub struct BackTestBroker{
    pub(crate) cash: f64,
    pub(crate) net_assets: f64,
}

impl Broker for BackTestBroker {
    fn set_cash(&mut self, cash: f64) {
        self.cash = cash
    }

}