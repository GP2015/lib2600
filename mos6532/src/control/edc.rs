use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn write_edc(&mut self, enable_irq: bool, use_pos_edge: bool) {
        self.reg.edc_enable_irq.drive(enable_irq);
        self.reg.edc_use_pos_edge.drive(use_pos_edge);
    }

    pub(super) fn update_edc(&mut self) -> Result<(), RiotError> {
        let new_pa7 = self.buf.pa.read_bit(7)?;

        if new_pa7 != self.reg.old_pa7.read()? && new_pa7 == self.reg.edc_use_pos_edge.read()? {
            self.reg.edc_interrupt_flag.drive(true);
        }

        self.reg.old_pa7.drive(new_pa7);

        Ok(())
    }
}
