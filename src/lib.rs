//! A crate for running rust on the VEX V5.

#![no_std]
#![feature(alloc_error_handler)]
#![feature(negative_impls)]
#![warn(missing_docs)]

extern crate alloc;

use core::panic::PanicInfo;
use libc_print::libc_eprintln;

mod allocator;
mod bindings;
mod context;
mod entry;
mod error;
mod motor;
mod peripherals;
mod rtos;
mod smart_port;
mod util;

pub use context::*;
pub use entry::*;
pub use error::*;
pub use motor::*;
pub use peripherals::*;
pub use rtos::*;
pub use smart_port::*;

#[doc(hidden)]
pub use spin::once;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    libc_eprintln!("panic occurred!: {:?}", panic_info);

    unsafe {
        libc::exit(1);
    }
}
