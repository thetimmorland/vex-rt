#![no_std]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![warn(missing_docs)]

extern crate alloc;

use core::panic::PanicInfo;
use libc_print::libc_println;

mod allocator;
mod bindings;
mod entry;
mod error;
mod motor;
mod peripherals;
mod rtos;
mod smart_port;
mod util;

pub use entry::*;
pub use error::*;
pub use motor::*;
pub use peripherals::*;
pub use rtos::*;
pub use smart_port::*;

pub use spin::*;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        libc_println!("panic occurred!: {:?}", s);
    } else {
        libc_println!("panic occurred!");
    }

    unsafe {
        libc::exit(1);
    }
}
