//! Implements a differential tank drivetrain.

use vexide::prelude::*;

use super::*;

#[derive(Debug)]
pub struct DifferentialDrivetrain {
    left_motor: Motor,
    right_motor: Motor,

    velocity: f64,
}

impl DifferentialDrivetrain {
    fn new(left_motor: Motor, right_motor: Motor) -> Self {
        Self {
            left_motor,
            right_motor,

            velocity: 0.0,
        }
    }
}

impl Drivetrain for DifferentialDrivetrain {
    async fn drive(&self, direction: DriveDirection) -> Result<()> {
        todo!("not implemented")
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
