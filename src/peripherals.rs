/// A struct which represents all the peripherals on the V5 brain.

pub struct Peripherals {
    /// Smart Port 1
    pub port01: crate::SmartPort,
    /// Smart Port 2
    pub port02: crate::SmartPort,
    /// Smart Port 3
    pub port03: crate::SmartPort,
    /// Smart Port 4
    pub port04: crate::SmartPort,
    /// Smart Port 5
    pub port05: crate::SmartPort,
    /// Smart Port 6
    pub port06: crate::SmartPort,
    /// Smart Port 7
    pub port07: crate::SmartPort,
    /// Smart Port 8
    pub port08: crate::SmartPort,
    /// Smart Port 9
    pub port09: crate::SmartPort,
    /// Smart Port 10
    pub port10: crate::SmartPort,
    /// Smart Port 11
    pub port11: crate::SmartPort,
    /// Smart Port 12
    pub port12: crate::SmartPort,
    /// Smart Port 13
    pub port13: crate::SmartPort,
    /// Smart Port 14
    pub port14: crate::SmartPort,
    /// Smart Port 15
    pub port15: crate::SmartPort,
    /// Smart Port 16
    pub port16: crate::SmartPort,
    /// Smart Port 17
    pub port17: crate::SmartPort,
    /// Smart Port 18
    pub port18: crate::SmartPort,
    /// Smart Port 19
    pub port19: crate::SmartPort,
    /// Smart Port 20
    pub port20: crate::SmartPort,
    /// Smart Port 21
    pub port21: crate::SmartPort,
}

static mut PERIPHERALS_TAKEN: bool = false;

impl Peripherals {
    /// Constructs a [`Peripherals`] struct once.
    ///
    /// **Warning:** Panics if called multiple times.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex_rt as rt;
    /// let peripherals = rt::Peripherals::take();
    /// ```
    pub fn take() -> Option<Self> {
        if unsafe { PERIPHERALS_TAKEN } {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Constructs a [`Peripherals`] struct unsafely.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex_rt as rt;
    /// let peripherals = unsafe { rt::Peripherals::steal() };
    /// ```
    pub unsafe fn steal() -> Self {
        PERIPHERALS_TAKEN = true;

        Peripherals {
            port01: crate::SmartPort::new(1),
            port02: crate::SmartPort::new(2),
            port03: crate::SmartPort::new(3),
            port04: crate::SmartPort::new(4),
            port05: crate::SmartPort::new(5),
            port06: crate::SmartPort::new(6),
            port07: crate::SmartPort::new(7),
            port08: crate::SmartPort::new(8),
            port09: crate::SmartPort::new(9),
            port10: crate::SmartPort::new(10),
            port11: crate::SmartPort::new(11),
            port12: crate::SmartPort::new(12),
            port13: crate::SmartPort::new(13),
            port14: crate::SmartPort::new(14),
            port15: crate::SmartPort::new(15),
            port16: crate::SmartPort::new(16),
            port17: crate::SmartPort::new(17),
            port18: crate::SmartPort::new(18),
            port19: crate::SmartPort::new(19),
            port20: crate::SmartPort::new(20),
            port21: crate::SmartPort::new(21),
        }
    }
}
