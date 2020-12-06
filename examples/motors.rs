#![no_std]
#![no_main]

extern crate vex_rt as rt;

#[no_mangle]
extern "C" fn initialize() {
    let peripherals = vex_rt::Peripherals::take().unwrap();
    peripherals.port1.as_motor().set_voltage(32);
}
