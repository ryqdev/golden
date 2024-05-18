
pub trait Strategy {
    fn next(&self);
    fn buy(&self);
    fn sell(&self);
}