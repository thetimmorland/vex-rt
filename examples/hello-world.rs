#![no_std]
#![no_main]

extern crate vex_rt as rt;
use libc_print::std_name::*;

#[no_mangle]
extern "C" fn initialize() {
    println!("hello world")
}
