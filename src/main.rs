#![no_main]
#![no_std]

extern crate alloc;

use core::time::Duration;

use evian::prelude::*;
use vexide::prelude::*;

// Hai likes Molly v2 by team 1599W
struct Robot {
    controller: Controller,
    drivetrain: Drivetrain<Differential, ()>,

    stake_piston: AdiDigitalOut,
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Starting autonomous control");
    }

    async fn driver(&mut self) {
        println!("Starting driver control");

        let mut stake_clamped = false;

        loop {
            let controller = self.controller.state().unwrap_or_default();

            _ = self.drivetrain.motors.set_voltages((
                controller.left_stick.y() * Motor::V5_MAX_VOLTAGE,
                controller.right_stick.y() * Motor::V5_MAX_VOLTAGE,
            ));

            if controller.button_x.is_now_pressed() {
                if stake_clamped {
                    println!("Unclamping stake");
                    _ = self.stake_piston.set_low();
                } else {
                    println!("Clamping stake");
                    _ = self.stake_piston.set_high();
                }

                stake_clamped = !stake_clamped;
            }

            sleep(Duration::from_millis(25)).await;
        }
    }
}

#[vexide::main(banner(enabled = false))]
async fn main(peripherals: Peripherals) {
    println!("Hai likes Molly v2 by team 1599W");

    Robot {
        controller: peripherals.primary_controller,
        drivetrain: Drivetrain::new(
            Differential::new(
                shared_motors![
                    Motor::new(peripherals.port_19, Gearset::Green, Direction::Forward),
                    Motor::new(peripherals.port_17, Gearset::Green, Direction::Forward),
                ],
                shared_motors![
                    Motor::new(peripherals.port_20, Gearset::Green, Direction::Forward),
                    Motor::new(peripherals.port_18, Gearset::Green, Direction::Forward),
                ],
            ),
            (),
        ),

        stake_piston: AdiDigitalOut::new(peripherals.adi_a),
    }
    .compete()
    .await;
}
