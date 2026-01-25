use crate::{InputPin, PinState, RiotError};

const BUS_SIZE: usize = 7;

pub struct AddressBus {
    pins: [InputPin; BUS_SIZE],
}

impl AddressBus {
    pub(crate) fn new() -> Self {
        Self {
            pins: [
                InputPin::new(String::from("A0")),
                InputPin::new(String::from("A1")),
                InputPin::new(String::from("A2")),
                InputPin::new(String::from("A3")),
                InputPin::new(String::from("A4")),
                InputPin::new(String::from("A5")),
                InputPin::new(String::from("A6")),
            ],
        }
    }

    fn get_bit_of_usize(val: usize, bit: usize) -> PinState {
        PinState::from_bool((val >> bit) & 1 == 1)
    }

    fn usize_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
        val >> bit_count != 0
    }

    fn get_low_bits_of_usize(val: usize, bit_count: usize) -> usize {
        val & ((1 << bit_count) - 1)
    }

    pub fn read(&self) -> Result<usize, RiotError> {
        let mut combined = 0;

        for bit in (0..BUS_SIZE).rev() {
            let val = self.pins[bit].read()?;
            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: String::from("A"),
                bit,
                size: BUS_SIZE,
            });
        }

        self.pins[bit].read()
    }

    pub fn state(&self) -> [Option<PinState>; BUS_SIZE] {
        [
            self.pins[0].state(),
            self.pins[1].state(),
            self.pins[2].state(),
            self.pins[3].state(),
            self.pins[4].state(),
            self.pins[5].state(),
            self.pins[6].state(),
        ]
    }

    pub fn bit_state(&self, bit: usize) -> Result<Option<PinState>, RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: String::from("A"),
                bit,
                size: BUS_SIZE,
            });
        }

        Ok(self.pins[bit].state())
    }

    pub fn drive_value_in(&mut self, val: usize) -> Result<(), RiotError> {
        if Self::usize_exceeds_bit_count(val, BUS_SIZE) {
            return Err(RiotError::BusDriveValueTooLarge {
                name: String::from("A"),
                value: val,
                size: BUS_SIZE,
            });
        }

        for bit in 0..BUS_SIZE {
            self.pins[bit].set_signal_in(Self::get_bit_of_usize(val, bit));
        }

        Ok(())
    }

    pub fn drive_value_in_wrapped(&mut self, val: usize) {
        self.drive_value_in(Self::get_low_bits_of_usize(val, BUS_SIZE))
            .unwrap();
    }

    pub fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: String::from("A"),
                bit,
                size: BUS_SIZE,
            });
        }

        self.pins[bit].set_signal_in(state);
        Ok(())
    }

    pub fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }
}
