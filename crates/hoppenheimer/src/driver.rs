//! Implements competition driver control

use core::{f64::consts::PI, time::Duration};

use anyhow::{Context, Result};
use drivetrains::prelude::*;
// use log::info;
use vexide::{devices::controller::ControllerState, prelude::*};

use crate::prelude::*;

// const INTAKE_SPINNING_VELOCITY: i32 = 100;
// const INTAKE_STOPPED_VELOCITY: i32 = 0;

fn process_controller_state(robot: &mut Robot, controller: ControllerState) -> Result<()> {
    println!("ANGLE: {}", Angle::from_controller_joysticks(&controller).as_degrees());

    robot.drivetrain.set(
        Angle::from_controller_joysticks(&controller),
        Some(PI / 2.0 - speed_from_controller_joysticks(&controller)),
        None,
    )?;

    // if controller.button_x.is_now_pressed() {
    //     if robot.stake_piston.is_high()? {
    //         info!("Unclamping stake");
    //         robot.stake_piston.set_low()?;
    //     } else {
    //         info!("Clamping stake");
    //         robot.stake_piston.set_high()?;
    //     }
    // }

    // if controller.button_r1.is_pressed() {
    //     robot.intake_motor.set_velocity(INTAKE_SPINNING_VELOCITY)?;
    // } else {
    //     robot.intake_motor.set_velocity(INTAKE_STOPPED_VELOCITY)?;
    // }

    Ok(())
}

pub async fn driver_control(robot: &mut Robot) -> Result<()> {
    // robot.intake_motor.set_velocity(INTAKE_STOPPED_VELOCITY)?;

    loop {
        let controller = robot.controller.state().unwrap_or_default();
        process_controller_state(robot, controller).context("failed to process controller state")?;

        sleep(Duration::from_millis(25)).await;
    }
}
