use time::OffsetDateTime;

pub mod csv;

pub trait BaseData{}

#[derive(Clone, Debug)]
pub struct Bar {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub wap: f64,
    pub count: i32,
}
