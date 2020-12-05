#![no_std]

use core::panic::PanicInfo;

mod bindings;

pub mod task;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
