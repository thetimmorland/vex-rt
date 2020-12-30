//! SmartPort.

use crate::motor::{Gearset, Motor};
/// A struct which represents an unconfigured smart port.
pub struct SmartPort {
    port: u8,
}

impl SmartPort {
    /// Unsafely constructs a new smart port
    pub unsafe fn new(port: u8) -> Self {
        assert!(
            (1..22).contains(&port),
            "Cannot construct a smart port on port {}",
            port
        );
        Self { port }
    }

    /// Converts a `SmartPort` into a [`Motor`](crate::motor::Motor).
    ///
    /// # Examples
    ///
    /// ```
    /// use vex_rt as rt;
    /// let peripherals = rt::Peripherals::take();
    /// let gearset = rt::Gearset::ThirtySixToOne;
    /// let is_reversed = false;
    /// let motor01 = peripherals.port01.as_motor(gearset, is_reversed);
    /// ```
    pub fn as_motor(self, gearset: Gearset, is_reversed: bool) -> Motor {
        unsafe { Motor::new(self.port, gearset, is_reversed) }
    }
}
