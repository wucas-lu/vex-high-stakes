//! Implements a holonomic x-drive drivetrain.

use vexide::prelude::*;

use super::*;

pub const PI_OVER_FOUR: f64 = PI / 4.0;
pub const MAX_MOTOR_SPEED: f64 = 127.0;
pub const DEADBAND: f64 = 20.0;

/**
    Radian heading values that can be used with holonomic drivetrains.
*/
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
    fn from(heading: Heading) -> Self {
        match heading {
            HolonomicHeading::North => 0.0_f64,
            HolonomicHeading::Northeast => PI_OVER_FOUR,
            HolonomicHeading::East => PI / 2.0,
            HolonomicHeading::Southeast => 3.0 * PI / 4.0,
            HolonomicHeading::South => PI,
            HolonomicHeading::Southwest => 5.0 * PI / 4.0,
            HolonomicHeading::West => 3.0 * PI / 2.0,
            HolonomicHeading::Northwest => 7.0 * PI / 4.0,
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
) -> Option<HolonomicRadians> {
    let radians = heading_radians.unwrap_or(0.0);
    let speed = speed.unwrap_or(1.0).clamp(0.0, 1.0);
    let rotation_speed = rotation_speed.unwrap_or(0.0).clamp(-MAX_MOTOR_SPEED, MAX_MOTOR_SPEED);

    if speed <= 0.0 {
        if rotation_speed > DEADBAND || rotation_speed < -DEADBAND {
            return Some(HolonomicRadians::new(
                rotation_speed,
                rotation_speed,
                rotation_speed,
                rotation_speed,
            ));
        }

        return Some(HolonomicRadians::new(0.0, 0.0, 0.0, 0.0));
    }

    let fl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;
    let fr = MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
    let rl = -MAX_MOTOR_SPEED * (PI_OVER_FOUR + radians).cos() + rotation_speed;
    let rr = MAX_MOTOR_SPEED * (PI_OVER_FOUR - radians).cos() + rotation_speed;

    let max_value = [fl, fr, rl, rr].into_iter().reduce(f64::max)?;
    let speed = speed * (MAX_MOTOR_SPEED / max_value);

    Some(HolonomicRadians::new(fl * speed, fr * speed, rl * speed, rr * speed))
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
        let radians = calculate_holonomic_radians(radians, speed, rotation_speed)
            .ok_or("failed to calculate holonomic radians")?;

        self.front_left_motor.set_velocity(radians.front_left as i32)?;
        self.front_right_motor.set_velocity(radians.front_right as i32)?;
        self.rear_left_motor.set_velocity(radians.rear_left as i32)?;
        self.rear_right_motor.set_velocity(radians.rear_right as i32)?;

        Ok(())
    }
}

impl Drivetrain for HolonomicDrivetrain {
    async fn drive(&self, direction: DriveDirection) -> Result<()> {
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
        &self,
        distance: f64,
        direction: Option<DriveDirection>,
        unit: Option<DistanceUnit>,
    ) -> Result<()> {
        todo!("not implemented")
    }

    async fn stop(&self, brake_mode: Option<BrakeMode>) -> Result<()> {
        todo!("not implemented")
    }

    async fn turn(&self, direction: TurnDirection) -> Result<()> {
        todo!("not implemented")
    }

    async fn turn_for(&self, angle: f64, unit: Option<RotationUnit>) -> Result<()> {
        todo!("not implemented")
    }

    fn is_spinning(&self) -> Result<bool> {
        todo!("not implemented")
    }

    fn is_done(&self) -> Result<bool> {
        todo!("not implemented")
    }

    fn velocity(&self, unit: Option<VelocityUnit>) -> Result<f64> {
        todo!("not implemented")
    }

    fn torque(&self, unit: Option<TorqueUnit>) -> Result<f64> {
        todo!("not implemented")
    }

    fn current(&self, unit: Option<CurrentUnit>) -> Result<f64> {
        todo!("not implemented")
    }

    fn efficiency_percent(&self) -> Result<f64> {
        todo!("not implemented")
    }

    fn power_watts(&self) -> Result<f64> {
        todo!("not implemented")
    }
}
