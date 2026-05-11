use crate::{
    common::line::{bus::Bus, error::LineError},
    riot::core::{Riot, states::RiotStates},
};

impl Riot {
    // Only instruction that writes to edc_edge_type
    pub fn write_edc(
        &mut self,
        db: &mut Bus<8>,
        states: &RiotStates,
        only_possible: bool,
    ) -> Result<(), LineError> {
        if only_possible {
            self.reg.edc_edge_type.remove_all();
        }

        for &edc_state in states.a.line_state::<0>().possible_reads() {
            self.reg.edc_edge_type.add(edc_state);
        }

        db.add_high_z(self.db_con)
    }
}
