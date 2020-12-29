#![no_std]
#![no_main]

use libc_print::std_name::println;
use vex_rt::{entry, Robot};

struct HelloBot;

impl Robot for HelloBot {
    fn initialize() -> Self {
        println!("initialize");
        HelloBot
    }
    fn autonomous(&mut self) {
        println!("autonomous");
    }
    fn opcontrol(&mut self) {
        println!("opcontrol");
    }
    fn disabled(&mut self) {
        println!("disabled");
    }
}

entry!(HelloBot);
