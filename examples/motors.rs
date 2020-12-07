#![no_std]
#![no_main]

extern crate vex_rt as rt;

use rt::Gearset::*;

#[no_mangle]
extern "C" fn opcontrol() {
    let peripherals = rt::Peripherals::take();
    peripherals
        .port1
        .as_motor(E_MOTOR_GEARSET_06, true)
        .move_velocity(32);
}
