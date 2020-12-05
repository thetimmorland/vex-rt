#![no_std]
#![no_main]

extern crate vex_rt as rt;

use core::time::Duration;
use libc_print::libc_println;
use rt::rtos::Task;

#[no_mangle]
extern "C" fn opcontrol() {
    let x: u32 = 0;
    loop {
        libc_println!("x = {}", x);
        Task::delay(Duration::from_secs(1));
    }
}
