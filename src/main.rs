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

extern crate alloc;

use anyhow::Result;
use log::info;
use vex_high_stakes::{prelude::*, set_velocities};
use vexide::{devices::smart::GpsSensor, prelude::*, startup::banner::themes::THEME_MURICA};

const TEAM_BANNER: &str = r" __   _____  _____  _____ _____
/  | |  ___||  _  ||  _  |_   _|
`| | |___ \ | |_| || |_| | | |
| |     \ \\____ |\____ | | |
_| |_/\__/ /.___/ /.___/ / | |
\___/\____/ \____/ \____/  \_/

Höppenheimer, Destroyer of High Stakes
By Team 1599T Jöppenheimer";

#[vexide::main(banner(theme = THEME_MURICA))]
async fn main(peripherals: Peripherals) -> Result<()> {
    info!("{}", TEAM_BANNER);

    let mut intake_motor = Motor::new(peripherals.port_9, Gearset::Red, Direction::Forward);
    let mut arm_motor = Motor::new(peripherals.port_10, Gearset::Red, Direction::Forward);

    let mut drivetrain_front_left = Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward);
    let mut drivetrain_front_right = Motor::new(peripherals.port_18, Gearset::Green, Direction::Reverse);
    let mut drivetrain_rear_left = Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward);
    let mut drivetrain_rear_right = Motor::new(peripherals.port_20, Gearset::Green, Direction::Reverse);

    set_velocities!(
        100,
        intake_motor,
        arm_motor,
        drivetrain_front_left,
        drivetrain_front_right,
        drivetrain_rear_left,
        drivetrain_rear_right
    );

    let mut stake_piston = Pneumatic::new(AdiDigitalOut::new(peripherals.adi_a), Some(PneumaticState::Low));

    let gps = GpsSensor::new(peripherals.port_1, [2.0, 1.0], ([0.0, 0.0], 90.0))?;
    let inertial = InertialSensor::new(peripherals.port_2);
    let ringsort_optical = OpticalSensor::new(peripherals.port_3);

    let robot = CompetitionRobot {
        controller: peripherals.primary_controller,
        drivetrain: HolonomicDrivetrain::new(
            drivetrain_front_left,
            drivetrain_front_right,
            drivetrain_rear_left,
            drivetrain_rear_right,
        ),
        intake_motor,
        arm_motor,
        stake_piston,
        gps,
        inertial,
        ringsort_optical,
    };

    robot.compete().await;
}
