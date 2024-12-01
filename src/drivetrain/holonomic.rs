//! Implements a holonomic x-drive drivetrain.

// Original implementation by edjubuh
// http://blog.elliotjb.com/
// https://github.com/edjubuh/HolonomicXDrive-PROS

use core::f64::consts::PI;

use libm::{atan2, sqrt};
use vexide::{devices::controller::JoystickState, prelude::*};

use super::{BrakeMode, *};

pub const PI_OVER_FOUR: f64 = PI / 4.0;
pub const MAX_MOTOR_SPEED: f64 = 127.0;
pub const DEADBAND: f64 = 20.0;

/**
    Radian heading values that can be used with holonomic drivetrains.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolonomicHeading {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl From<HolonomicHeading> for f64 {
    fn from(heading: HolonomicHeading) -> Self {
        match heading {
            HolonomicHeading::North => 0.0_f64,
            HolonomicHeading::Northeast => PI_OVER_FOUR,
            HolonomicHeading::East => PI / 2.0_f64,
            HolonomicHeading::Southeast => 3.0_f64 * PI / 4.0_f64,
            HolonomicHeading::South => PI,
            HolonomicHeading::Southwest => 5.0_f64 * PI / 4.0_f64,
            HolonomicHeading::West => 3.0_f64 * PI / 2.0_f64,
            HolonomicHeading::Northwest => 7.0_f64 * PI / 4.0_f64,
        }
    }
}

pub struct HolonomicRadians {
    pub front_left: f64,
    pub front_right: f64,
    pub rear_left: f64,
    pub rear_right: f64,
}

impl HolonomicRadians {
    fn new(front_left: f64, front_right: f64, rear_left: f64, rear_right: f64) -> Self {
        Self {
            front_left,
            front_right,
            rear_left,
            rear_right,
        }
    }
}

impl From<(f64, f64, f64, f64)> for HolonomicRadians {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self {
            front_left: value.0,
            front_right: value.1,
            rear_left: value.2,
            rear_right: value.3,
        }
    }
}

pub fn calculate_holonomic_radians(
    radians: Option<f64>,
    speed: Option<f64>,
    rotation_speed: Option<f64>,
) -> HolonomicRadians {
    let radians = radians.unwrap_or(0.0);
    let speed = speed.unwrap_or(1.0).clamp(0.0, 1.0);
    let rotation_speed = rotation_speed.unwrap_or(0.0).clamp(-MAX_MOTOR_SPEED, MAX_MOTOR_SPEED);

    if speed <= 0.0 {
        if rotation_speed > DEADBAND || rotation_speed < -DEADBAND {
            return HolonomicRadians::new(rotation_speed, rotation_speed, rotation_speed, rotation_speed);
        }

        return HolonomicRadians::new(0.0, 0.0, 0.0, 0.0);
    }

    let fl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;
    let fr = MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
    let rl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
    let rr = MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;

    let max_value = [fl, fr, rl, rr].into_iter().reduce(f64::max).unwrap();
    let speed = speed * (MAX_MOTOR_SPEED / max_value);

    HolonomicRadians::new(fl * speed, fr * speed, rl * speed, rr * speed)
}

pub fn radians_from_controller_joystick(joystick: JoystickState) -> f64 {
    atan2(joystick.y(), joystick.x())
}

pub fn speed_from_controller_joystick(joystick: JoystickState) -> f64 {
    (sqrt(joystick.x().powf(2.0_f64) + joystick.y().powf(2.0_f64)) / MAX_MOTOR_SPEED)
        .min(1.0_f64)
        .max(0.0_f64)
}

#[derive(Debug)]
pub struct HolonomicDrivetrain {
    front_left_motor: Motor,
    front_right_motor: Motor,
    rear_left_motor: Motor,
    rear_right_motor: Motor,

    velocity: f64,
}

impl HolonomicDrivetrain {
    pub fn new(
        front_left_motor: Motor,
        front_right_motor: Motor,
        rear_left_motor: Motor,
        rear_right_motor: Motor,
    ) -> Self {
        Self {
            front_left_motor,
            front_right_motor,
            rear_left_motor,
            rear_right_motor,

            velocity: 0.0,
        }
    }

    pub fn set(&mut self, radians: Option<f64>, speed: Option<f64>, rotation_speed: Option<f64>) -> Result<()> {
        let radians = calculate_holonomic_radians(radians, speed, rotation_speed);

        self.front_left_motor.set_velocity(radians.front_left as i32)?;
        self.front_right_motor.set_velocity(radians.front_right as i32)?;
        self.rear_left_motor.set_velocity(radians.rear_left as i32)?;
        self.rear_right_motor.set_velocity(radians.rear_right as i32)?;

        Ok(())
    }
}

#[expect(unused)]
impl Drivetrain for HolonomicDrivetrain {
    async fn drive(&mut self, direction: DriveDirection) -> Result<()> {
        self.set(
            Some(
                match direction {
                    DriveDirection::Forward => HolonomicHeading::North,
                    DriveDirection::Reverse => HolonomicHeading::South,
                    DriveDirection::Left => HolonomicHeading::West,
                    DriveDirection::Right => HolonomicHeading::East,
                }
                .into(),
            ),
            Some(self.velocity),
            None,
        )
    }

    async fn drive_for(
        &mut self,
        distance: f64,
        direction: Option<DriveDirection>,
        unit: Option<DistanceUnit>,
    ) -> Result<()> {
        todo!()
    }

    async fn turn(&mut self, direction: TurnDirection) -> Result<()> {
        todo!()
    }

    async fn turn_for(&mut self, angle: f64, unit: Option<RotationUnit>) -> Result<()> {
        todo!()
    }

    fn stop(&mut self, brake_mode: Option<BrakeMode>) -> Result<()> {
        todo!()
    }

    fn is_spinning(&self) -> Result<bool> {
        todo!()
    }

    fn is_done(&self) -> Result<bool> {
        todo!()
    }

    fn velocity(&self, unit: Option<VelocityUnit>) -> Result<f64> {
        todo!()
    }

    fn torque(&self, unit: Option<TorqueUnit>) -> Result<f64> {
        todo!()
    }

    fn current(&self, unit: Option<CurrentUnit>) -> Result<f64> {
        todo!()
    }

    fn efficiency_percent(&self) -> Result<f64> {
        todo!()
    }

    fn power_watts(&self) -> Result<f64> {
        todo!()
    }
}
