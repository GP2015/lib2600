use crate::{Riot, RiotLineRefs};

impl Riot {
    pub(crate) fn call_reset(&mut self, lines: &mut RiotLineRefs, only_possible: bool) {
        self.ram.reset();

        self.ddra.add(0, only_possible).expect("must fit");
        self.ddrb.add(0, only_possible).expect("must fit");
        self.ora.add(0, only_possible).expect("must fit");
        self.orb.add(0, only_possible).expect("must fit");
        lines
            .pa
            .iter_mut(self.pa_con)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));
        lines
            .pb
            .iter_mut(self.pb_con)
            .for_each(|(line, connection)| line.add_high_z(connection, only_possible));

        self.edc_enables_irq.add(false, only_possible);
        self.edc_edge_type.add(false, only_possible);
        self.edc_interrupt_flag.set_all(true, true);
    }
}
