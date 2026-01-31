use crate::{
    Riot, RiotError,
    data::pins::bus::{Bus, BusOutput},
};

impl Riot {
    pub(super) fn reset(&mut self) -> Result<(), RiotError> {
        let old_pa7 = self.pa().read_bit(7)?;
        self.reg.old_pa7.write(old_pa7);

        self.ram.reset();

        self.reg.ddra.write(0);
        self.reg.ddrb.write(0);
        self.reg.ora.write(0);
        self.reg.orb.write(0);

        // self.irq_o().reset();
        self.reg.edc_enable_irq.write(false);
        self.reg.edc_use_pos_edge.write(false);

        Ok(())
    }

    pub(super) fn read_interrupt_flag(&mut self) -> Result<(), RiotError> {
        let edc_interrupt_flag_usize = self.reg.edc_interrupt_flag.read()? as usize;
        let timer_flag_usize = self.reg.timer_flag.read()? as usize;
        let interrupt_reg = (edc_interrupt_flag_usize << 7) | (timer_flag_usize << 6);

        self.db_o().drive_value_out(interrupt_reg)?;

        self.reg.edc_interrupt_flag.write(false);

        Ok(())
    }
}
