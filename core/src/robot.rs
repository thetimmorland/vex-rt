pub trait Robot {
    fn initialize() -> Self;
    fn autonomous(&self);
    fn opcontrol(&self);
    fn disable(&self);
}
