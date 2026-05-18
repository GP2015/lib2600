use crate::{
    BusDriveState, DriveState, ExtDrives, IsBusDriveState, LineError,
    common::read::{multi::MultiRead, single::SingleRead},
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
        let mut inp1 = MultiRead::from([SingleRead::Unknown; _]);
        for (bit, read) in inp1.iter_mut().enumerate() {
            *read = match bit {
                0..4 => {
                    #[expect(clippy::indexing_slicing)]
                    let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
                    DriveState::contend("inp1(0..4)", drives)?.read()
                }
                4..7 =>
                {
                    #[expect(clippy::indexing_slicing)]
                    ext_drives.inp1[bit].read()
                }
                _ => unreachable!(),
            }
        }

        let mut inp2 = MultiRead::from([SingleRead::Unknown; _]);
        for (bit, read) in inp2.iter_mut().enumerate() {
            *read = match bit {
                0..4 => {
                    #[expect(clippy::indexing_slicing)]
                    let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
                    DriveState::contend("inp2(0..4)", drives)?.read()
                }
                4..7 =>
                {
                    #[expect(clippy::indexing_slicing)]
                    ext_drives.inp2[bit].read()
                }
                _ => unreachable!(),
            }
        }

        let a = ext_drives.a.read();

        let drives = &[ext_drives.db, riot.db_out];
        let db = BusDriveState::contend("db", drives)?.read();

        macro_rules! create {
            ($name:ident, $drives:expr) => {
                let $name = DriveState::contend(stringify!($name), $drives.into_iter())?.read();
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
