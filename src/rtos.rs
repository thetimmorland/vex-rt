use alloc::format;
use core::convert::TryInto;
use core::option::Option;
use core::ptr::null_mut;
use core::time::Duration;
use rcstring::*;

use crate::bindings::rtos;

pub fn time_since_start() -> Duration {
    unsafe {
        Duration::from_millis(rtos::millis().into())
    }
}

pub struct Task(rtos::Task);

impl Task {
    pub fn delay(dur: Duration) {
        unsafe {
            rtos::task_delay(dur.as_millis().try_into().unwrap());
        }
    }

    pub fn find_by_name(name: &str) -> Option<Task> {
        match CString::new(&format!("{}\0", name)) {
            Ok(cname) => {
                let ptr = unsafe {
                    rtos::task_get_by_name(cname.into_raw())
                };
                if ptr == null_mut() {
                    None
                } else {
                    Some(Task(ptr))
                }
            },
            Err(_) => None,
        }
    }
}

pub struct Loop {
    last_time: u32,
    delta: u32,
}

impl Loop {
    pub fn new(delta: Duration) -> Loop {
        unsafe {
            Loop {
                last_time: rtos::millis(),
                delta: delta.as_millis().try_into().unwrap(),
            }
        }
    }

    pub fn delay(&mut self) {
        unsafe {
            rtos::task_delay_until(&mut self.last_time, self.delta)
        }
    }
}
