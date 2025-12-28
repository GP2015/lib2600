use crate::RIOT;

impl RIOT {
    pub fn update_phi2(&mut self, state: bool) {
        let prev_state = match self.pin.phi2.read() {
            Ok(val) => val,
            Err(_) => false,
        };

        self.pin.phi2.drive(state);

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
