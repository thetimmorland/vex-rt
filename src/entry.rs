/// A trait representing a competition-ready VEX Robot.
pub trait Robot {
    /// Runs during the autonomous period.
    fn autonomous(&mut self);
    /// Runs during the opcontrol period.
    fn opcontrol(&mut self);
    /// Runs when the robot is disabled.
    fn disable(&mut self);
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
/// impl Robot for HelloBot {
///     fn autonomous(&mut self) {}
///     fn opcontrol(&mut self) {}
///     fn disable(&mut self) {}
/// }
///
/// entry!(HelloBot::initialize(), HelloBot);
/// ```
macro_rules! entry {
    ($robot_expr:expr, $robot_type:ty) => {
        static mut ROBOT: $crate::once::Once<$robot_type> = $crate::once::Once::new();

        #[no_mangle]
        unsafe extern "C" fn initialize() {
            ROBOT.call_once(|| $robot_expr);
        }

        #[no_mangle]
        unsafe extern "C" fn opcontrol() {
            $crate::Robot::opcontrol(ROBOT.get_mut().unwrap());
        }

        #[no_mangle]
        unsafe extern "C" fn autonomous() {
            $crate::Robot::autonomous(ROBOT.get_mut().unwrap());
        }

        #[no_mangle]
        unsafe extern "C" fn disabled() {
            $crate::Robot::disable(ROBOT.get_mut().unwrap());
        }
    };
}
