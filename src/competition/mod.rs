//! Exports competition autonomous control, driver control, and competition robot.

use evian::prelude::*;
use log::info;
use vexide::prelude::*;

use crate::prelude::*;

pub mod autonomous;
pub mod driver;
pub mod screen;

pub struct CompetitionRobot {
    pub controller: Controller,
    pub drivetrain: Holonomic,
    pub intake_motor: Motor,
    pub stake_piston: AdiDigitalOut,
}

impl Compete for CompetitionRobot {
    async fn autonomous(&mut self) -> () {
        info!("starting autonomous control");
        autonomous_control(self).await.unwrap();
    }

    async fn driver(&mut self) -> () {
        info!("starting driver control");
        driver_control(self).await.unwrap();
    }
}
