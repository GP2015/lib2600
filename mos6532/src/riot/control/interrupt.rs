use crate::{Riot, riot::lines::RiotOutputLines};
use emutils::line::LineError;

impl Riot {
    pub(crate) fn read_ir_flag(
        &mut self,
        lines: &mut RiotOutputLines,
        only_instruction: bool,
    ) -> Result<(), LineError> {
        for (i, (line, con)) in lines.db.iter_mut(self.con.db)?.enumerate() {
            match i {
                7 => line.copy_from_reg(con, &self.timer_ir_flag, only_instruction)?,
                6 => line.copy_from_reg(con, &self.edc_ir_flag, only_instruction)?,
                _ => line.add_low(con, only_instruction)?,
            }
        }

        self.edc_ir_flag.add(false, only_instruction);

        Ok(())
    }
}
