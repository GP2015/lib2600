use crate::{Riot, riot::lines::RiotLineStates};

impl Riot {
    pub(crate) const fn write_edc(&mut self, states: &RiotLineStates, only_instruction: bool) {
        let a0 = states.a.line_state::<0>();

        if a0.could_read_low() {
            self.edc_edge_type.add(false, only_instruction);
        }

        if a0.could_read_high() {
            self.edc_edge_type.add(true, only_instruction);
        }
    }
}
