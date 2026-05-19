pub mod ext_drives;
pub mod line_reads;

use crate::{
    common::line::error::LineError,
    cpu::Cpu,
    full::{ext_drives::ExtDrives, line_reads::EmuLineStates},
    riot::Riot,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emulator {
    cpu: Cpu,
    riot: Riot,
    phi0: bool,
    line_states: EmuLineStates,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            riot: Riot::new(),
            phi0: false,
            line_states: EmuLineStates::new(),
        }
    }

    fn update(&mut self, ext: &ExtDrives) -> Result<(), LineError> {
        self.line_states.update(ext, &self.cpu, &self.riot)
    }

    pub fn tick(&mut self, ext: &ExtDrives) -> Result<(), LineError> {
        self.update(ext)?;
        self.cpu.handle_rising_edge(self.line_states.cpu_reads());

        self.update(ext)?;
        self.riot.handle_rising_edge(self.line_states.riot_reads());

        self.update(ext)?;
        self.cpu.handle_falling_edge(self.line_states.cpu_reads());

        self.update(ext)?;
        self.riot.handle_falling_edge();

        Ok(())
    }
}
