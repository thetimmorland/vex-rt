#![no_std]
#![no_main]

#[macro_use]
extern crate vex_rt;
use vex_rt::Robot;

use libc_print::std_name::*;

struct MyRobot;

#[vex_rt::entry]
impl vex_rt::Robot for MyRobot {
    fn initialize() -> Self {
        Self
    }
    fn autonomous(&self) {
        println!("autonomous");
    }
    fn opcontrol(&self) {
        println!("opcontrol");
    }
    fn disable(&self) {
        println!("disabled");
    }
}
