use crate::{Riot, RiotError, RiotLineRefs};

impl Riot {
    #[allow(unused_variables)]
    pub(crate) fn handle_read_interrupt_flag(
        &mut self,
        lines: &mut RiotLineRefs,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        for line_index in 0..8 {
            let (line, con) = lines
                .db
                .line_mut(&self.db_con, line_index)
                .expect("already checked");

            match line_index {
                7 => line.copy_from_reg(con, &self.timer_interrupt_flag, only_possible)?,
                6 => line.copy_from_reg(con, &self.edc_interrupt_flag, only_possible)?,
                _ => line.add_low(con, only_possible)?,
            }
        }

        self.edc_interrupt_flag.add(false, only_possible);

        Ok(())
    }
}
