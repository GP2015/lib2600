use crate::{
    common::{
        line::{
            error::LineError,
            ident::LineIdent,
            multi::{BusDriveState, CheckBusDriveState, IsBusDriveState},
            single::{CheckDriveState, DriveState},
        },
        read::{
            multi::MultiRead,
            single::{CheckSingleRead, SingleRead},
        },
    },
    cpu::{Cpu, reads::CpuLineReads},
    full::ext_drives::ExtDrives,
    riot::{Riot, reads::RiotLineReads},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EmuLineStates {
    pub a: MultiRead<13>,
    pub db: MultiRead<8>,
    pub inp1: MultiRead<7>,
    pub inp2: MultiRead<7>,
    pub rdiff: SingleRead,
    pub ldiff: SingleRead,
    pub col: SingleRead,
    pub sel: SingleRead,
    pub res: SingleRead,
    pub rw: SingleRead,
    pub rdy: SingleRead,
}

impl EmuLineStates {
    pub const fn new() -> Self {
        Self {
            a: [SingleRead::Unknown; _],
            db: [SingleRead::Unknown; _],
            inp1: [SingleRead::Unknown; _],
            inp2: [SingleRead::Unknown; _],
            rdiff: SingleRead::Unknown,
            ldiff: SingleRead::Unknown,
            col: SingleRead::Unknown,
            sel: SingleRead::Unknown,
            res: SingleRead::Unknown,
            rw: SingleRead::Unknown,
            rdy: SingleRead::Unknown,
        }
    }

    pub fn update(
        &mut self,
        ext_drives: &ExtDrives,
        cpu: &Cpu,
        riot: &Riot,
    ) -> Result<(), LineError> {
        let mut inp1 = [SingleRead::Unknown; _];
        for (bit, read) in inp1.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp1",
                bit,
            };

            *read = if bit < 4 {
                let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
                DriveState::contend(drives).ok_read_or_error(ident)?
            } else {
                ext_drives.inp1[bit].read_or_error(ident)?
            }
        }

        let mut inp2 = [SingleRead::Unknown; _];
        for (bit, read) in inp2.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp2",
                bit,
            };

            *read = if bit < 4 {
                let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
                DriveState::contend(drives).ok_read_or_error(ident)?
            } else {
                ext_drives.inp2[bit].read_or_error(ident)?
            };
        }

        macro_rules! create_lines {
            ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
                let ident = LineIdent::UniqueLine {
                    name: stringify!($name),
                };
                let $name = DriveState::contend($drives.into_iter()).ok_read_or_error(ident)?;
            )+};
        }

        macro_rules! create_buses {
            ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
                let $name = BusDriveState::contend(&$drives).ok_read_or_error(stringify!($name))?;
            )+};
        }

        create_buses!(
            (a, [ext_drives.a, cpu.a_out]),
            (db, [ext_drives.db, cpu.db_out, riot.db_out])
        );
        create_lines!(
            (rdiff, [ext_drives.rdiff, riot.pb_out[4]]),
            (ldiff, [ext_drives.ldiff, riot.pb_out[3]]),
            (col, [ext_drives.col, riot.pb_out[2]]),
            (sel, [ext_drives.sel, riot.pb_out[1]]),
            (res, [ext_drives.sel, riot.pb_out[0]])
        );

        let rw = cpu.rw_out.read().ok_or_impossible("rw".into())?;
        let rdy = SingleRead::Unknown;

        *self = Self {
            a,
            db,
            inp1,
            inp2,
            rdiff,
            ldiff,
            col,
            sel,
            res,
            rw,
            rdy,
        };

        Ok(())
    }

    pub fn riot_reads(&self) -> RiotLineReads {
        let mut pa = [SingleRead::Unknown; _];
        pa[0..4].copy_from_slice(&self.inp2[0..4]);
        pa[4..8].copy_from_slice(&self.inp1[0..4]);

        RiotLineReads {
            a: self.a[0..7].try_into().expect("same-sized slices"),
            db: self.db,
            pa,
            pb: [self.res, self.sel, self.col, self.ldiff, self.rdiff],
            cs1: self.a[11],
            cs2: self.a[12],
            rs: self.a[9],
            rw: self.rw,
        }
    }

    pub const fn cpu_reads(&self) -> CpuLineReads {
        CpuLineReads {
            db: self.db,
            rdy: self.rdy,
        }
    }
}
