pub struct SmartPort {
    port: u8,
}

impl SmartPort {
    pub unsafe fn new(port: u8) -> Self {
        assert!(
            (1..22).contains(&port),
            "Cannot construct a smart port on port {}",
            port
        );
        Self { port }
    }

    pub fn as_motor(self, gearset: crate::Gearset, is_reversed: bool) -> crate::Motor {
        unsafe { crate::Motor::new(self.port, gearset, is_reversed) }
    }
}
