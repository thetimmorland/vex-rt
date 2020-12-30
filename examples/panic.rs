#![no_std]
#![no_main]

use vex_rt::prelude::*;

struct PanicBot;

impl Robot for PanicBot {
    fn initialize() -> Self {
        panic!("Panic Message")
    }
    fn autonomous(&self, _: Context) {}
    fn opcontrol(&self, _: Context) {}
    fn disabled(&self, _: Context) {}
}

entry!(PanicBot);
