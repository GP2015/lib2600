use crate::{Riot, RiotError};

impl Riot {
    pub(crate) fn handle_reset(&mut self, only_possible: bool) -> Result<(), RiotError> {
        Ok(())
    }

    // pub(super) fn reset(&mut self) -> Result<(), RiotError> {
    //     self.ram.reset();

    //     self.reg.ddra.write(0)?;
    //     self.reg.ddrb.write(0)?;
    //     self.reg.ora.write(0)?;
    //     self.reg.orb.write(0)?;
    //     self.pa_out().tri_state_out();
    //     self.pb_out().tri_state_out();

    //     self.reg.edc_enable_irq.write(false);
    //     self.reg.edc_use_pos_edge.write(false);

    //     Ok(())
    // }
}
