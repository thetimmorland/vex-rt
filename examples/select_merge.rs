#![no_std]
#![no_main]

use core::time::Duration;

use vex_rt::prelude::*;
use vex_rt::rtos::Loop;
use vex_rt::select;
use vex_rt::select_merge;

struct SelectRobot;

impl Robot for SelectRobot {
    fn initialize() -> Self {
        Self
    }
    fn autonomous(&self, ctx: Context) {
        println!("autonomous");
        let mut x = 0;
        let mut l = Loop::new(Duration::from_secs(1));
        loop {
            println!("{}", x);
            x += 1;
            let event = select_merge! {
                _ = l.next() => false,
                _ = ctx.done() => true,
            };
            if select! { b = event => b } {
                break;
            }
        }
        println!("auto done")
    }
    fn opcontrol(&self, _: Context) {
        println!("opcontrol");
    }
    fn disabled(&self, _: Context) {
        println!("disabled");
    }
}

entry!(SelectRobot);
