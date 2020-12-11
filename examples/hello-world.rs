#![no_std]
#![no_main]

extern crate alloc;
extern crate vex_rt;

use alloc::string::*;
use libc_print::std_name::println;

struct Robot;

#[vex_rt::entry]
impl Robot {
    fn initialize() -> Self {
        let s = "Hello, world".to_string();
        println!("{}", s);
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
