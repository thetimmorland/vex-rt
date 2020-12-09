/// A struct which represents all the peripherals on the V5 brain.

pub struct Peripherals {
    pub port01: crate::SmartPort,
    pub port02: crate::SmartPort,
    pub port03: crate::SmartPort,
    pub port04: crate::SmartPort,
    pub port05: crate::SmartPort,
    pub port06: crate::SmartPort,
    pub port07: crate::SmartPort,
    pub port08: crate::SmartPort,
    pub port09: crate::SmartPort,
    pub port10: crate::SmartPort,
    pub port11: crate::SmartPort,
    pub port12: crate::SmartPort,
    pub port13: crate::SmartPort,
    pub port14: crate::SmartPort,
    pub port15: crate::SmartPort,
    pub port16: crate::SmartPort,
    pub port17: crate::SmartPort,
    pub port18: crate::SmartPort,
    pub port19: crate::SmartPort,
    pub port20: crate::SmartPort,
    pub port21: crate::SmartPort,
}

static mut PERIPHERALS_TAKEN: bool = false;

impl Peripherals {
    pub fn take() -> Option<Self> {
        //! Constructs a [`Peripherals`] struct once.
        //!
        //! **Warning:** Panics if called multiple times.
        //!
        //! # Examples
        //!
        //! ```
        //! use vex_rt as rt;
        //! let peripherals = rt::Peripherals::take();
        //! ```

        if unsafe { PERIPHERALS_TAKEN } {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    pub unsafe fn steal() -> Self {
        //! Constructs a [`Peripherals`] struct unsafely.
        //!
        //! # Examples
        //!
        //! ```
        //! use vex_rt as rt;
        //! let peripherals = unsafe { rt::Peripherals::steal() };
        //! ```

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
