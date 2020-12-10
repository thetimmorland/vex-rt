#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;
use libc_print::libc_println;

mod bindings;
mod motor;
mod peripherals;
mod robot;
mod rtos;
mod smart_port;

pub use motor::*;
pub use peripherals::*;
pub use robot::*;
pub use rtos::*;
pub use smart_port::*;

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

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[alloc_error_handler]
fn handle(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation failed: {:#?}", layout);
}
