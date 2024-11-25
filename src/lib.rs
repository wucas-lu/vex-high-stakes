#![no_main]
#![no_std]

mod autonomous;
mod driver;
mod holonomic;
mod robot;
mod screen;

use anyhow::Result;
pub use autonomous::autonomous_control;
pub use driver::driver_control;
pub use holonomic::*;
use log::warn;
pub use robot::Robot;
use vexide::prelude::*;

pub async fn check_motor(motor: &mut Motor) -> Result<()> {
    let over_temperature = motor.is_over_temperature()?;
    let over_current = motor.is_over_current()?;

    if over_temperature {
        warn!("Motor port no. {} overheated, stopping motor", motor.port_number());

        motor.brake(BrakeMode::Coast)?;
    } else {
        motor.set_voltage(Motor::V5_MAX_VOLTAGE)?;
    };

    if over_current {
        warn!(
            "Motor port no. {} is drawing too much current: {:.2}A",
            motor.port_number(),
            motor.current().unwrap_or(0.0)
        );
    };

    Ok(())
}

/**
    Observes the given motor ensuring it does not overheat and does not draw too
    much current.
*/
#[macro_export]
macro_rules! observe_motors {
    ($( $m:expr ),* $(,)?) => {{
        todo!("FIXME: this shit broken asf");
        // use vexide::prelude::*;
        // use log::{info, warn};
        // use vex_high_stakes::check_motor;

        // $(
        //     info!("observing motor port no. {}", $m.port_number());

        //     spawn(async {
        //         loop {
        //             if let Err(e) = check_motor(&mut $m).await {
        //                 warn!(
        //                     "failed to observe motor port no. {}: {}",
        //                     $m.port_number(),
        //                     e
        //                 );
        //             }
        //         }
        //     });
        // )*
    }};
}

/**
    Sets the given motors' velocities to the specified velocity in RPM.
*/
#[macro_export]
macro_rules! set_velocities {
    ($v:expr, $( $m:expr ),* $(,)?) => {{
        $(
            $m.set_velocity($v)?;
        )*
    }};
}
