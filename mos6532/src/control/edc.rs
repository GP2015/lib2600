use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn write_edc(&mut self) -> Result<(), RiotError> {
        self.reg.edc_enable_irq.write(self.buf.a.read_bit(1)?);
        self.reg.edc_use_pos_edge.write(self.buf.a.read_bit(0)?);
        Ok(())
    }

    pub(super) fn update_edc(&mut self) -> Result<(), RiotError> {
        let new_pa7 = self.buf.pa.read_bit(7)?;

        if new_pa7 != self.reg.old_pa7.read()? && new_pa7 == self.reg.edc_use_pos_edge.read()? {
            self.reg.edc_interrupt_flag.write(true);
        }

        self.reg.old_pa7.write(new_pa7);

        Ok(())
    }
}
