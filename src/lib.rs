#![no_std]

extern crate alloc;
use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

mod bindings;

pub mod rtos;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
