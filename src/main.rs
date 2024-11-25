#![no_main]
#![no_std]

extern crate alloc;

use anyhow::Result;
use evian::prelude::*;
use log::info;
use vex_high_stakes::{set_velocities, Holonomic, Robot};
use vexide::{devices::smart::GpsSensor, prelude::*, startup::banner::themes::THEME_MURICA};

const TEAM_BANNER: &str = " \
 __   _____  _____  _____  _    _
/  | |  ___||  _  ||  _  || |  | |
`| | |___ \\ | |_| || |_| || |  | |
 | |     \\ \\\\____ |\\____ || |/\\| |
_| |_/\\__/ /.___/ /.___/ /\\  /\\  /
\\___/\\____/ \\____/ \\____/  \\/  \\/

W U C A S   L U   1 5 9 9 W   S S I S";

#[vexide::main(banner(theme = THEME_MURICA))]
async fn main(peripherals: Peripherals) -> Result<()> {
    info!("{}", TEAM_BANNER);

    // TODO move intake motor port to 10 from port 1
    let mut intake_motor = Motor::new(peripherals.port_10, Gearset::Red, Direction::Forward);

    let mut drivetrain_front_left = Motor::new(peripherals.port_20, Gearset::Green, Direction::Forward);
    let mut drivetrain_front_right = Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward);
    let mut drivetrain_rear_left = Motor::new(peripherals.port_18, Gearset::Green, Direction::Forward);
    let mut drivetrain_rear_right = Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward);

    set_velocities!(
        100,
        intake_motor,
        drivetrain_front_left,
        drivetrain_front_right,
        drivetrain_rear_left,
        drivetrain_rear_right
    );

    let gps = GpsSensor::new(peripherals.port_1, [2.0, 1.0], ([0.0, 0.0], 90.0))?;
    let inertial = InertialSensor::new(peripherals.port_2);

    // observe_motors!(
    //     intake_motor,
    //     drivetrain_top_left,
    //     drivetrain_top_right,
    //     drivetrain_bottom_left,
    //     drivetrain_bottom_right
    // );

    let robot = Robot {
        controller: peripherals.primary_controller,
        drivetrain: Holonomic::new(
            drivetrain_front_left,
            drivetrain_front_right,
            drivetrain_rear_left,
            drivetrain_rear_right,
        ),
        intake_motor,
        stake_piston: AdiDigitalOut::new(peripherals.adi_a),
    };

    info!("starting competition");
    robot.compete().await;

    Ok(())
}
