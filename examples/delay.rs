#![no_std]
#![no_main]

use core::time::Duration;
use libc_print::libc_println;
use vex_rt::{entry, Robot, Task};

struct DelayBot;

impl DelayBot {
    fn initialize() -> Self {
        DelayBot
    }
}

impl Robot for DelayBot {
    fn autonomous() {}
    fn opcontrol() {
        let x: u32 = 0;
        loop {
            libc_println!("x = {}", x);
            Task::delay(Duration::from_secs(1));
        }
    }
    fn disable() {}
}

entry!(DelayBot::initialize(), DelayBot);
