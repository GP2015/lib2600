use crate::{
    common::{
        bit::{multi::BitStringState, single::BitState},
        line::error::LineError,
    },
    cpu::Cpu,
    ext_drives::ExtDrives,
    riot::{Riot, data::RiotLineReads},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct EmuLineStates {
    pub a: BitStringState<13>,
    pub db: BitStringState<8>,
    pub inp1: BitStringState<7>,
    pub inp2: BitStringState<7>,
    pub rdiff: BitState,
    pub ldiff: BitState,
    pub col: BitState,
    pub sel: BitState,
    pub res: BitState,
    pub rw: BitState,
    pub rdy: BitState,
}

impl EmuLineStates {
    pub fn update(
        &mut self,
        ext_drives: &ExtDrives,
        cpu: &Cpu,
        riot: &Riot,
    ) -> Result<(), LineError> {
        todo!()

        // let mut inp1: BitStringState<_> = [BitState::Unknown; _].into();
        // for (bit, read) in inp1.iter_mut().enumerate() {
        //     let ident = LineIdent::BusLine {
        //         bus_name: "inp1",
        //         bit,
        //     };

        //     *read = if bit < 4 {
        //         let drives = [ext_drives.inp1[bit], riot.pa_out[bit + 4]].into_iter();
        //         LineOutput::contend(drives)
        //             .map_short(ident)?
        //             .read()
        //             .map_imp(ident)?
        //     } else {
        //         ext_drives.inp1[bit].read().map_imp(ident)?
        //     }
        // }

        // let mut inp2: BitStringState<_> = [BitState::Unknown; _].into();
        // for (bit, read) in inp2.iter_mut().enumerate() {
        //     let ident = LineIdent::BusLine {
        //         bus_name: "inp2",
        //         bit,
        //     };

        //     *read = if bit < 4 {
        //         let drives = [ext_drives.inp2[bit], riot.pa_out[bit]].into_iter();
        //         LineOutput::contend(drives)
        //             .map_short(ident)?
        //             .read()
        //             .map_imp(ident)?
        //     } else {
        //         ext_drives.inp2[bit].read().map_imp(ident)?
        //     };
        // }

        // macro_rules! create_lines {
        //     ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
        //         let ident = LineIdent::UniqueLine {
        //             name: stringify!($name),
        //         };
        //         let $name = LineOutput::contend($drives.into_iter())
        //             .map_short(ident)?
        //             .read()
        //             .map_imp(ident)?;
        //     )+};
        // }

        // macro_rules! create_buses {
        //     ($(($name:ident, $drives:expr)),+ $(,)?) => {$(
        //         let ident = stringify!($name);
        //         let $name = BusOutput::contend(&$drives)
        //             .map_short(ident.into())?
        //             .read()
        //             .map_imp(ident.into())?;
        //     )+};
        // }

        // create_buses!(
        //     (a, [&ext_drives.a, &cpu.a_out]),
        //     (db, [&ext_drives.db, &cpu.db_out, &riot.db_out])
        // );
        // create_lines!(
        //     (rdiff, [ext_drives.rdiff, riot.pb_out[4]]),
        //     (ldiff, [ext_drives.ldiff, riot.pb_out[3]]),
        //     (col, [ext_drives.col, riot.pb_out[2]]),
        //     (sel, [ext_drives.sel, riot.pb_out[1]]),
        //     (res, [ext_drives.sel, riot.pb_out[0]])
        // );

        // let rw = cpu.rw_out.read().map_imp("rw".into())?;
        // let rdy = BitState::Unknown;

        // *self = Self {
        //     a,
        //     db,
        //     inp1,
        //     inp2,
        //     rdiff,
        //     ldiff,
        //     col,
        //     sel,
        //     res,
        //     rw,
        //     rdy,
        // };

        // Ok(())
    }

    pub fn riot_reads(&self) -> RiotLineReads {
        let mut pa: BitStringState<_> = [BitState::Unknown; _].into();
        pa[0..4].copy_from_slice(&self.inp2[0..4]);
        pa[4..8].copy_from_slice(&self.inp1[0..4]);

        let a_arr: [BitState; _] = self.a[0..7].try_into().expect("same-sized slices");

        RiotLineReads {
            a: a_arr.into(),
            db: self.db,
            pa,
            pb: [self.res, self.sel, self.col, self.ldiff, self.rdiff],
            cs1: self.a[11],
            cs2: self.a[12],
            rs: self.a[9],
            rw: self.rw,
        }
    }
}
