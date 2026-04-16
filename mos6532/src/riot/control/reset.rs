use emu_utils::pin::{BusCore, SinglePinOutput};

use crate::{Riot, RiotError};

impl Riot {
    pub(crate) fn handle_reset(&mut self, only_possible: bool) -> Result<(), RiotError> {
        self.ram.reset();

        self.reg.ddra.add(0, only_possible)?;
        self.reg.ddrb.add(0, only_possible)?;
        self.reg.ora.add(0, only_possible)?;
        self.reg.orb.add(0, only_possible)?;
        self.pin
            .pa
            .iter_mut()
            .for_each(|pin| pin.add_high_z_out(only_possible));
        self.pin
            .pb
            .iter_mut()
            .for_each(|pin| pin.add_high_z_out(only_possible));

        // self.reg.edc_enable_irq.add(false, only_possible);
        // self.reg.edc_use_pos_edge.add(false, only_possible);

        Ok(())
    }
}
