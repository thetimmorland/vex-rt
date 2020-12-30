#![no_std]
#![no_main]

extern crate vex_rt;

use core::time::Duration;
use libc_print::std_name::println;
use vex_rt::*;

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
            select! {
                _ = l.next() => {},
                _ = ctx.done() => break,
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
