#![no_std]
#![no_main]

use libc_print::std_name::println;
use vex_rt::{entry, Context, Robot};

struct PanicBot;

impl Robot for PanicBot {
    fn initialize() -> Self {
        panic!("Bruh")
    }
    fn autonomous(&self, _: Context) {}
    fn opcontrol(&self, _: Context) {}
    fn disabled(&self, _: Context) {}
}

entry!(PanicBot);
