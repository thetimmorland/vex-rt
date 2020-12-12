#![no_std]
#![no_main]

extern crate alloc;
extern crate vex_rt;

use libc_print::std_name::println;
use vex_rt::*;

struct Robot;

#[entry]
impl Robot {
    fn initialize() -> Self {
        println!("Hello, world");
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
