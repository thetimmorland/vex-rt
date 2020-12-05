#![no_std]
#![no_main]

extern crate vex_rt as rt;
use core::time::Duration;
use libc_print::libc_println;

#[no_mangle]
extern "C" fn opcontrol() {
    let x: u32 = 0;
    loop {
        libc_println!("x = {}", x);
        rt::task::sleep(Duration::from_secs(1));
    }
}
