use crate::{Riot, RiotLineRefs};

impl Riot {
    pub(crate) fn handle_reset(&mut self, lines: &mut RiotLineRefs, only_possible: bool) {
        self.ram.reset();

        self.ddra.add(0, only_possible).expect("valid value");
        self.ddrb.add(0, only_possible).expect("valid value");
        self.ora.add(0, only_possible).expect("valid value");
        self.orb.add(0, only_possible).expect("valid value");
        lines
            .pa
            .iter_mut(self.pa_con)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));
        lines
            .pb
            .iter_mut(self.pb_con)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));

        // self.edc_enable_irq.add(false, only_possible);
        // self.edc_use_pos_edge.add(false, only_possible);
    }
}
