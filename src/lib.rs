#![no_main]
#![no_std]

pub mod competition;
pub mod drivetrain;
pub mod utils;

pub mod prelude {
    pub use crate::{
        competition::{self, autonomous, driver, CompetitionRobot},
        drivetrain::{
            self,
            differential::DifferentialDrivetrain,
            holonomic::{HolonomicDrivetrain, HolonomicHeading, HolonomicRadians},
        },
        utils::{
            self,
            macros::{self, set_velocities},
        },
    };
}
