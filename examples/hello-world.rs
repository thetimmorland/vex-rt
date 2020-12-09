#![no_std]
#![no_main]

#[macro_use]
extern crate vex_rt;

use libc_print::std_name::*;

struct Robot {}

impl vex_rt::Robot for Robot {
    fn autonomous(&self) {}
    fn opcontrol(&self) {}
    fn disable(&self) {}
}

lazy_static! {
    static ref ROBOT: Robot = {
        let p = vex_rt::Peripherals::take();
        Robot {}
    };
}

#[no_mangle]
extern "C" fn initialize() {
    let robot = robot;
}
