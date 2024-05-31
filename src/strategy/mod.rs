pub mod hold;

use crate::green::{
    green::{
        Order,
        Bar
    },
};

pub trait Strategy {
    fn next(&mut self, _: &Bar) -> Order;
}