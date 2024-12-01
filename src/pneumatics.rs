//! Pneumatics API

use anyhow::Result;
use vexide::{devices::adi::digital::LogicLevel, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PneumaticState {
    Low,
    High,
}

impl From<bool> for PneumaticState {
    fn from(value: bool) -> Self {
        match value {
            true => PneumaticState::High,
            false => PneumaticState::Low,
        }
    }
}

impl From<LogicLevel> for PneumaticState {
    fn from(value: LogicLevel) -> Self {
        match value {
            LogicLevel::Low => PneumaticState::Low,
            LogicLevel::High => PneumaticState::High,
        }
    }
}

#[derive(Debug)]
pub struct Pneumatic {
    adi_digital_out: AdiDigitalOut,
    pneumatics_state: PneumaticState,
}

impl Pneumatic {
    pub fn new(adi_digital_out: AdiDigitalOut, default_state: Option<PneumaticState>) -> Self {
        let pneumatics_state = default_state.unwrap_or(PneumaticState::Low);
        Self {
            adi_digital_out,
            pneumatics_state,
        }
    }

    pub fn toggle(&mut self) -> Result<PneumaticState> {
        match self.pneumatics_state {
            PneumaticState::Low => {
                self.adi_digital_out.set_high()?;
                self.pneumatics_state = PneumaticState::High;
                Ok(PneumaticState::High)
            }
            PneumaticState::High => {
                self.adi_digital_out.set_low()?;
                self.pneumatics_state = PneumaticState::Low;
                Ok(PneumaticState::Low)
            }
        }
    }

    pub fn set(&mut self, state: PneumaticState) -> Result<()> {
        match state {
            PneumaticState::Low => self.adi_digital_out.set_low()?,
            PneumaticState::High => self.adi_digital_out.set_high()?,
        };

        self.pneumatics_state = state;

        Ok(())
    }

    pub fn set_low(&mut self) -> Result<()> {
        self.set(PneumaticState::Low)
    }

    pub fn set_high(&mut self) -> Result<()> {
        self.set(PneumaticState::High)
    }

    pub fn is_low(&mut self) -> bool {
        self.pneumatics_state == PneumaticState::Low
    }

    pub fn is_high(&mut self) -> bool {
        self.pneumatics_state == PneumaticState::High
    }

    pub fn state(&mut self) -> PneumaticState {
        self.pneumatics_state.clone()
    }
}
