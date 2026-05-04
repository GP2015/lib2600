use crate::{Riot, riot::lines::RiotOutputLines};
use emutils::line::LineError;

impl Riot {
    pub(crate) fn read_ir_flag(
        &mut self,
        lines: &mut RiotOutputLines,
        only_instruction: bool,
    ) -> Result<(), LineError> {
        for line_index in 0..8 {
            let (line, con) = lines
                .db
                .line_mut(self.con.db, line_index)
                .expect("already checked");

            match line_index {
                7 => line.copy_from_reg(con, &self.timer_interrupt_flag, only_instruction)?,
                6 => line.copy_from_reg(con, &self.edc_interrupt_flag, only_instruction)?,
                _ => line.add_low(con, only_instruction)?,
            }
        }

        self.edc_interrupt_flag.add(false, only_instruction);

        Ok(())
    }
}
