pub struct SmartPort {
    port: u8,
}

impl SmartPort {
    /// Unsafely constructs a new smartport
    pub unsafe fn new(port: u8) -> Self {
        assert!(
            (1..22).contains(&port),
            "Cannot construct a smart port on port {}",
            port
        );
        Self { port }
    }

    /// Converts a `SmartPort` into a [`Motor`](crate::Motor).
    ///
    /// # Examples
    ///
    /// ```
    /// use vex_rt as rt;
    /// let peripherals = rt::Peripherals::take();
    /// let gearset = rt::Gearset::E_MOTOR_GEARSET_06;
    /// let is_reversed = false;
    /// let motor01 = peripherals.port01.as_motor(gearset, is_reversed);
    /// ```
    pub fn as_motor(self, gearset: crate::Gearset, is_reversed: bool) -> crate::Motor {
        unsafe { crate::Motor::new(self.port, gearset, is_reversed) }
    }
}
