/// A trait representing a competition-ready VEX Robot.
pub trait Robot {
    /// Runs at startup. This should be non-blocking, since the FreeRTOS
    /// scheduler doesn't start until it returns.
    fn initialize() -> Self;
    /// Runs during the autonomous period.
    fn autonomous(&mut self);
    /// Runs during the opcontrol period.
    fn opcontrol(&mut self);
    /// Runs when the robot is disabled.
    fn disabled(&mut self);
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
///     fn autonomous(&mut self) {}
///     fn opcontrol(&mut self) {}
///     fn disabled(&mut self) {}
/// }
///
/// entry!(FooBot);
/// ```
macro_rules! entry {
    ($robot_type:ty) => {
        static mut ROBOT: $crate::once::Once<$robot_type> = $crate::once::Once::new();

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT.call_once(|| $crate::Robot::initialize());
        }

        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            $crate::Task::spawn_ext(
                "opcontrol",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                || $crate::Robot::opcontrol(ROBOT.get_mut().unwrap()),
            );
        }

        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            $crate::Task::spawn_ext(
                "autonomous",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                || $crate::Robot::autonomous(ROBOT.get_mut().unwrap()),
            );
        }

        #[no_mangle]
        unsafe extern "C" fn disabled() {
            $crate::Task::spawn_ext(
                "disabled",
                $crate::Task::DEFAULT_PRIORITY,
                $crate::Task::DEFAULT_STACK_DEPTH,
                || $crate::Robot::disabled(ROBOT.get_mut().unwrap()),
            );
        }
    };
}
