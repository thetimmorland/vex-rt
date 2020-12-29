pub trait Robot {
    fn autonomous(&mut self);
    fn opcontrol(&mut self);
    fn disable(&mut self);
}

#[macro_export]
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
