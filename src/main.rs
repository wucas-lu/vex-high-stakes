#![no_main]
#![no_std]

extern crate alloc;

use anyhow::Result;
use evian::prelude::*;
use log::info;
use vex_high_stakes::Robot;
use vexide::{prelude::*, startup::banner::themes::THEME_MURICA};

const TEAM_BANNER: &str = " \
 __   _____  _____  _____  _    _
/  | |  ___||  _  ||  _  || |  | |
`| | |___ \\ | |_| || |_| || |  | |
 | |     \\ \\\\____ |\\____ || |/\\| |
_| |_/\\__/ /.___/ /.___/ /\\  /\\  /
\\___/\\____/ \\____/ \\____/  \\/  \\/

W U C A S   L U   1 5 9 9 W   S S I S
Hai Likes Molly version 2.0";

#[vexide::main(banner(theme = THEME_MURICA))]
async fn main(peripherals: Peripherals) -> Result<()> {
    info!("{}", TEAM_BANNER);

    let mut intake_motor = Motor::new(peripherals.port_1, Gearset::Red, Direction::Forward);
    intake_motor.set_velocity(100)?;

    let robot = Robot {
        controller: peripherals.primary_controller,
        drivetrain: Drivetrain::new(
            Differential::new(
                shared_motors![
                    Motor::new(peripherals.port_20, Gearset::Green, Direction::Forward),
                    Motor::new(peripherals.port_18, Gearset::Green, Direction::Forward),
                ],
                shared_motors![
                    Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward),
                    Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward),
                ],
            ),
            None,
        ),
        intake_motor,
        stake_piston: AdiDigitalOut::new(peripherals.adi_a),
    };

    info!("Starting competition");
    robot.compete().await;
}
