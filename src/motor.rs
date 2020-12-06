use crate::bindings;

pub struct Motor {
    pub port: u8,
}

impl Motor {
    pub fn set_voltage(&self, voltage: i32) {
        unsafe { bindings::motor_move(self.port, voltage) };
    }
}
