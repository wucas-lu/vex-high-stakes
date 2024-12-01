//! Exports competition autonomous control, driver control, and competition robot.

use log::info;
use vexide::{devices::smart::GpsSensor, prelude::*};

use crate::prelude::*;

pub mod autonomous;
pub mod driver;
pub mod screen;

#[derive(Debug)]
pub struct CompetitionRobot {
    pub controller: Controller,
    pub drivetrain: HolonomicDrivetrain,
    pub intake_motor: Motor,
    pub arm_motor: Motor,
    pub stake_piston: AdiDigitalOut,
    pub gps: GpsSensor,
    pub inertial: InertialSensor,
    pub ringsort_optical: OpticalSensor,
}

impl Compete for CompetitionRobot {
    #[cfg(feature = "competition")]
    async fn autonomous(&mut self) -> () {
        info!("starting autonomous control");
        autonomous::autonomous_control(self).await.unwrap();
    }

    #[cfg(feature = "skills")]
    async fn autonomous(&mut self) -> () {
        info!("starting autonomous skills");
        autonomous::autonomous_skills(self).await.unwrap();
    }

    async fn driver(&mut self) -> () {
        info!("starting driver control");
        driver::driver_control(self).await.unwrap();
    }
}
