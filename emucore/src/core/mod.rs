pub mod ext_drives;
pub mod line_reads;

use crate::{
    common::line::error::LineError,
    core::{ext_drives::ExtDrives, line_reads::EmuLineStates},
    riot::Riot,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emulator {
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
            riot: Riot::new(),
            phi0: false,
            line_states: EmuLineStates::new(),
        }
    }

    pub fn tick(&mut self, ext: &ExtDrives) -> Result<(), LineError> {
        self.line_states.update(ext, &self.riot)?;

        let riot_reads = self.line_states.riot_reads();
        self.riot.drive_phi2(&riot_reads, self.phi0);

        self.line_states.update(ext, &self.riot)
    }
}
