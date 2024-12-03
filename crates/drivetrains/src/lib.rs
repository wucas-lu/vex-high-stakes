#![no_std]
#![no_main]

/// Drivetrain measurements and implementations.
use anyhow::Result;

pub mod angle;
pub mod holonomic;

use angle::Angle;

pub mod prelude {
    pub use crate::{
        angle::{Angle, IntoAngle},
        holonomic::{
            speed_from_controller_joysticks, speed_from_single_controller_joystick, Holonomic, Radian,
            DEADBAND as HOLONOMIC_MOTOR_DEADBAND, MAX_MOTOR_SPEED as HOLONOMIC_MAX_MOTOR_SPEED,
        },
        CurrentUnit, DistanceUnit, DriveDirection, Drivetrain, RotationUnit, TorqueUnit, TurnDirection, VelocityUnit,
        VoltageUnit,
    };
}

pub enum DriveDirection {
    Forward,
    Reverse,
    Left,
    Right,
}

pub enum TurnDirection {
    Forward,
    Reverse,
}

pub enum DistanceUnit {
    Centimeters(f64),
    Meters(f64),
    Inches(f64),
}

pub enum CurrentUnit {
    Ampere(f64),
    Percent(f64),
}

pub enum VoltageUnit {
    Volt(f64),
    Milivolts(f64),
}

pub enum VelocityUnit {
    RotationsPerMinute(f64),
    DegreesPerSecond(f64),
    Percent(f64),
}

pub enum TorqueUnit {
    NewtonMeter(f64),
    InchPound(f64),
}

pub enum Brake {
    /// Allows the motor to gradually come to a stop.
    Coast,
    /// Stops the motor immediately.
    Brake,
    /// Stops the robot immediately and holds the motor in the stopped position.
    Hold,
}

/// A drivetrain allows a robot to be mobile by using wheels, tank treads, or
/// another method.
///
/// Trait methods are derived from VEX V5's C++ Drivetrain API:
/// https://api.vex.com/v5/home/cpp/Drivetrain.html
#[allow(async_fn_in_trait)]
pub trait Drivetrain {
    async fn drive(&mut self, direction: DriveDirection) -> Result<()>;
    async fn drive_for(&mut self, distance: f64, heading: Angle, unit: DistanceUnit) -> Result<()>;
    async fn turn(&mut self, turn: Angle) -> Result<()>;
    async fn turn_for(&mut self, turn: Angle) -> Result<()>;
    fn stop(&mut self, mode: Brake) -> Result<()>;
    fn is_spinning(&self) -> Result<bool>;
    fn is_done(&self) -> Result<bool>;
    // fn velocity(&self, unit: VelocityUnit) -> Result<f64>;
    // fn current(&self, unit: CurrentUnit) -> Result<f64>;
    // fn torque(&self, unit: TorqueUnit) -> Result<f64>;
    // fn efficiency_percent(&self) -> Result<f64>;
    // fn power_watts(&self) -> Result<f64>;

    // TODO: set_velocity, set_direction?
}
