use crate::{
    LineIdent,
    common::{
        line::{
            error::{LineError, MapToLineError},
            multi::BusDriveState,
            single::DriveState,
        },
        read::{multi::MultiRead, single::SingleRead},
    },
    cpu::{Cpu, reads::CpuLineReads},
    ext_drives::ExtDrives,
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

impl Default for EmuLineStates {
    fn default() -> Self {
        Self::new()
    }
}

impl EmuLineStates {
    pub fn new() -> Self {
        Self {
            a: [SingleRead::Unknown; _].into(),
            db: [SingleRead::Unknown; _].into(),
            inp1: [SingleRead::Unknown; _].into(),
            inp2: [SingleRead::Unknown; _].into(),
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
        let mut inp1: MultiRead<_> = [SingleRead::Unknown; _].into();
        for (bit, read) in inp1.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp1",
                bit,
            };

            *read = if bit < 4 {
                let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
                DriveState::contend(drives)
                    .map_short(ident)?
                    .read()
                    .map_imp(ident)?
            } else {
                ext_drives.inp1[bit].read().map_imp(ident)?
            }
        }

        let mut inp2: MultiRead<_> = [SingleRead::Unknown; _].into();
        for (bit, read) in inp2.iter_mut().enumerate() {
            let ident = LineIdent::BusLine {
                bus_name: "inp2",
                bit,
            };

            *read = if bit < 4 {
                let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
                DriveState::contend(drives)
                    .map_short(ident)?
                    .read()
                    .map_imp(ident)?
            } else {
                ext_drives.inp2[bit].read().map_imp(ident)?
            };
        }

        macro_rules! create_lines {
            ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
                let ident = LineIdent::UniqueLine {
                    name: stringify!($name),
                };
                let $name = DriveState::contend($drives.into_iter())
                    .map_short(ident)?
                    .read()
                    .map_imp(ident)?;
            )+};
        }

        macro_rules! create_buses {
            ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
                let ident = stringify!($name);
                let $name = BusDriveState::contend(&$drives)
                    .map_short(ident.into())?
                    .read()
                    .map_imp(ident.into())?;
            )+};
        }

        create_buses!(
            (a, [&ext_drives.a, &cpu.a_out]),
            (db, [&ext_drives.db, &cpu.db_out, &riot.db_out])
        );
        create_lines!(
            (rdiff, [ext_drives.rdiff, riot.pb_out[4]]),
            (ldiff, [ext_drives.ldiff, riot.pb_out[3]]),
            (col, [ext_drives.col, riot.pb_out[2]]),
            (sel, [ext_drives.sel, riot.pb_out[1]]),
            (res, [ext_drives.sel, riot.pb_out[0]])
        );

        let rw = cpu.rw_out.read().map_imp("rw".into())?;
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
        let mut pa: MultiRead<_> = [SingleRead::Unknown; _].into();
        pa[0..4].copy_from_slice(&self.inp2[0..4]);
        pa[4..8].copy_from_slice(&self.inp1[0..4]);

        let a_arr: [SingleRead; _] = self.a[0..7].try_into().expect("same-sized slices");

        RiotLineReads {
            a: a_arr.into(),
            db: self.db,
            pa,
            pb: [self.res, self.sel, self.col, self.ldiff, self.rdiff].into(),
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
