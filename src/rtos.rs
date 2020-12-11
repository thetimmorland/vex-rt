use alloc::boxed::Box;
use alloc::string::String;
use core::ptr::null_mut;
use core::time::Duration;

use crate::bindings;
use crate::error::*;
use crate::util::*;

pub fn time_since_start() -> Duration {
    unsafe { Duration::from_millis(bindings::millis().into()) }
}

pub struct Task(bindings::task_t);

impl Task {
    pub fn delay(dur: Duration) {
        unsafe {
            bindings::task_delay(dur.as_millis() as u32);
        }
    }

    pub fn find_by_name(name: &str) -> Result<Task, Error> {
        as_cstring(name, |cname| {
            let ptr = unsafe { bindings::task_get_by_name(cname.into_raw()) };
            if ptr == null_mut() {
                todo!("Error")
            } else {
                Ok(Task(ptr))
            }
        })
    }

    pub fn spawn<F>(f: F) -> Result<Task, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        Task::spawn_ext(
            "",
            bindings::TASK_PRIORITY_DEFAULT,
            bindings::TASK_STACK_DEPTH_DEFAULT as u16,
            f,
        )
    }

    pub fn spawn_ext<F>(name: &str, priority: u32, stack_depth: u16, f: F) -> Result<Task, Error>
    where
        F: FnOnce() + Send + 'static,
    {
        extern "C" fn run<F: FnOnce()>(arg: *mut libc::c_void) {
            let cb_box: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            cb_box()
        }

        let cb = Box::new(f);
        unsafe {
            let arg = Box::into_raw(cb);
            let r = Task::spawn_raw(name, priority, stack_depth, run::<F>, arg as *mut _);
            if let Err(_) = r {
                // We need to re-box the pointer if the task could not be created, to avoid a
                // memory leak.
                Box::from_raw(arg);
            }
            r
        }
    }

    pub unsafe fn spawn_raw(
        name: &str,
        priority: u32,
        stack_depth: u16,
        f: unsafe extern "C" fn(arg1: *mut libc::c_void),
        arg: *mut libc::c_void,
    ) -> Result<Task, Error> {
        as_cstring(name, |cname| {
            let task = bindings::task_create(Some(f), arg, priority, stack_depth, cname.into_raw());
            if task != null_mut() {
                Ok(Task(task))
            } else {
                Err(from_errno())
            }
        })
    }

    pub fn name(&self) -> String {
        unsafe { from_cstring_raw(bindings::task_get_name(self.0)) }
    }
}

pub struct Loop {
    last_time: u32,
    delta: u32,
}

impl Loop {
    pub fn new(delta: Duration) -> Loop {
        Loop {
            last_time: unsafe { bindings::millis() },
            delta: delta.as_millis() as u32,
        }
    }

    pub fn delay(&mut self) {
        unsafe { bindings::task_delay_until(&mut self.last_time, self.delta) }
    }
}
