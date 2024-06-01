/// log::info!("RED: \x1b[41m  \x1b[0m");
/// log::info!("Yellow: \x1b[43m  \x1b[0m");
/// log::info!("GREEN: \x1b[42m  \x1b[0m");
#[derive(Debug)]
pub enum GoldenColor {}

impl GoldenColor {
    pub const RED: &'static str = "\x1b[41m";
    pub const GREEN: &'static str = "\x1b[42m";
    pub const YELLOW: &'static str = "\x1b[43m";
    pub const RESET: &'static str = "\x1b[0m";
}
