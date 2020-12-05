#![no_std]
#![no_main]

extern crate vex_rt as rt;
use libc_print::libc_println;

#[no_mangle]
extern "C" fn initialize() {
    libc_println!("hello, world");
}
