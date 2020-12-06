#![no_std]

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

mod bindings;
pub mod motor;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

pub struct SmartPort {
    port: u8,
}

impl SmartPort {
    pub fn as_motor(self) -> motor::Motor {
        motor::Motor { port: self.port }
    }
}

pub struct Peripherals {
    pub port1: SmartPort,
    pub port2: SmartPort,
    pub port3: SmartPort,
    pub port4: SmartPort,
    pub port5: SmartPort,
    pub port6: SmartPort,
    pub port7: SmartPort,
    pub port8: SmartPort,
    pub port9: SmartPort,
    pub port10: SmartPort,
    pub port11: SmartPort,
    pub port12: SmartPort,
    pub port13: SmartPort,
    pub port14: SmartPort,
    pub port15: SmartPort,
    pub port16: SmartPort,
    pub port17: SmartPort,
    pub port18: SmartPort,
    pub port19: SmartPort,
    pub port20: SmartPort,
}

static mut PERIPHERALS_TAKEN: bool = false;

impl Peripherals {
    pub fn take() -> Option<Self> {
        if unsafe { PERIPHERALS_TAKEN } {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    pub unsafe fn steal() -> Self {
        Peripherals {
            port1: SmartPort { port: 1 },
            port2: SmartPort { port: 2 },
            port3: SmartPort { port: 3 },
            port4: SmartPort { port: 4 },
            port5: SmartPort { port: 5 },
            port6: SmartPort { port: 6 },
            port7: SmartPort { port: 7 },
            port8: SmartPort { port: 8 },
            port9: SmartPort { port: 9 },
            port10: SmartPort { port: 10 },
            port11: SmartPort { port: 11 },
            port12: SmartPort { port: 12 },
            port13: SmartPort { port: 13 },
            port14: SmartPort { port: 14 },
            port15: SmartPort { port: 15 },
            port16: SmartPort { port: 16 },
            port17: SmartPort { port: 17 },
            port18: SmartPort { port: 18 },
            port19: SmartPort { port: 19 },
            port20: SmartPort { port: 20 },
        }
    }
}
