use crate::RIOT;

impl RIOT {
    pub(crate) fn reset(&mut self) {
        self.buf.irq.reset();
        self.reg.ddra.drive(0).unwrap();
        self.reg.ddrb.drive(0).unwrap();
        self.reg.ora.drive(0).unwrap();
        self.reg.orb.drive(0).unwrap();
    }
}
