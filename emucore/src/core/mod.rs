pub mod ext_drives;

use crate::{
    common::{
        line::{
            error::LineError,
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        read::single::SingleRead,
    },
    core::ext_drives::EmuLineDrives,
    riot::{core::Riot, lines::RiotLineReads},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Emulator {
    riot: Riot,
    phi0: bool,
}

impl Emulator {
    pub fn tick(&mut self, ext: &EmuLineDrives) -> Result<(), LineError> {
        let riot_line_reads = self.riot_line_reads(ext)?;
        self.riot.drive_phi2(&riot_line_reads, self.phi0);
        Ok(())
    }

    fn riot_line_reads(&self, ext: &EmuLineDrives) -> Result<RiotLineReads, LineError> {
        let mut ext_pa = BusDriveState::default();
        ext_pa[0..4].copy_from_slice(&ext.inp2[0..4]);
        ext_pa[4..8].copy_from_slice(&ext.inp1[0..4]);

        let mut ext_riot_a = BusDriveState::default();
        ext_riot_a.copy_from_slice(&ext.a[0..7]);

        Ok(RiotLineReads {
            db: BusDriveState::contend("db", &[ext.db, self.riot.db_out])?.read(),
            pa: BusDriveState::contend("pa", &[ext_pa, self.riot.pa_out])?.read(),

            pb0: DriveState::contend("res", [ext.res, self.riot.p0_out].into_iter())?.read(),
            pb1: DriveState::contend("sel", [ext.sel, self.riot.p1_out].into_iter())?.read(),
            pb3: DriveState::contend("col", [ext.col, self.riot.p3_out].into_iter())?.read(),
            pb6: DriveState::contend("ldiff", [ext.ldiff, self.riot.p6_out].into_iter())?.read(),
            pb7: DriveState::contend("rdiff", [ext.rdiff, self.riot.p7_out].into_iter())?.read(),

            a: BusDriveState::contend("a", &[ext_riot_a])?.read(),
            cs1: DriveState::contend("cs1", [ext.a[11]].into_iter())?.read(),
            cs2: DriveState::contend("cs2", [ext.a[12]].into_iter())?.read(),
            rs: DriveState::contend("cs2", [ext.a[9]].into_iter())?.read(),
            rw: SingleRead::Unknown,
        })
    }
}
