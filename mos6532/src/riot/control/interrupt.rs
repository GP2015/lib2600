use crate::{Riot, riot::states::RiotStates};
use emutils::line::{Bus, LineError};

impl Riot {
    // Only instruction that writes to edc_ir_flag
    pub(crate) fn read_ir_flag(
        &mut self,
        db: &mut Bus<8>,
        states: &RiotStates,
        only_possible: bool,
    ) -> Result<(), LineError> {
        for (i, (db_line, db_line_con)) in db.iter_mut(self.db_con)?.enumerate() {
            match i {
                7 => db_line.copy_from_reg_state(db_line_con, &states.timer_ir_flag)?,
                6 => db_line.copy_from_reg_state(db_line_con, &states.edc_ir_flag)?,
                _ => db_line.add_low(db_line_con)?,
            }
        }

        if only_possible {
            self.reg.edc_ir_flag.remove_all();
        }

        self.reg.edc_ir_flag.add(false);
        Ok(())
    }
}
