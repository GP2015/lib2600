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
    core::ext_drives::ExtDrives,
    riot::{Riot, reads::RiotLineReads},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Emulator {
    riot: Riot,
    phi0: bool,
}

impl Emulator {
    pub fn tick(&mut self, ext: &ExtDrives) -> Result<(), LineError> {
        let riot_line_reads = self.riot_line_reads(ext)?;
        self.riot.drive_phi2(&riot_line_reads, self.phi0);
        Ok(())
    }

    fn riot_line_reads(&self, ext: &ExtDrives) -> Result<RiotLineReads, LineError> {
        let mut ext_pa = BusDriveState::default();
        ext_pa[0..4].copy_from_slice(&ext.inp2[0..4]);
        ext_pa[4..8].copy_from_slice(&ext.inp1[0..4]);

        let mut ext_a = BusDriveState::default();
        ext_a.copy_from_slice(&ext.a[0..7]);

        let ext_pb = [ext.res, ext.sel, ext.col, ext.ldiff, ext.rdiff];

        Ok(RiotLineReads {
            db: BusDriveState::contend("db", &[ext.db, self.riot.db_out])?.read(),
            pa: BusDriveState::contend("pa", &[ext_pa, self.riot.pa_out])?.read(),
            pb: BusDriveState::contend("pa", &[ext_pb, self.riot.pb_out])?.read(),

            a: BusDriveState::contend("a", &[ext_a])?.read(),
            cs1: DriveState::contend("cs1", [ext.a[11]].into_iter())?.read(),
            cs2: DriveState::contend("cs2", [ext.a[12]].into_iter())?.read(),
            rs: DriveState::contend("cs2", [ext.a[9]].into_iter())?.read(),
            rw: SingleRead::Unknown,
        })
    }
}
