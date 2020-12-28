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
        let mut x = 0;
        let mut l = Loop::new(Duration::from_secs(1));
        Task::spawn_ext(
            "test",
            Task::DEFAULT_PRIORITY,
            Task::DEFAULT_STACK_DEPTH,
            move || {
                println!("Task name: {}", Task::current().name());
                loop {
                    println!("{}", x);
                    x += 1;
                    select! {
                        _ = l.next() => ()
                    }
                }
            },
        );
        Robot
    }
    fn autonomous(&self) {
        println!("autonomous");
    }
    fn opcontrol(&self) {
        println!("opcontrol");
    }
    fn disable(&self) {
        println!("disabled");
    }
}
