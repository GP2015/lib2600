use crate::{Riot, pins::RiotLineRefs};

impl Riot {
    pub(crate) fn handle_reset(&mut self, lines: &mut RiotLineRefs, only_possible: bool) {
        self.ram.reset();

        self.reg.ddra.add(0, only_possible).expect("valid value");
        self.reg.ddrb.add(0, only_possible).expect("valid value");
        self.reg.ora.add(0, only_possible).expect("valid value");
        self.reg.orb.add(0, only_possible).expect("valid value");
        lines
            .pa
            .iter_mut(self.connections.pa)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));
        lines
            .pb
            .iter_mut(self.connections.pb)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));

        // self.reg.edc_enable_irq.add(false, only_possible);
        // self.reg.edc_use_pos_edge.add(false, only_possible);
    }
}
