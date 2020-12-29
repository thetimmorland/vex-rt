#![no_std]
#![no_main]

extern crate vex_rt;

use core::time::Duration;
use libc_print::std_name::println;
use vex_rt::{entry, Loop, Robot, Task};

struct TaskBot;

impl TaskBot {
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
                    l.delay()
                }
            },
        );
        TaskBot
    }
}

impl Robot for TaskBot {
    fn autonomous(&mut self) {
        println!("autonomous");
    }
    fn opcontrol(&mut self) {
        println!("opcontrol");
    }
    fn disable(&mut self) {
        println!("disabled");
    }
}

entry!(TaskBot::initialize(), TaskBot);
