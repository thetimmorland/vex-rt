//! For use with the [`entry`] macro.

use crate::rtos::{Context, Mutex};

/// A trait representing a competition-ready VEX Robot.
pub trait Robot {
    /// Runs at startup. This should be non-blocking, since the FreeRTOS
    /// scheduler doesn't start until it returns.
    fn initialize() -> Self;
    /// Runs during the autonomous period.
    fn autonomous(&self, ctx: Context);
    /// Runs during the opcontrol period.
    fn opcontrol(&self, ctx: Context);
    /// Runs when the robot is disabled.
    fn disabled(&self, ctx: Context);
}

#[doc(hidden)]
pub struct ContextWrapper(Mutex<Option<Context>>);

impl ContextWrapper {
    #[doc(hidden)]
    #[inline]
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }

    #[doc(hidden)]
    pub fn replace(&self) -> Context {
        let mut opt = self.0.lock();
        if let Some(ctx) = opt.take() {
            ctx.cancel();
        }
        let ctx = Context::new_global();
        *opt = Some(ctx.clone());
        ctx
    }
}
