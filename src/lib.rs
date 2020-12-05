#![no_std]

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

mod bindings;

pub mod task;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
