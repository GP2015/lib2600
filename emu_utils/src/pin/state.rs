use crate::pin::PinSignal;

#[derive(Clone, Copy, Debug)]
pub struct PinState {
    high: bool,
    low: bool,
    tri_state: bool,
}

impl PinState {
    pub fn new(high: bool, low: bool, tri_state: bool) -> Self {
        Self {
            high,
            low,
            tri_state,
        }
    }

    pub fn set(&mut self, high: bool, low: bool, tri_state: bool) {
        (self.high, self.low, self.tri_state) = (high, low, tri_state);
    }

    pub fn set_high(&mut self, enable: bool) {
        self.high = enable;
    }

    pub fn set_low(&mut self, enable: bool) {
        self.low = enable;
    }

    pub fn set_tri_state(&mut self, enable: bool) {
        self.tri_state = enable;
    }

    pub fn iter_enabled(&self) -> impl Iterator<Item = PinSignal> {
        [
            (self.high, PinSignal::High),
            (self.low, PinSignal::Low),
            (self.tri_state, PinSignal::TriState),
        ]
        .into_iter()
        .filter_map(|(enabled, signal)| enabled.then_some(signal))
    }

    pub fn high(&self) -> bool {
        self.high
    }

    pub fn low(&self) -> bool {
        self.low
    }

    pub fn tri_state(&self) -> bool {
        self.tri_state
    }
}
