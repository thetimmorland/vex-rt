#![no_std]
#![no_main]

extern crate alloc;
extern crate vex_rt;

use libc_print::std_name::println;
use vex_rt::entry;
use vex_rt::Robot;

struct HelloBot;

impl HelloBot {
    fn initialize() -> Self {
        println!("initialize");
        HelloBot
    }
}

impl Robot for HelloBot {
    fn autonomous(&mut self) {
        println!("autonomous");
    }
    fn opcontrol(&mut self) {
        println!("opcontrol");
    }
    fn disable(&mut self) {
        println!("disabled");
    }
}

entry!(HelloBot::initialize(), HelloBot);
