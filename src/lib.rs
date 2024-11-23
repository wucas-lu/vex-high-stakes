#![no_main]
#![no_std]

mod autonomous;
mod driver;
mod robot;
mod screen;

use anyhow::Result;
pub use autonomous::autonomous_control;
pub use driver::driver_control;
use log::warn;
pub use robot::Robot;
use vexide::prelude::*;

#[inline]
pub fn check_motor(motor: &mut Motor, name: &str) -> Result<()> {
    let over_temperature = motor.is_over_temperature()?;
    let over_current = motor.is_over_current()?;

    if over_temperature {
        warn!(
            "{} of port no. {} overheated, stopping motor",
            name,
            motor.port_number()
        );

        motor.brake(BrakeMode::Coast)?;
    } else {
        motor.set_voltage(Motor::V5_MAX_VOLTAGE)?;
    };

    if over_current {
        warn!(
            "{} of port no. {} is drawing too much current: {:.2}A",
            name,
            motor.port_number(),
            motor.current().unwrap_or(0.0)
        );
    };

    Ok(())
}
