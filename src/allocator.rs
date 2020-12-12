use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};
use libc;

struct Alloc;

// This is heavily adapted from the libc_alloc code at
// https://github.com/daniel5151/libc_alloc/blob/aaf3c99494c1a938520c7d70668454e456e9a694/src/lib.rs#L31-L68
unsafe impl GlobalAlloc for Alloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::memalign(
            layout.align().max(core::mem::size_of::<usize>()),
            layout.size(),
        ) as *mut _
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut _);
    }
}

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

#[alloc_error_handler]
fn handle(layout: core::alloc::Layout) -> ! {
    panic!("memory allocation failed: {:#?}", layout);
}
