//! Implements a differential tank drivetrain.

// I'm lazy
#![expect(unused)]

use vexide::prelude::*;

use super::{BrakeMode, *};

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

#[expect(unused)]
impl Drivetrain for DifferentialDrivetrain {
    async fn drive(&mut self, direction: DriveDirection) -> Result<()> {
        todo!()
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
