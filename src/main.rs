#![no_main]
#![no_std]

extern crate alloc;

use anyhow::Result;
use evian::prelude::*;
use log::info;
use vex_high_stakes::{set_velocities, Robot};
use vexide::{
    devices::smart::GpsSensor,
    prelude::*,
    startup::{banner::themes::THEME_MURICA, *},
};

const TEAM_BANNER: &str = " \
 __   _____  _____  _____  _    _
/  | |  ___||  _  ||  _  || |  | |
`| | |___ \\ | |_| || |_| || |  | |
 | |     \\ \\\\____ |\\____ || |/\\| |
_| |_/\\__/ /.___/ /.___/ /\\  /\\  /
\\___/\\____/ \\____/ \\____/  \\/  \\/

W U C A S   L U   1 5 9 9 W   S S I S";

const CODE_SIGNATURE: CodeSignature =
    CodeSignature::new(ProgramType::User, ProgramOwner::Partner, ProgramFlags::empty());

#[vexide::main(code_sig = CODE_SIGNATURE, banner(theme = THEME_MURICA))]
async fn main(peripherals: Peripherals) -> Result<()> {
    info!("{}", TEAM_BANNER);

    // TODO move intake motor port to 10 from port 1
    let mut intake_motor = Motor::new(peripherals.port_10, Gearset::Red, Direction::Forward);

    let drivetrain_top_left = Motor::new(peripherals.port_20, Gearset::Green, Direction::Forward);
    let drivetrain_top_right = Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward);
    let drivetrain_bottom_left = Motor::new(peripherals.port_18, Gearset::Green, Direction::Forward);
    let drivetrain_bottom_right = Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward);

    let drivetrain_left = shared_motors![drivetrain_top_left, drivetrain_bottom_left];
    let drivetrain_right = shared_motors![drivetrain_top_right, drivetrain_bottom_right];

    let gps = GpsSensor::new(peripherals.port_1, [2.0, 1.0], ([0.0, 0.0], 90.0))?;
    let inertial = InertialSensor::new(peripherals.port_2);

    // observe_motors!(
    //     intake_motor,
    //     drivetrain_top_left,
    //     drivetrain_top_right,
    //     drivetrain_bottom_left,
    //     drivetrain_bottom_right
    // );

    set_velocities!(100, intake_motor);

    let robot = Robot {
        controller: peripherals.primary_controller,
        drivetrain: Drivetrain::new(Differential::new(drivetrain_left, drivetrain_right), None),
        intake_motor,
        stake_piston: AdiDigitalOut::new(peripherals.adi_a),
    };

    info!("starting competition");
    robot.compete().await;

    Ok(())
}
