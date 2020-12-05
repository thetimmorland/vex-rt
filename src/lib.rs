#![no_std]

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

extern "C" {
    fn motor_move(port: u8, voltage: i8) -> i32;
}

pub struct SmartPort (u8);

impl SmartPort {
    pub fn as_motor(&self) -> Motor {
        return Motor(self.0)
    }
}
pub struct Motor (u8);

impl Motor {
    pub fn set_voltage(&self, voltage: i8) {
        unsafe {
            motor_move(self.0, voltage);
        };
    }
}

pub struct Peripherals {
    pub port1: SmartPort,
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
            port1: SmartPort(1),
        }
    }
}
