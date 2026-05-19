use crate::{
    common::{
        line::{
            error::LineError,
            ident::LineIdent,
            multi::{BusDriveState, CheckBusDriveState, IsBusDriveState},
            single::{CheckDriveState, DriveState},
        },
        read::{multi::MultiRead, single::SingleRead},
    },
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
}

impl EmuLineStates {
    pub fn new() -> Self {
        Self {
            a: MultiRead::from([SingleRead::Unknown; _]),
            db: MultiRead::from([SingleRead::Unknown; _]),
            inp1: MultiRead::from([SingleRead::Unknown; _]),
            inp2: MultiRead::from([SingleRead::Unknown; _]),
            rdiff: SingleRead::Unknown,
            ldiff: SingleRead::Unknown,
            col: SingleRead::Unknown,
            sel: SingleRead::Unknown,
            res: SingleRead::Unknown,
            rw: SingleRead::Unknown,
        }
    }

    pub fn update(&mut self, ext_drives: &ExtDrives, riot: &Riot) -> Result<(), LineError> {
        let mut inp1: [SingleRead; 7] = MultiRead::from([SingleRead::Unknown; _]);
        for (bit, read) in inp1.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp1",
                bit,
            };

            *read = if bit < 4 {
                #[expect(clippy::indexing_slicing)]
                let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
                DriveState::contend(drives).ok_read_or_error(ident)?
            } else {
                #[expect(clippy::indexing_slicing)]
                ext_drives.inp1[bit].read_or_error(ident)?
            }
        }

        let mut inp2: [SingleRead; 7] = MultiRead::from([SingleRead::Unknown; _]);
        for (bit, read) in inp2.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp2",
                bit,
            };

            *read = if bit < 4 {
                #[expect(clippy::indexing_slicing)]
                let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
                DriveState::contend(drives).ok_read_or_error(ident)?
            } else {
                #[expect(clippy::indexing_slicing)]
                ext_drives.inp2[bit].read_or_error(ident)?
            };
        }

        let a = ext_drives.a.read_or_error("a")?;

        let drives = &[ext_drives.db, riot.db_out];
        let db = BusDriveState::contend(drives).ok_read_or_error("db")?;

        macro_rules! create {
            ($name:ident, $drives:expr) => {
                let ident = LineIdent::UniqueLine {
                    name: stringify!($name),
                };
                let $name = DriveState::contend($drives.into_iter()).ok_read_or_error(ident)?;
            };
        }

        create!(rdiff, [ext_drives.rdiff, riot.pb_out[4]]);
        create!(ldiff, [ext_drives.ldiff, riot.pb_out[3]]);
        create!(col, [ext_drives.col, riot.pb_out[2]]);
        create!(sel, [ext_drives.sel, riot.pb_out[1]]);
        create!(res, [ext_drives.sel, riot.pb_out[0]]);

        let rw = SingleRead::Unknown;

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
        };

        Ok(())
    }

    pub fn riot_reads(&self) -> RiotLineReads {
        let mut pa = MultiRead::from([SingleRead::Unknown; _]);
        pa[0..4].copy_from_slice(&self.inp2[0..4]);
        pa[4..8].copy_from_slice(&self.inp1[0..4]);

        RiotLineReads {
            db: self.db,
            pa,
            pb: [self.res, self.sel, self.col, self.ldiff, self.rdiff],
            a: self.a[0..7].try_into().expect("same-sized slices"),
            cs1: self.a[11],
            cs2: self.a[12],
            rs: self.a[9],
            rw: self.rw,
        }
    }
}
