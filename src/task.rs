use core::convert::TryInto;
use core::time::Duration;

use crate::bindings::rtos;

pub fn sleep(dur: Duration) {
    unsafe {
        rtos::delay(dur.as_millis().try_into().unwrap());
    }
}
