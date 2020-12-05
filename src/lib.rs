#![no_std]

use core::panic::PanicInfo;
use libc_alloc::LibcAlloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;
