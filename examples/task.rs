#![no_std]
#![no_main]

extern crate vex_rt;

use core::time::Duration;
use libc_print::std_name::println;

struct Robot;

#[vex_rt::entry]
impl Robot {
    fn initialize() -> Self {
        vex_rt::Task::spawn(|| {
            let mut x = 0;
            while true {
                println!("{}", x);
                x += 1;
                vex_rt::Task::delay(Duration::from_secs(1));
            }
        });
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
