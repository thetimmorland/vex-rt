#![no_std]
#![no_main]

use libc_print::std_name::println;
use vex_rt::{entry, Context, Robot};

struct HelloBot;

impl Robot for HelloBot {
    fn initialize() -> Self {
        println!("initialize");
        HelloBot
    }
    fn autonomous(&self, _: Context) {
        println!("autonomous");
    }
    fn opcontrol(&self, _: Context) {
        println!("opcontrol");
    }
    fn disabled(&self, _: Context) {
        println!("disabled");
    }
}

entry!(HelloBot);
