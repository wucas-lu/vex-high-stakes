//! Exports drivetrain measurements and implementations.

use anyhow::Result;

pub mod differential;
pub mod holonomic;

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
    Centimeters,
    Meters,
    Inches,
}

pub enum RotationUnit {
    Degrees,
    Revolution,
}

pub enum CurrentUnit {
    Ampere,
    Percent,
}

pub enum VoltageUnit {
    Volt,
    Milivolts,
}

pub enum VelocityUnit {
    RotationsPerMinute,
    DegreesPerSecond,
    Percent,
}

pub enum TorqueUnit {
    NewtonMeter,
    InchPound,
}

pub enum BrakeMode {
    /**
        Allows the motor to gradually come to a stop.
    */
    Coast,
    /**
        Stops the motor immediately.
    */
    Brake,
    /**
        tops the robot immediately and holds the motor in the stopped position.
    */
    Hold,
}

/**
    A drivetrain allows a robot to be mobile by using wheels, tank treads, or
    another method.

    Trait methods are derived from VEX v5's C++ Drivetrain API:
    https://api.vex.com/v5/home/cpp/Drivetrain.html
*/
#[allow(async_fn_in_trait)]
pub trait Drivetrain {
    async fn drive(&self, direction: DriveDirection) -> Result<()>;
    #[rustfmt::skip]
    async fn drive_for(&self, distance: f64, direction: Option<DriveDirection>, unit: Option<DistanceUnit>) -> Result<()>;
    async fn turn(&self, direction: TurnDirection) -> Result<()>;
    async fn turn_for(&self, angle: f64, unit: Option<RotationUnit>) -> Result<()>;
    fn stop(&self, brake_mode: Option<BrakeMode>) -> Result<()>;
    fn is_spinning(&self) -> Result<bool>;
    fn is_done(&self) -> Result<bool>;
    fn velocity(&self, unit: Option<VelocityUnit>) -> Result<f64>;
    fn current(&self, unit: Option<CurrentUnit>) -> Result<f64>;
    fn torque(&self, unit: Option<TorqueUnit>) -> Result<f64>;
    fn efficiency_percent(&self) -> Result<f64>;
    fn power_watts(&self) -> Result<f64>;
}
