use crate::{Riot, riot::states::RiotStates};
use emutils::{
    line::{Bus, LineError},
    reg::MBitReg,
};
use itertools::iproduct;

impl Riot {
    // Only instruction that writes to ora, orb, ddra, ddrb
    pub(crate) fn call_io(
        &mut self,
        db: &mut Bus<8>,
        states: &RiotStates,
        mut only_possible: bool,
    ) -> Result<(), LineError> {
        let a0 = states.a.line_state::<0>();
        let a1 = states.a.line_state::<1>();

        only_possible &= states.rw.is_defined() & a0.is_defined() & a1.is_defined();

        if states.rw.high {
            db.add_high_z(self.db_con)?;
        }

        for (&a0, &a1, &rw) in iproduct!(
            a0.possible_reads(),
            a1.possible_reads(),
            states.rw.possible_reads(),
        ) {
            let write_io = |reg: &mut MBitReg<8>| {
                if only_possible {
                    reg.remove_all();
                }

                reg.copy_from_bus_state(&states.db);
            };

            let mut read_io = |reg| db.copy_from_reg_state(self.db_con, reg);

            match (a0, a1, rw) {
                (false, false, false) => write_io(&mut self.reg.ora),
                (false, true, false) => write_io(&mut self.reg.orb),
                (true, false, false) => write_io(&mut self.reg.ddra),
                (true, true, false) => write_io(&mut self.reg.ddrb),
                (false, false, true) => read_io(&states.ora)?,
                (false, true, true) => self.read_orb(db, states)?,
                (true, false, true) => read_io(&states.ddra)?,
                (true, true, true) => read_io(&states.ddrb)?,
            }
        }

        Ok(())
    }

    fn read_orb(&self, db: &mut Bus<8>, states: &RiotStates) -> Result<(), LineError> {
        for (bit, ((db_line, db_line_con), ddrb_bit_state)) in db
            .iter_mut(self.db_con)?
            .zip(states.ddrb.iter())
            .enumerate()
        {
            if ddrb_bit_state.low {
                let pb_line_state = states.pb.try_line_state(bit).unwrap();
                db_line.copy_from_line_state(db_line_con, &pb_line_state)?;
            }

            if ddrb_bit_state.high {
                let orb_bit_state = states.orb.try_bit_state(bit).unwrap();
                db_line.copy_from_reg_state(db_line_con, &orb_bit_state)?;
            }
        }

        Ok(())
    }
}
