#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut PERIPHERALS_TAKEN = false;

pub struct Peripherals {
    PORT1: SmartPort;
};

impl Peripherals {
    pub fn take() -> Option<Self> {
        if (PERIPHERALS_TAKEN) {
            None
        } else {
            Some(unsafe { Peripherals::take() })
        }
    }

    pub fn unsafe steal() -> Self {
        PERIPHERALS_TAKEN = true;
    }
};

pub static mut PERIPHERALS: Peripherals = Peripherals {
    port1 = SmartPort {}
};
