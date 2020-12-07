#![no_std]

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;
use libc_print::libc_println;

mod bindings;
mod motor;
mod peripherals;
mod smart_port;

pub use motor::*;
pub use peripherals::*;
pub use smart_port::*;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        libc_println!("panic occurred!: {:?}", s);
    } else {
        libc_println!("panic occurred!");
    }

    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
