use crate::{ContentionPin, PinState, RiotError};

const BUS_SIZE: usize = 7;

pub struct ContentionByteBus {
    name: String,
    pins: [ContentionPin; 8],
}

impl ContentionByteBus {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            pins: [
                ContentionPin::new(format!("{}0", name.clone())),
                ContentionPin::new(format!("{}1", name.clone())),
                ContentionPin::new(format!("{}2", name.clone())),
                ContentionPin::new(format!("{}3", name.clone())),
                ContentionPin::new(format!("{}4", name.clone())),
                ContentionPin::new(format!("{}5", name.clone())),
                ContentionPin::new(format!("{}6", name.clone())),
                ContentionPin::new(format!("{}7", name)),
            ],
        }
    }

    fn get_bit_of_u8(val: u8, bit: usize) -> PinState {
        PinState::from_bool((val >> bit) & 1 == 1)
    }

    pub fn read(&self) -> Result<u8, RiotError> {
        let mut combined = 0;

        for bit in (0..BUS_SIZE).rev() {
            let val = self.pins[bit].read()?;
            combined <<= 1;
            combined |= val as u8;
        }

        Ok(combined)
    }

    pub fn read_bit(&self, bit: usize) -> Result<bool, RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
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
                name: self.name.clone(),
                bit,
                size: BUS_SIZE,
            });
        }

        Ok(self.pins[bit].state())
    }

    pub fn drive_value_in(&mut self, val: u8) -> Result<(), RiotError> {
        for bit in 0..BUS_SIZE {
            self.pins[bit].set_signal_in(Self::get_bit_of_u8(val, bit))?;
        }

        Ok(())
    }

    pub(crate) fn drive_value_out(&mut self, val: u8) -> Result<(), RiotError> {
        for bit in 0..BUS_SIZE {
            self.pins[bit].set_signal_out(Self::get_bit_of_u8(val, bit))?;
        }

        Ok(())
    }

    pub fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: BUS_SIZE,
            });
        }

        self.pins[bit].set_signal_in(state)
    }

    pub(crate) fn set_signal_out_bit(
        &mut self,
        bit: usize,
        state: PinState,
    ) -> Result<(), RiotError> {
        if bit >= BUS_SIZE {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: BUS_SIZE,
            });
        }

        self.pins[bit].set_signal_out(state)
    }

    pub fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }

    pub fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_out_bit(bit, PinState::from_bool(state))
    }
}
