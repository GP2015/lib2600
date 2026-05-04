use crate::{Riot, riot::lines::RiotOutputLines};

impl Riot {
    pub(crate) fn call_reset(&mut self, lines: &mut RiotOutputLines, only_possible: bool) {
        self.ram.reset();

        self.ddra.add(0, only_possible).expect("must fit");
        self.ddrb.add(0, only_possible).expect("must fit");
        self.ora.add(0, only_possible).expect("must fit");
        self.orb.add(0, only_possible).expect("must fit");
        lines.pa.add_high_z(self.con.pa, only_possible);
        lines.pb.add_high_z(self.con.pb, only_possible);

        self.edc_enables_irq.add(false, only_possible);
        self.edc_edge_type.add(false, only_possible);
        self.edc_interrupt_flag.set_all(true, true);
    }
}
