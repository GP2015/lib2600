use crate::{
    PinState, RiotError,
    data::{
        bit_utils,
        pins::{
            bus::{Bus, BusOutput},
            single::{SinglePin, SinglePinNew, SinglePinOutput},
        },
    },
};

pub struct ContentionByteBus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T> ContentionByteBus<T>
where
    T: SinglePinNew,
{
    pub(crate) fn new(name: String) -> Self {
        Self {
            name: name.clone(),
            size: 8,
            pins: vec![
                T::new(format!("{}0", name.clone())),
                T::new(format!("{}1", name.clone())),
                T::new(format!("{}2", name.clone())),
                T::new(format!("{}3", name.clone())),
                T::new(format!("{}4", name.clone())),
                T::new(format!("{}5", name.clone())),
                T::new(format!("{}6", name.clone())),
                T::new(format!("{}7", name)),
            ],
        }
    }
}

impl<T> Bus for ContentionByteBus<T>
where
    T: SinglePin,
{
    fn read(&self) -> Result<usize, RiotError> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let val = self.pins[bit].read()?;
            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    fn read_bit(&self, bit: usize) -> Result<bool, RiotError> {
        if bit >= self.size {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.pins[bit].read()
    }

    fn state(&self) -> Vec<Option<PinState>> {
        vec![
            self.pins[0].state(),
            self.pins[1].state(),
            self.pins[2].state(),
            self.pins[3].state(),
            self.pins[4].state(),
            self.pins[5].state(),
            self.pins[6].state(),
        ]
    }

    fn bit_state(&self, bit: usize) -> Result<Option<PinState>, RiotError> {
        if bit >= self.size {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        Ok(self.pins[bit].state())
    }

    fn drive_value_in(&mut self, val: usize) -> Result<(), RiotError> {
        for bit in 0..self.size {
            let b = bit_utils::get_bit_of_usize(val, bit);
            let signal = PinState::from_bool(b);
            self.pins[bit].set_signal_in(signal)?;
        }

        Ok(())
    }

    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), RiotError> {
        self.drive_value_in(bit_utils::get_low_bits_of_usize(val, self.size))
    }

    fn tristate_in(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].set_signal_in(PinState::TriState).unwrap();
        }
    }

    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError> {
        if bit >= self.size {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.pins[bit].set_signal_in(state)
    }

    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }

    fn tristate_in_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::TriState)
    }
}

impl<T> BusOutput for ContentionByteBus<T>
where
    T: SinglePinOutput,
{
    fn drive_value_out(&mut self, val: usize) -> Result<(), RiotError> {
        for bit in 0..self.size {
            let b = bit_utils::get_bit_of_usize(val, bit);
            let signal = PinState::from_bool(b);
            self.pins[bit].set_signal_out(signal)?;
        }

        Ok(())
    }

    fn tristate_out(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].set_signal_out(PinState::TriState).unwrap();
        }
    }

    fn set_signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError> {
        if bit >= self.size {
            return Err(RiotError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }

        self.pins[bit].set_signal_out(state)
    }

    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_out_bit(bit, PinState::from_bool(state))
    }

    fn tristate_out_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_out_bit(bit, PinState::TriState)
    }
}
