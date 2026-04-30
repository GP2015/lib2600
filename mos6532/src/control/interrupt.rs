use crate::{Riot, RiotError, RiotLineRefs};

impl Riot {
    #[allow(unused_variables)]
    pub(crate) fn handle_read_interrupt_flag(
        &mut self,
        lines: &mut RiotLineRefs,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        todo!()
    }

    // pub(super) fn read_interrupt_flag(&mut self) -> Result<(), RiotError> {
    //     let edc_interrupt_flag_usize = self.edc_interrupt_flag.read()? as usize;
    //     let timer_flag_usize = self.timer_flag.read()? as usize;
    //     let interrupt_reg = (edc_interrupt_flag_usize << 7) | (timer_flag_usize << 6);

    //     self.db_out().drive_out(interrupt_reg)?;

    //     self.edc_interrupt_flag.write(false);

    //     Ok(())
    // }
}
