//! Holonomic X drive

use vexide::{
    devices::controller::{ControllerState, JoystickState},
    prelude::*,
};

use crate::prelude::{Angle, *};

pub const MAX_MOTOR_SPEED: f64 = 127.0_f64;
pub const DEADBAND: f64 = 20.0_f64;

pub fn speed_from_single_controller_joystick(joystick: &JoystickState) -> f64 {
    ((joystick.x().powf(2.0_f64) + joystick.y().powf(2.0_f64)).sqrt() / MAX_MOTOR_SPEED)
        .min(1.0_f64)
        .max(0.0_f64)
}

pub fn speed_from_controller_joysticks(controller: &ControllerState) -> f64 {
    (speed_from_single_controller_joystick(&controller.left_stick)
        + speed_from_single_controller_joystick(&controller.right_stick))
        / 2.0_f64
}

pub fn holonomic_motor_velocities(
    heading: Angle,
    speed: Option<f64>,
    rotation_speed: Option<f64>,
) -> (f64, f64, f64, f64) {
    let speed = speed.unwrap_or(1.0_f64).clamp(0.0_f64, 1.0_f64);
    let rotation_speed = rotation_speed
        .unwrap_or(0.0_f64)
        .clamp(-MAX_MOTOR_SPEED, MAX_MOTOR_SPEED);

    if speed <= 0.0 {
        if rotation_speed > DEADBAND || rotation_speed < -DEADBAND {
            return (rotation_speed, rotation_speed, rotation_speed, rotation_speed);
        }

        return (0.0, 0.0, 0.0, 0.0);
    }

    let heading_radian = heading.as_radians();
    let fl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR - heading_radian).cos() + rotation_speed;
    let fr = MAX_MOTOR_SPEED * (PI_OVER_FOUR + heading_radian).cos() + rotation_speed;
    let rl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR + heading_radian).cos() + rotation_speed;
    let rr = MAX_MOTOR_SPEED * (PI_OVER_FOUR - heading_radian).cos() + rotation_speed;

    let max_value = [fl, fr, rl, rr].into_iter().reduce(f64::max).unwrap();
    let speed = speed * (MAX_MOTOR_SPEED / max_value);

    (fl * speed, fr * speed, rl * speed, rr * speed)
}

#[derive(Debug)]
pub struct Holonomic {
    front_left_motor: Motor,
    front_right_motor: Motor,
    rear_left_motor: Motor,
    rear_right_motor: Motor,
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

    pub fn set(&mut self, heading: Angle, speed: Option<f64>, rotation_speed: Option<f64>) -> Result<()> {
        let velocities = holonomic_motor_velocities(heading, speed, rotation_speed);
        info!(
            "SETTING RADIANS: {:?} SPEED: {:?} ROT SPEED: {:?}",
            heading, speed, rotation_speed
        );

        self.front_left_motor.set_velocity(velocities.0 as i32)?;
        self.front_right_motor.set_velocity(velocities.1 as i32)?;
        self.rear_left_motor.set_velocity(velocities.2 as i32)?;
        self.rear_right_motor.set_velocity(velocities.3 as i32)?;

        Ok(())
    }
}

#[expect(unused)]
impl Drivetrain for HolonomicDrivetrain {
    async fn drive(&mut self, direction: DriveDirection) -> Result<()> {
        self.set(
            match direction {
                DriveDirection::Forward => Radian::NORTH,
                DriveDirection::Reverse => Radian::SOUTH,
                DriveDirection::Left => Radian::WEST,
                DriveDirection::Right => Radian::EAST,
            },
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

    // fn velocity(&self, unit: Option<VelocityUnit>) -> Result<f64> {
    //     todo!()
    // }

    // fn torque(&self, unit: Option<TorqueUnit>) -> Result<f64> {
    //     todo!()
    // }

    // fn current(&self, unit: Option<CurrentUnit>) -> Result<f64> {
    //     todo!()
    // }

    // fn efficiency_percent(&self) -> Result<f64> {
    //     todo!()
    // }

    // fn power_watts(&self) -> Result<f64> {
    //     todo!()
    // }
}
