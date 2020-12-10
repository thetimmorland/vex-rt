#![no_std]
#![no_main]

extern crate vex_rt;

use libc_print::std_name::println;

struct Robot;

#[vex_rt::entry]
impl Robot {
    fn initialize() -> Self {
        Robot
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
