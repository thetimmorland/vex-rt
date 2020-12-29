#![no_std]
#![no_main]

use core::time::Duration;
use libc_print::libc_println;
use vex_rt::{entry, Context, Robot, Task};

struct DelayBot;

impl Robot for DelayBot {
    fn initialize() -> Self {
        Self
    }
    fn autonomous(&self, _: Context) {}
    fn opcontrol(&self, _: Context) {
        let x: u32 = 0;
        loop {
            libc_println!("x = {}", x);
            Task::delay(Duration::from_secs(1));
        }
    }
    fn disabled(&self, _: Context) {}
}

entry!(DelayBot);
