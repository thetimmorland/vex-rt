#![no_std]
#![no_main]

use vex_rt::prelude::*;

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
