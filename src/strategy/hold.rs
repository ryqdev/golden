use crate::green::broker::backtest::BackTestBroker;
use crate::green::green::Green;
use crate::green::strategy::Strategy;

#[derive(Default, Clone, Debug)]
pub(crate) struct Order {
    pub(crate) symbol: String,
    pub(crate) size: f64,
}

#[derive(Default, Clone, Debug)]
pub struct SimpleStrategy {
    // broker: BackTestBroker,
    pub(crate) name: String,
    pub(crate) cash: Vec<f64>,
    pub position: Vec<f64>,
    pub(crate) net_assets: Vec<f64>,
    pub order: Vec<Order>
}

impl Strategy for SimpleStrategy {
    fn next(&mut self, data: &Vec<f64>) {
        let open_price = data[0];
        let close_price = data[3];
        if close_price > open_price {
            self.buy(1.0, close_price);
        } else {
            self.sell(1.0, close_price);
        }
    }

    fn buy(&mut self, size: f64, price: f64) {
        log::info!("buy");
        let cash = self.cash.last().unwrap();
        let position = self.position.last().unwrap();
        self.cash.push(cash - size * price);
        self.position.push(position + size);
        self.net_assets.push(self.cash.last().unwrap() + self.position.last().unwrap() * price);
        self.order.push(Order{
            symbol: self.name.to_owned(),
            size
        });
    }

    fn sell(&mut self, size: f64, price: f64) {
        log::info!("sell");
        let cash = self.cash.last().unwrap();
        let position = self.position.last().unwrap();
        self.cash.push(cash + size * price);
        self.position.push(position - size);
        self.net_assets.push(self.cash.last().unwrap() + self.position.last().unwrap() * price);
        self.order.push(Order{
            symbol: self.name.to_owned(),
            size
        });
    }

}