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

pub mod competition;
pub mod drivetrain;
pub mod utils;
pub mod pneumatics;

pub mod prelude {
    pub use crate::{
        competition::{self, autonomous, driver, CompetitionRobot},
        drivetrain::{
            self,
            differential::DifferentialDrivetrain,
            holonomic::{
                radians_from_controller_joystick, speed_from_controller_joystick, HolonomicDrivetrain,
                HolonomicHeading, HolonomicRadians,
            },
        },
        utils::{self, macros},
        pneumatics::{self, Pneumatic, PneumaticState};
    };
}
