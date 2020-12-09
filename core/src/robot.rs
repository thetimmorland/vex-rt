pub trait Robot {
    fn initialize(peripherals: crate::Peripherals) -> Self;
    fn autonomous(&self);
    fn opcontrol(&self);
    fn disable(&self);
}
