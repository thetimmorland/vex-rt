#![no_std]
#![no_main]

extern crate vex_rt as rt;

use rt::motor::Gearset::*;

#[no_mangle]
extern "C" fn opcontrol() {
    let peripherals = vex_rt::Peripherals::take().unwrap();
    peripherals
        .port1
        .as_motor(E_MOTOR_GEARSET_06, true)
        .move_velocity(32);
}
