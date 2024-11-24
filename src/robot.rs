use evian::prelude::*;
use log::info;
use vexide::prelude::*;

use crate::{autonomous_control, driver_control};

pub struct Robot {
    pub controller: Controller,
    pub drivetrain: Drivetrain<Differential, Option<()>>,
    pub intake_motor: Motor,
    pub stake_piston: AdiDigitalOut,
}

impl Compete for Robot {
    async fn autonomous(&mut self) -> () {
        info!("starting autonomous control");
        autonomous_control(self).await.unwrap();
    }

    async fn driver(&mut self) -> () {
        info!("starting driver control");
        driver_control(self).await.unwrap();
    }
}
