#![no_std]
#![no_main]

extern crate vex_rt;

use core::time::Duration;
use libc_print::std_name::println;
use vex_rt::*;

struct Robot;

#[entry]
impl Robot {
    fn initialize() -> Self {
        Robot
    }
    fn autonomous(&self) {
        println!("autonomous");
    }
    fn opcontrol(&self) {
        println!("opcontrol");
        let ctx = Context::new_global();
        Task::spawn({
            let mut x = 0;
            let mut l = Loop::new(Duration::from_secs(1));
            let ctx = ctx.clone();
            move || loop {
                println!("{}", x);
                x += 1;
                select! {
                    _ = l.next() => {},
                    _ = ctx.done() => break,
                }
            }
        });
        Task::delay(Duration::from_secs(10));
        ctx.cancel();
    }
    fn disable(&self) {
        println!("disabled");
    }
}
