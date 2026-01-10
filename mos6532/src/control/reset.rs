use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn reset(&mut self) -> Result<(), RiotError> {
        self.reg.old_pa7.drive(self.buf.pa.read_bit(7)?);

        self.buf.irq.reset();
        self.reg.ddra.drive(0).unwrap();
        self.reg.ddrb.drive(0).unwrap();
        self.reg.ora.drive(0).unwrap();
        self.reg.orb.drive(0).unwrap();

        self.reg.edc_enable_irq.drive(false);
        self.reg.edc_use_pos_edge.drive(false);

        Ok(())
    }
}
