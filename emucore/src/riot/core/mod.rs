mod control;
mod instructions;
mod reads;
mod registers;

use crate::{
    common::{
        line::{
            drive_state::DriveState,
            error::LineError,
            multi::{Bus, BusConId},
        },
        read::single::SingleRead,
        reg::multi::MBitReg,
        signal::LineSignal,
    },
    riot::{
        core::{instructions::PossibleInstructions, reads::RiotReads, registers::RiotRegs},
        lines::RiotLines,
    },
};
use std::array;

const RAM_SIZE: usize = 128;
const TIMER_INTERVALS: [usize; 4] = [1, 8, 64, 1024];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    db_con: BusConId,
    pa_con: BusConId,
    pb_con: BusConId,
    reg: RiotRegs,
    ram: [MBitReg<8>; RAM_SIZE],
    phi2_signal: bool,
    old_pa7_state: SingleRead,
}

impl Riot {
    pub fn new(db: &mut Bus<8>, pa: &mut Bus<8>, pb: &mut Bus<8>) -> Self {
        let riot = Self {
            db_con: db.create_connection(),
            pa_con: pa.create_connection(),
            pb_con: pb.create_connection(),
            reg: RiotRegs::new(),
            ram: array::from_fn(|_| MBitReg::new(SingleRead::Unknown)),
            phi2_signal: false,
            old_pa7_state: SingleRead::Unknown,
        };

        let high_z_out = [DriveState::from_only(LineSignal::HighZ); 8];
        pa.set_drive_state(riot.pa_con, &high_z_out).unwrap();
        pb.set_drive_state(riot.pb_con, &high_z_out).unwrap();

        riot
    }

    pub fn drive_phi2(&mut self, lines: RiotLines, bool_signal: bool) -> Result<(), LineError> {
        lines.check_valid()?;

        match (self.phi2_signal, bool_signal) {
            (false, true) => self.handle_rising_edge(lines)?,
            (true, false) => {
                let high_z_out = [DriveState::from_only(LineSignal::HighZ); 8];
                lines.db.set_drive_state(self.db_con, &high_z_out).unwrap();
            }
            _ => return Ok(()),
        }

        self.phi2_signal = bool_signal;
        Ok(())
    }

    fn handle_rising_edge(&mut self, lines: RiotLines) -> Result<(), LineError> {
        let states = RiotReads::new(&lines, &self.reg);
        let RiotLines { db, pa, pb, .. } = lines;

        db.remove_all(self.db_con)?;
        pa.remove_all(self.pa_con)?;
        pb.remove_all(self.pb_con)?;

        self.update_edc(pa);
        self.update_timer();

        if states.cs1.could_read_high() && states.cs2.could_read_low() {
            let instructions = PossibleInstructions::from(&states);

            let only_possible = instructions.only_possible()
                && lines.cs1.state().could_read_low()
                && !states.cs2.could_read_high();

            if instructions.wt || instructions.rt {
                self.clear_timer_ir_flag(instructions.timer_only_possible());
            }

            macro_rules! call_instr_fns {
                ($(($instr:ident, $action:expr)),+ $(,)?) => {$(
                    if instructions.$instr {
                        $action;
                    }
                )+};
            }

            call_instr_fns!(
                (ram, self.call_ram(db, &states, only_possible)?),
                (io, self.call_io(db, &states, only_possible)?),
                (wt, self.write_timer(&states, only_possible)),
                (rt, self.read_timer(db)?),
                (rirf, self.read_ir_flag(db, &states, only_possible)?),
                (wedc, self.write_edc(db, &states, only_possible)?),
            );

            if instructions.io {
                self.update_peripherals(pa, pb)?;
            }
        }

        Ok(())
    }
}
