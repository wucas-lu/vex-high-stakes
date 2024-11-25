use core::f64::consts::PI;

use anyhow::Result;
use libm::*;
use log::debug;
use vexide::{devices::controller::JoystickState, prelude::*};

const PI_OVER_FOUR: f64 = PI / 4.0;
const MAX_MOTOR_SPEED: f64 = 127.0;
const DEADBAND: f64 = 20.0;

use crate::set_velocities;

/**
    Holonomic X drivetrain
*/
pub struct Holonomic {
    front_left_motor: Motor,
    front_right_motor: Motor,
    rear_left_motor: Motor,
    rear_right_motor: Motor,
}

pub enum Heading {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl From<Heading> for f64 {
    fn from(heading: Heading) -> Self {
        match heading {
            Heading::North => 0.0_f64,
            Heading::Northeast => PI_OVER_FOUR,
            Heading::East => PI / 2.0,
            Heading::Southeast => 3.0 * PI / 4.0,
            Heading::South => PI,
            Heading::Southwest => 5.0 * PI / 4.0,
            Heading::West => 3.0 * PI / 2.0,
            Heading::Northwest => 7.0 * PI / 4.0,
        }
    }
}

impl Holonomic {
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
        }
    }

    // https://github.com/edjubuh/HolonomicXDrive-PROS/blob/master/Holonomic/include/HolonomicRadians.h
    // FIXME: this will make it spin so fucking fast lol
    pub fn update(&mut self, radians: Option<f64>, speed: Option<f64>, rotation_speed: Option<f64>) -> Result<()> {
        let radians = radians.unwrap_or(0.0);
        let speed = speed.unwrap_or(1.0).clamp(0.0, 1.0);
        let rotation_speed = rotation_speed.unwrap_or(0.0).clamp(-MAX_MOTOR_SPEED, MAX_MOTOR_SPEED);

        if speed <= 0.0 {
            if rotation_speed > DEADBAND || rotation_speed < -DEADBAND {
                set_velocities!(
                    rotation_speed as i32,
                    self.front_left_motor,
                    self.front_right_motor,
                    self.rear_left_motor,
                    self.rear_right_motor
                );

                return Ok(());
            }

            set_velocities!(
                0,
                self.front_left_motor,
                self.front_right_motor,
                self.rear_left_motor,
                self.rear_right_motor
            );

            return Ok(());
        }

        let fl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;
        let fr = MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
        let rl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
        let rr = MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;

        let max_value = [fl, fr, rl, rr].into_iter().reduce(f64::max).unwrap();
        let speed = speed * (MAX_MOTOR_SPEED / max_value);

        debug!("FL {} FR {} RL {} RR {} SPEED {}", fl, fr, rl, rr, speed);

        self.front_left_motor.set_velocity((fl * speed) as i32)?;
        self.front_right_motor.set_velocity((fr * speed) as i32)?;
        self.rear_left_motor.set_velocity((rl * speed) as i32)?;
        self.rear_right_motor.set_velocity((rr * speed) as i32)?;

        Ok(())
    }
}

pub fn radians_from_controller_joystick(joystick: JoystickState) -> f64 {
    atan2(joystick.x(), joystick.y())
}

pub fn speed_from_controller_joystick(joystick: JoystickState) -> f64 {
    debug!(
        "SPEED FROM CONTROLLER JOYSTICK IS {}",
        (sqrt(joystick.x().powf(2.0) + joystick.y().powf(2.0)) / MAX_MOTOR_SPEED).min(1.0)
    );
    (sqrt(joystick.x().powf(2.0) + joystick.y().powf(2.0)) / MAX_MOTOR_SPEED).min(1.0)
}
