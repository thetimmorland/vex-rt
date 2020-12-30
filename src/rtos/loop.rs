use core::time::Duration;

use crate::{
    bindings,
    rtos::{GenericSleep, Selectable},
};

/// Provides a constant-period looping construct.
pub struct Loop {
    last_time: u32,
    delta: u32,
}

impl Loop {
    /// Creates a new loop object with a given period.
    pub fn new(delta: Duration) -> Self {
        Loop {
            last_time: unsafe { bindings::millis() },
            delta: delta.as_millis() as u32,
        }
    }

    /// Delays until the next loop cycle.
    pub fn delay(&mut self) {
        unsafe { bindings::task_delay_until(&mut self.last_time, self.delta) }
    }

    /// A [`Selectable`] event which occurs at the next loop cycle.
    pub fn next<'a>(&'a mut self) -> impl Selectable + 'a {
        struct LoopSelect<'a>(&'a mut Loop);

        impl<'a> Selectable for LoopSelect<'a> {
            fn poll(self) -> Result<(), Self> {
                if unsafe { bindings::millis() } >= self.0.last_time + self.0.delta {
                    self.0.last_time += self.0.delta;
                    Ok(())
                } else {
                    Err(self)
                }
            }
            fn sleep(&self) -> GenericSleep {
                GenericSleep::Timestamp(Duration::from_millis(
                    (self.0.last_time + self.0.delta).into(),
                ))
            }
        }

        LoopSelect(self)
    }
}
