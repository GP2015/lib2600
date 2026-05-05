use crate::{Riot, riot::states::RiotLineStates};

impl Riot {
    pub(crate) const fn update_edc(&mut self, states: &RiotLineStates) {
        let new_pa7_state = states.pa.line_state::<7>();

        let (old_pa7_low, old_pa7_high) = self.old_pa7_state.could_read_low_high();
        let (new_pa7_low, new_pa7_high) = new_pa7_state.could_read_low_high();
        let (edc_type_low, edc_type_high) = self.edc_edge_type.low_high_possible();

        if edc_type_high && old_pa7_low && new_pa7_high {
            let only_transition = !edc_type_low && !old_pa7_high && !new_pa7_low;
            self.edc_ir_flag.add(true, only_transition);
        }

        if edc_type_low && old_pa7_high && new_pa7_low {
            let only_transition = !edc_type_high && !old_pa7_low && !new_pa7_high;
            self.edc_ir_flag.add(true, only_transition);
        }

        self.old_pa7_state = new_pa7_state;
    }

    pub(crate) const fn write_edc(&mut self, states: &RiotLineStates, only_instruction: bool) {
        let a0 = states.a.line_state::<0>();
        let a1 = states.a.line_state::<1>();

        if a1.could_read_low() {
            self.edc_enables_irq.add(false, only_instruction);
        }

        if a1.could_read_high() {
            self.edc_enables_irq.add(true, only_instruction);
        }

        if a0.could_read_low() {
            self.edc_edge_type.add(false, only_instruction);
        }

        if a0.could_read_high() {
            self.edc_edge_type.add(true, only_instruction);
        }
    }
}
