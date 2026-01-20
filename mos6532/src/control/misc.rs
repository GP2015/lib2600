use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn reset(&mut self) -> Result<(), RiotError> {
        self.reg.old_pa7.write(self.buf.pa.read_bit(7)?);

        self.ram.reset();

        self.buf.irq.reset();
        self.reg.ddra.write(0).unwrap();
        self.reg.ddrb.write(0).unwrap();
        self.reg.ora.write(0).unwrap();
        self.reg.orb.write(0).unwrap();

        self.reg.edc_enable_irq.write(false);
        self.reg.edc_use_pos_edge.write(false);

        Ok(())
    }

    pub(super) fn read_interrupt_flag(&mut self) -> Result<(), RiotError> {
        let edc_interrupt_flag_usize = self.reg.edc_interrupt_flag.read()? as usize;
        let timer_flag_usize = self.reg.timer_flag.read()? as usize;
        let interrupt_reg = (edc_interrupt_flag_usize << 7) | (timer_flag_usize << 6);

        self.buf.db.write(interrupt_reg).unwrap();

        self.reg.edc_interrupt_flag.write(false);

        Ok(())
    }
}
