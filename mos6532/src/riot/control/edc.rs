use crate::{Riot, RiotError, RiotLineRefs};

impl Riot {
    pub(crate) fn write_edc(
        &mut self,
        lines: &mut RiotLineRefs,
        only_possible: bool,
    ) -> Result<(), RiotError> {
        todo!()
    }

    // pub(super) fn write_edc(&mut self, enable_irq: bool, use_pos_edge: bool) {
    //     self.edc_enable_irq.write(enable_irq);
    //     self.edc_use_pos_edge.write(use_pos_edge);
    // }

    // pub(super) fn update_edc(&mut self) -> Result<(), RiotError> {
    //     let new_pa7 = self.pa().bit_state(7)?;

    //     if new_pa7 != self.old_pa7 && new_pa7 == self.edc_use_pos_edge.read()? {
    //         self.edc_interrupt_flag.write(true);
    //     }

    //     self.old_pa7.write(new_pa7);

    //     Ok(())
    // }
}
