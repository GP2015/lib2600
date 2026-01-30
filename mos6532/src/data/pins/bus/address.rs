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
            self.pins[bit].drive_in(bit_utils::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), RiotError> {
        self.drive_value_in(bit_utils::get_low_bits_of_usize(val, self.size))
    }

    fn tri_state_in(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].tri_state_in();
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

        self.pins[bit].set_signal_in(state)
    }

    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }

    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::TriState)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::pins::bus::mock_pin::MockPin;
    use rstest::{fixture, rstest};

    #[fixture]
    fn bus() -> AddressBus<MockPin> {
        AddressBus::new()
    }

    #[rstest]
    fn drive_value_and_read(mut bus: AddressBus<MockPin>) {
        bus.drive_value_in(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_value_large(mut bus: AddressBus<MockPin>) {
        assert!(matches!(
            bus.drive_value_in(0x87).err().unwrap(),
            RiotError::BusDriveValueTooLarge { .. }
        ))
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0xE7, 0x67)]
    fn drive_value_wrapped_and_read(
        mut bus: AddressBus<MockPin>,
        #[case] ival: usize,
        #[case] oval: usize,
    ) {
        bus.drive_value_in_wrapped(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn tri_state(mut bus: AddressBus<MockPin>) {
        bus.tri_state_in();
        assert_eq!(bus.state(), vec![Some(PinState::TriState); 7]);
    }

    #[rstest]
    fn set_signal_and_state_bit(
        mut bus: AddressBus<MockPin>,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
    ) {
        bus.set_signal_in_bit(4, state).unwrap();
        assert_eq!(bus.bit_state(3).unwrap(), None);
        assert_eq!(bus.bit_state(4).unwrap(), Some(state));
    }

    #[rstest]
    fn set_signal_bit_out_of_range(
        mut bus: AddressBus<MockPin>,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
    ) {
        assert!(matches!(
            bus.set_signal_in_bit(7, state).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn bit_state_out_of_range(bus: AddressBus<MockPin>) {
        assert!(matches!(
            bus.bit_state(7).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn drive_and_read_bit(mut bus: AddressBus<MockPin>, #[values(true, false)] state: bool) {
        bus.drive_in_bit(4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn drive_bit_out_of_range(mut bus: AddressBus<MockPin>, #[values(true, false)] state: bool) {
        assert!(matches!(
            bus.drive_in_bit(7, state).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn read_bit_out_of_range(bus: AddressBus<MockPin>) {
        assert!(matches!(
            bus.read_bit(7).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn tri_state_bit(mut bus: AddressBus<MockPin>) {
        bus.tri_state_in_bit(4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), Some(PinState::TriState));
    }

    #[rstest]
    fn tri_state_bit_out_of_range(mut bus: AddressBus<MockPin>) {
        assert!(matches!(
            bus.tri_state_in_bit(7).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn state(mut bus: AddressBus<MockPin>) {
        bus.set_signal_in_bit(2, PinState::High).unwrap();
        bus.set_signal_in_bit(3, PinState::Low).unwrap();
        bus.set_signal_in_bit(4, PinState::TriState).unwrap();

        assert_eq!(
            bus.state(),
            vec![
                None,
                None,
                Some(PinState::High),
                Some(PinState::Low),
                Some(PinState::TriState),
                None,
                None
            ]
        )
    }
}
