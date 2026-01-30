use crate::{
    PinState, RiotError,
    data::{
        bit_utils,
        pins::{
            bus::Bus,
            single::{SinglePin, SinglePinNew},
        },
    },
};

pub struct AddressBus<T> {
    size: usize,
    pins: Vec<T>,
}

impl<T> AddressBus<T>
where
    T: SinglePinNew,
{
    pub(crate) fn new() -> Self {
        Self {
            size: 7,
            pins: vec![
                T::new(String::from("A0")),
                T::new(String::from("A1")),
                T::new(String::from("A2")),
                T::new(String::from("A3")),
                T::new(String::from("A4")),
                T::new(String::from("A5")),
                T::new(String::from("A6")),
            ],
        }
    }
}

impl<T> Bus for AddressBus<T>
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
                name: String::from("A"),
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
                name: String::from("A"),
                bit,
                size: self.size,
            });
        }

        Ok(self.pins[bit].state())
    }

    fn drive_value_in(&mut self, val: usize) -> Result<(), RiotError> {
        if bit_utils::usize_exceeds_bit_count(val, self.size) {
            return Err(RiotError::BusDriveValueTooLarge {
                name: String::from("A"),
                value: val,
                size: self.size,
            });
        }

        for bit in 0..self.size {
            let b = bit_utils::get_bit_of_usize(val, bit);
            let signal = PinState::from_bool(b);
            self.pins[bit].set_signal_in(signal);
        }

        Ok(())
    }

    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), RiotError> {
        self.drive_value_in(bit_utils::get_low_bits_of_usize(val, self.size))
    }

    fn tristate_in(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].set_signal_in(PinState::TriState);
        }
    }

    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), RiotError> {
        if bit >= self.size {
            return Err(RiotError::BusPinOutOfRange {
                name: String::from("A"),
                bit,
                size: self.size,
            });
        }

        self.pins[bit].set_signal_in(state);
        Ok(())
    }

    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }

    fn tristate_in_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::TriState)
    }
}
