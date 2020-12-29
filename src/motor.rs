use crate::bindings;

/// An enum which represents possible gear cartridges for a motor.
pub enum Gearset {
    /// Blue 6:1 Gearset (600RPM)
    SixToOne,
    /// Green 18:1 Gearset (200RPM)
    EighteenToOne,
    /// Red 36:1 Gearset (100RPM)
    ThirtySixToOne,
}

impl Gearset {
    fn to_motor_gearset_e_t(&self) -> bindings::motor_gearset_e_t {
        match self {
            Gearset::SixToOne => bindings::motor_gearset_e::E_MOTOR_GEARSET_06,
            Gearset::EighteenToOne => bindings::motor_gearset_e::E_MOTOR_GEARSET_18,
            Gearset::ThirtySixToOne => bindings::motor_gearset_e::E_MOTOR_GEARSET_36,
        }
    }
}

/// A struct which represents a V5 smart port configured as a motor.
pub struct Motor {
    port: u8,
}

impl Motor {
    /// constructs a new motor unsafely. You probably want to use
    /// [`crate::Peripherals::take()`] instead.
    pub unsafe fn new(port: u8, gearset: Gearset, reverse: bool) -> Motor {
        assert!((1..22).contains(&port));
        bindings::motor_set_gearing(port, gearset.to_motor_gearset_e_t());
        bindings::motor_set_reversed(port, reverse);
        Motor { port }
    }

    /// Sets the voltage for the motor on the range -127 to 127. Useful when
    /// mapping controller output to motor control.
    pub fn move_i8(&self, voltage: i8) {
        unsafe { bindings::motor_move(self.port, voltage as i32) };
    }

    /// Sets the target position for the motor, relative to either the motor's
    /// position when it was initialized or the motor's position during the most
    /// recent call to `Motor::tare_position(&self)`.
    pub fn move_absolute(&self, position: f64, velocity: i32) {
        unsafe { bindings::motor_move_absolute(self.port, position, velocity) };
    }

    /// Sets target position for motor, relative to it's current position.
    pub fn move_relative(&self, position: f64, velocity: i32) {
        unsafe { bindings::motor_move_relative(self.port, position, velocity) };
    }

    /// Sets the velocity for the motor.
    ///
    /// This velocity corresponds to different actual speeds depending on the
    /// gearset used for the motor. This results in a range of +-100 for
    /// E_MOTOR_GEARSET_36, +-200 for E_MOTOR_GEARSET_18, and +-600 for blue.
    /// The velocity is held with PID to ensure consistent speed, as opposed to
    /// setting the motor’s voltage.
    pub fn move_velocity(&self, velocity: i32) {
        unsafe { bindings::motor_move_velocity(self.port, velocity) };
    }

    /// Sets the voltage for the motor from -12000 mV to 12000 mV.
    pub fn move_voltage(&self, voltage: i32) {
        unsafe { bindings::motor_move_voltage(self.port, voltage) };
    }

    /// Changes the output velocity for a profiled movement (motor_move_absolute
    /// or motor_move_relative). This will have no effect if the motor is not
    /// following a profiled movement.
    pub fn modify_profiled_velocity(&self, velocity: i32) {
        unsafe { bindings::motor_modify_profiled_velocity(self.port, velocity) };
    }

    /// Gets the target position set for the motor by the user.
    pub fn get_target_position(&self) -> f64 {
        unsafe { bindings::motor_get_target_position(self.port) }
    }

    /// Gets the target velocity set for the motor by the user.
    pub fn get_target_velocity(&self) -> i32 {
        unsafe { bindings::motor_get_target_velocity(self.port) }
    }
}
