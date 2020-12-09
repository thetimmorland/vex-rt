#![no_std]
#![no_main]

#[macro_use]
extern crate vex_rt;

use libc_print::std_name::*;

static mut ROBOT: Option<Robot> = None;

#[derive(Debug)]
struct Robot {}

impl Robot {
    fn initialize(_periph: vex_rt::Peripherals) -> Robot {
        Robot {}
    }
    fn autonomous(&self) {}
    fn opcontrol(&self) {
        println!("{:#?}", self)
    }
    fn disable(&self) {}
}

#[no_mangle]
unsafe extern "C" fn initialize() {
    let peripherals = vex_rt::Peripherals::steal();
    ROBOT = Some(Robot::initialize(peripherals))
}

#[no_mangle]
unsafe extern "C" fn opcontrol() {
    ROBOT.as_mut().unwrap().opcontrol();
}

#[no_mangle]
unsafe extern "C" fn autonomous() {
    ROBOT.as_mut().unwrap().autonomous();
}
