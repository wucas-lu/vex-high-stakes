use core::time::Duration;

use anyhow::Result;
use log::{info, warn};
use vexide::{devices::controller::ControllerState, prelude::*};

use crate::{check_motor, Robot};

fn process_controller_state(robot: &mut Robot, controller: ControllerState) -> Result<()> {
    robot.drivetrain.motors.set_voltages((
        controller.left_stick.y() * Motor::V5_MAX_VOLTAGE,
        controller.right_stick.y() * Motor::V5_MAX_VOLTAGE,
    ))?;

    let mut stake_clamped = false;
    let mut intake_spinning = false;

    if controller.button_x.is_now_pressed() {
        if stake_clamped {
            info!("Unclamping stake");
            robot.stake_piston.set_low()?;
        } else {
            info!("Clamping stake");
            robot.stake_piston.set_high()?;
        }

        stake_clamped = !stake_clamped;
    }

    if controller.button_r1.is_pressed() {
        if !intake_spinning {
            info!("Spinning intake");
            intake_spinning = true;
        }
        robot.intake_motor.set_velocity(100)?;
    } else {
        if intake_spinning {
            info!("Stopping intake");
            intake_spinning = false;
        }
        robot.intake_motor.set_velocity(0)?;
    }

    Ok(())
}

pub async fn driver_control(robot: &mut Robot) -> Result<()> {
    robot.stake_piston.set_low()?;
    robot.intake_motor.set_velocity(0)?;

    loop {
        let controller = robot.controller.state().unwrap_or_default();

        if let Err(e) = process_controller_state(robot, controller) {
            warn!("Failed to process controller state: {e}");
            return Err(e);
        }

        check_motor(&mut robot.intake_motor, "Intake motor")?;

        sleep(Duration::from_millis(25)).await;
    }
}
