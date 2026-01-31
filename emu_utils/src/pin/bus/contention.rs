use crate::{
    bit,
    pin::{Bus, BusOutput, PinError, PinState, SinglePin, SinglePinOutput, single::SinglePinNew},
};

pub struct ContentionBus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T: SinglePinNew> ContentionBus<T> {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            size,
            pins: (0..size)
                .map(|bit| T::new(format!("{}{}", name, bit)))
                .collect(),
            name,
        }
    }
}

impl<T: SinglePin> Bus for ContentionBus<T> {
    type Error = T::Error;

    fn read(&self) -> Result<usize, Self::Error> {
        let mut combined = 0;

        for bit in (0..self.size).rev() {
            let val = self.pins[bit].read()?;
            combined <<= 1;
            combined |= val as usize;
        }

        Ok(combined)
    }

    fn read_bit(&self, bit: usize) -> Result<bool, Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        self.pins[bit].read()
    }

    fn state(&self) -> Vec<Option<PinState>> {
        self.pins.iter().map(|pin| pin.state()).collect()
    }

    fn bit_state(&self, bit: usize) -> Result<Option<PinState>, Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        Ok(self.pins[bit].state())
    }

    fn drive_value_in(&mut self, val: usize) -> Result<(), Self::Error> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(Self::Error::from(PinError::BusDriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            }));
        }

        for bit in 0..self.size {
            self.pins[bit].drive_in(bit::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn drive_value_in_wrapped(&mut self, val: usize) -> Result<(), Self::Error> {
        self.drive_value_in(bit::get_low_bits_of_usize(val, self.size))
    }

    fn tri_state_in(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].tri_state_in();
        }
    }

    fn set_signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        self.pins[bit].set_signal_in(state)
    }

    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error> {
        self.set_signal_in_bit(bit, PinState::from_bool(state))
    }

    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.set_signal_in_bit(bit, PinState::TriState)
    }
}

impl<T: SinglePinOutput> BusOutput for ContentionBus<T> {
    type Error = T::Error;

    fn drive_value_out(&mut self, val: usize) -> Result<(), Self::Error> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(Self::Error::from(PinError::BusDriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            }));
        }

        for bit in 0..self.size {
            self.pins[bit].drive_out(bit::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn tri_state_out(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].tri_state_out();
        }
    }

    fn set_signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BusPinOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        self.pins[bit].set_signal_out(state)?;
        Ok(())
    }

    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error> {
        self.set_signal_out_bit(bit, PinState::from_bool(state))
    }

    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.set_signal_out_bit(bit, PinState::TriState)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pin::bus::mock_pin::MockPin;
    use rstest::{fixture, rstest};

    type BusType = ContentionBus<MockPin<PinError>>;

    #[fixture]
    fn bus() -> BusType {
        ContentionBus::new(String::new(), 8)
    }

    type EmptyRes = Result<(), PinError>;
    type DriveValue = fn(&mut BusType, val: usize) -> EmptyRes;
    type TriState = fn(&mut BusType);
    type SetSignalBit = fn(&mut BusType, bit: usize, state: PinState) -> EmptyRes;
    type DriveBit = fn(&mut BusType, bit: usize, state: bool) -> EmptyRes;
    type TriStateBit = fn(&mut BusType, bit: usize) -> EmptyRes;

    #[rstest]
    fn drive_value_and_read(
        mut bus: BusType,
        #[values(ContentionBus::drive_value_in, ContentionBus::drive_value_out)] func: DriveValue,
    ) {
        func(&mut bus, 0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_value_large(
        mut bus: BusType,
        #[values(ContentionBus::drive_value_in, ContentionBus::drive_value_out)] func: DriveValue,
    ) {
        assert!(matches!(
            func(&mut bus, 0x167).err().unwrap(),
            PinError::BusDriveValueTooLarge { .. }
        ))
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0x167, 0x67)]
    fn drive_value_in_wrapped_and_read(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
        bus.drive_value_in_wrapped(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn tri_state(
        mut bus: BusType,
        #[values(ContentionBus::tri_state_in, ContentionBus::tri_state_out)] func: TriState,
    ) {
        func(&mut bus);
        assert_eq!(bus.state(), vec![Some(PinState::TriState); 8]);
    }

    #[rstest]
    fn set_signal_and_state_bit(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
        #[values(ContentionBus::set_signal_in_bit, ContentionBus::set_signal_out_bit)]
        func: SetSignalBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.bit_state(3).unwrap(), None);
        assert_eq!(bus.bit_state(4).unwrap(), Some(state));
    }

    #[rstest]
    fn set_signal_bit_out_of_range(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
        #[values(ContentionBus::set_signal_in_bit, ContentionBus::set_signal_out_bit)]
        func: SetSignalBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8, state).err().unwrap(),
            PinError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn bit_state_out_of_range(bus: BusType) {
        assert!(matches!(
            bus.bit_state(8).err().unwrap(),
            PinError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn drive_and_read_bit(
        mut bus: BusType,
        #[values(true, false)] state: bool,
        #[values(ContentionBus::drive_in_bit, ContentionBus::drive_out_bit)] func: DriveBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn drive_bit_out_of_range(
        mut bus: BusType,
        #[values(true, false)] state: bool,
        #[values(ContentionBus::drive_in_bit, ContentionBus::drive_out_bit)] func: DriveBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8, state).err().unwrap(),
            PinError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn read_bit_out_of_range(bus: BusType) {
        assert!(matches!(
            bus.read_bit(8).err().unwrap(),
            PinError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn tri_state_bit(mut bus: BusType) {
        bus.tri_state_in_bit(4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), Some(PinState::TriState));
    }

    #[rstest]
    fn tri_state_bit_out_of_range(
        mut bus: BusType,
        #[values(ContentionBus::tri_state_in_bit, ContentionBus::tri_state_out_bit)]
        func: TriStateBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8).err().unwrap(),
            PinError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn state(mut bus: BusType) {
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
                None,
                None,
            ]
        )
    }
}
