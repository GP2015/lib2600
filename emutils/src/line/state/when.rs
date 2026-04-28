use crate::line::{PinSignal, state::now::DriveState};
use delegate::delegate;
use getset::CopyGetters;

#[derive(Clone, Copy, Debug, PartialEq, CopyGetters)]
pub struct DriveStateWhen {
    #[get_copy = "pub(crate)"]
    prev_state: DriveState,
    #[get_copy = "pub(crate)"]
    state: DriveState,
}

impl Default for DriveStateWhen {
    fn default() -> Self {
        Self {
            prev_state: DriveState::from(false, false, true),
            state: DriveState::from(false, false, false),
        }
    }
}

impl DriveStateWhen {
    pub fn select(&self, prev: bool) -> &DriveState {
        if prev { &self.prev_state } else { &self.state }
    }

    pub fn copy_from_prev(&mut self) {
        self.state = self.prev_state;
    }

    pub fn copy_from_drive_state(&mut self, state: DriveState) {
        self.state = state;
    }

    delegate! {
        to self.state{
            pub fn is_possible(self, signal: PinSignal) -> bool;
            pub fn high_possible(self) -> bool;
            pub fn low_possible(self) -> bool;
            pub fn high_z_possible(self) -> bool;

            pub fn could_read_high(self) -> bool;
            pub fn could_read_low(self) -> bool;

            pub fn collapsed(self) -> Option<PinSignal>;
            pub fn read(self) -> Option<bool>;
            pub fn iter_possible(self) -> impl Iterator<Item = PinSignal>;
            pub fn possible_reads(self) -> &'static [bool];

            pub fn add(&mut self, signal: PinSignal, only_possible: bool);
            pub fn remove(&mut self, signal: PinSignal);

            pub fn add_high(&mut self, only_possible: bool);
            pub fn add_low(&mut self, only_possible: bool);
            pub fn add_high_z(&mut self, only_possible: bool);

            pub fn remove_high(&mut self);
            pub fn remove_low(&mut self);
            pub fn remove_high_z(&mut self);

            pub fn add_drive(&mut self, val: bool, only_possible: bool);
            pub fn remove_drive(&mut self, val: bool);
            pub fn set_all(&mut self, high: bool, low: bool, high_z: bool);
        }

        to self.prev_state{
            #[call(is_possible)]
            pub fn is_prev_possible(self, signal: PinSignal) -> bool;

            #[call(high_possible)]
            pub fn prev_high_possible(self) -> bool;
            #[call(low_possible)]
            pub fn prev_low_possible(self) -> bool;
            #[call(high_z_possible)]
            pub fn prev_high_z_possible(self) -> bool;

            #[call(could_read_high)]
            pub fn prev_could_read_high(self) -> bool;
            #[call(could_read_low)]
            pub fn prev_could_read_low(self) -> bool;

            #[call(collapsed)]
            pub fn prev_collapsed(self) -> Option<PinSignal>;
            #[call(read)]
            pub fn read_prev(self) -> Option<bool>;
            #[call(iter_possible)]
            pub fn iter_prev_possible(self) -> impl Iterator<Item = PinSignal>;
            #[call(possible_reads)]
            pub fn prev_possible_reads(self) -> &'static [bool];
        }

        to |prev: bool| self.select(prev){
            #[call(is_possible)]
            pub fn is_possible_when(self, signal: PinSignal) -> bool;

            #[call(high_possible)]
            pub fn high_possible_when(self) -> bool;
            #[call(low_possible)]
            pub fn low_possible_when(self) -> bool;
            #[call(high_z_possible)]
            pub fn high_z_possible_when(self) -> bool;

            #[call(could_read_high)]
            pub fn could_read_high_when(self) -> bool;
            #[call(could_read_low)]
            pub fn could_read_low_when(self) -> bool;

            #[call(collapsed)]
            pub fn collapsed_when(self) -> Option<PinSignal>;
            #[call(read)]
            pub fn read_when(self) -> Option<bool>;
            #[call(iter_possible)]
            pub fn iter_possible_when(self) -> impl Iterator<Item = PinSignal>;
            #[call(possible_reads)]
            pub fn possible_reads_when(self) -> &'static [bool];
        }
    }
}
