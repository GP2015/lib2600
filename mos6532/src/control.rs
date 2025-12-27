use crate::RIOT;

impl RIOT {
    pub fn update_phi2(&mut self, state: bool) {
        let prev_state = self.pin.phi2;
        self.pin.phi2 = state;

        match (prev_state, state) {
            (false, true) => self.phi2_rise(),
            (true, false) => self.phi2_fall(),
            _ => (),
        }
    }

    fn phi2_rise(&mut self) {
        //
    }

    fn phi2_fall(&mut self) {
        //
    }
}
