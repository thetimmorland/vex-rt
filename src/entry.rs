use crate::{Context, Mutex};

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

#[macro_export]
/// Specifies the entrypoint for the robot.
///
/// # Examples
///
/// ```
/// #![no_std]
/// #![no_main]
///
/// use vex_rt::{entry, Robot};
///
/// struct FooBot;
///
/// impl FooBot {
///     fn initialize() -> Self {
///         FooBot
///     }
/// }
///
/// impl Robot for FooBot {
///     fn autonomous(&self, ctx: Context) {}
///     fn opcontrol(&self, ctx: Context) {}
///     fn disabled(&self, ctx: Context) {}
/// }
///
/// entry!(FooBot);
/// ```
macro_rules! entry {
    ($robot_type:ty) => {
        static ROBOT: $crate::once::Once<($robot_type, $crate::ContextWrapper)> =
            $crate::once::Once::new();

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT.call_once(|| ($crate::Robot::initialize(), $crate::ContextWrapper::new()));
        }

        #[no_mangle]
        extern "C" fn opcontrol() {
            let (robot, wrapper) = ROBOT.get().unwrap();
            $crate::Task::spawn_ext(
                "opcontrol",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                move || $crate::Robot::opcontrol(robot, wrapper.replace()),
            );
        }

        #[no_mangle]
        extern "C" fn autonomous() {
            let (robot, wrapper) = ROBOT.get().unwrap();
            $crate::Task::spawn_ext(
                "autonomous",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                move || $crate::Robot::autonomous(robot, wrapper.replace()),
            );
        }

        #[no_mangle]
        extern "C" fn disabled() {
            let (robot, wrapper) = ROBOT.get().unwrap();
            $crate::Task::spawn_ext(
                "disabled",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                move || $crate::Robot::disabled(robot, wrapper.replace()),
            );
        }
    };
}
