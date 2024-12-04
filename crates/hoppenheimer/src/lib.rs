/*
     __   _____  _____  _____ _____
    /  | |  ___||  _  ||  _  |_   _|
    `| | |___ \ | |_| || |_| | | |
     | |     \ \\____ |\____ | | |
    _| |_/\__/ /.___/ /.___/ / | |
    \___/\____/ \____/ \____/  \_/

    Höppenheimer, Destroyer of High Stakes
    By Team 1599T Jöppenheimer
*/

#![no_main]
#![no_std]

use drivetrains::prelude::*;
use log::info;
use vexide::prelude::*;

pub mod autonomous;
pub mod driver;
pub mod macros;
pub mod screen;

pub mod prelude {
    pub use crate::{set_velocities, Robot, screen::draw_oppenheimer};
}

#[derive(Debug)]
pub struct Robot {
    pub controller: Controller,
    pub drivetrain: Holonomic,
    // pub intake_motor: Motor,
    // pub arm_motor: Motor,
    // pub stake_piston: Pneumatic,
    // pub gps: GpsSensor,
    // pub inertial: InertialSensor,
    // pub ringsort_optical: OpticalSensor,
}

impl Compete for Robot {
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
