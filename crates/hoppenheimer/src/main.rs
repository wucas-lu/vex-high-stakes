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

use core::future::IntoFuture;

use anyhow::Result;
use drivetrains::prelude::*;
use hoppenheimer::prelude::*;
use vexide::{
    /* devices::smart::GpsSensor, */ devices::smart::GpsSensor, prelude::*, startup::banner::themes::THEME_MURICA,
};

const TEAM_BANNER: &str = r"
 __   _____  _____  _____ _____
/  | |  ___||  _  ||  _  |_   _|
`| | |___ \ | |_| || |_| | | |
 | |     \ \\____ |\____ | | |
_| |_/\__/ /.___/ /.___/ / | |
\___/\____/ \____/ \____/  \_/

Höppenheimer, Destroyer of High Stakes
By Team 1599T Oppenheimer";

#[vexide::main(banner(theme = THEME_MURICA))]
async fn main(mut peripherals: Peripherals) -> Result<()> {
    println!("{}", TEAM_BANNER);

    let gps = GpsSensor::new(peripherals.port_2, [2.0, 1.0], ([0.0, 0.0], 90.0));
    let inertial = InertialSensor::new(peripherals.port_3);
    let intake_optic = OpticalSensor::new(peripherals.port_4);
    let mogo_distance = DistanceSensor::new(peripherals.port_5);

    let mut intake = Motor::new(peripherals.port_9, Gearset::Red, Direction::Forward);
    let mut intake_lift = Motor::new(peripherals.port_10, Gearset::Red, Direction::Forward);
    let mut left_robot_lift = Motor::new(peripherals.port_14, Gearset::Red, Direction::Forward);
    let mut right_robot_lift = Motor::new(peripherals.port_15, Gearset::Red, Direction::Forward);

    let mut drivetrain_front_left = Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward);
    let mut drivetrain_front_right = Motor::new(peripherals.port_18, Gearset::Green, Direction::Reverse);
    let mut drivetrain_rear_left = Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward);
    let mut drivetrain_rear_right = Motor::new(peripherals.port_20, Gearset::Green, Direction::Reverse);

    set_velocities!(
        100,
        intake_motor,
        intake_lift,
        left_robot_lift,
        right_robot_lift,
        drivetrain_front_left,
        drivetrain_front_right,
        drivetrain_rear_left,
        drivetrain_rear_right
    );

    let mut mogo_solenoid = AdiDigitalOut::new(peripherals.adi_a);
    let mut intake_solenoid = AdiDigitalOut::new(peripherals.adi_b);

    mogo_solenoid.set_high()?;
    intake_solenoid.set_high()?;

    println!("Calibrating inertial");
    inertial.calibrate().await?;
    println!("Finished calibration");

    let robot = Robot {
        controller: peripherals.primary_controller,

        gps,
        inertial,
        intake_optic,
        mogo_distance,

        intake,
        intake_lift,
        left_robot_lift,
        right_robot_lift,

        drivetrain: Holonomic::new(
            drivetrain_front_left,
            drivetrain_front_right,
            drivetrain_rear_left,
            drivetrain_rear_right,
        ),

        mogo_solenoid,
        intake_solenoid,
    };

    draw_oppenheimer(&mut peripherals.display)?;

    robot.compete().await;
}
