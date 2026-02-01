use crate::{
    bit,
    pin::{Bus, PinError, PinState, SinglePin, single::SinglePinNew},
};

pub struct InputBus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T: SinglePinNew> InputBus<T> {
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

impl<T: SinglePin> Bus for InputBus<T> {
    type Error = T::Error;

    fn state(&self) -> Vec<PinState> {
        self.pins.iter().map(|pin| pin.state()).collect()
    }

    fn bit_state(&self, bit: usize) -> Result<PinState, Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        Ok(self.pins[bit].state())
    }

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
            return Err(Self::Error::from(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        self.pins[bit].read()
    }

    fn drive_in(&mut self, val: usize) -> Result<(), Self::Error> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(Self::Error::from(PinError::DriveValueTooLarge {
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

    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), Self::Error> {
        self.drive_in(bit::get_low_bits_of_usize(val, self.size))
    }

    fn tri_state_in(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].tri_state_in();
        }
    }

    fn undefine_in(&mut self) -> Result<(), Self::Error> {
        for bit in 0..self.size {
            self.pins[bit].undefine_in()?;
        }
        Ok(())
    }

    fn signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error> {
        if bit >= self.size {
            return Err(Self::Error::from(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }

        self.pins[bit].signal_in(state)
    }

    fn drive_in_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error> {
        self.signal_in_bit(bit, PinState::from_bool(state))
    }

    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.signal_in_bit(bit, PinState::TriState)
    }

    fn undefine_in_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.signal_in_bit(bit, PinState::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pin::bus::mock_pin::MockPin;
    use rstest::{fixture, rstest};

    type BusType = InputBus<MockPin<PinError>>;

    #[fixture]
    fn bus() -> BusType {
        InputBus::new(String::new(), 8)
    }

    #[rstest]
    fn drive_and_read(mut bus: BusType) {
        bus.drive_in(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_large(mut bus: BusType) {
        assert!(matches!(
            bus.drive_in(0x167).err().unwrap(),
            PinError::DriveValueTooLarge { .. }
        ))
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0x167, 0x67)]
    fn wrapping_drive_and_read(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
        bus.wrapping_drive_in(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn tri_state(mut bus: BusType) {
        bus.tri_state_in();
        assert_eq!(bus.state(), vec![PinState::TriState; 8]);
    }

    #[rstest]
    fn undefine(mut bus: BusType) {
        bus.undefine_in().unwrap();
        assert_eq!(bus.state(), vec![PinState::Undefined; 8]);
    }

    #[rstest]
    fn signal_and_state_bit(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        bus.signal_in_bit(4, state).unwrap();
        assert_eq!(bus.bit_state(3).unwrap(), PinState::Undefined);
        assert_eq!(bus.bit_state(4).unwrap(), state);
    }

    #[rstest]
    fn signal_bit_out_of_range(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        assert!(matches!(
            bus.signal_in_bit(8, state).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn bit_state_out_of_range(bus: BusType) {
        assert!(matches!(
            bus.bit_state(8).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn drive_and_read_bit(mut bus: BusType, #[values(true, false)] state: bool) {
        bus.drive_in_bit(4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn drive_bit_out_of_range(mut bus: BusType, #[values(true, false)] state: bool) {
        assert!(matches!(
            bus.drive_in_bit(8, state).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn read_bit_out_of_range(bus: BusType) {
        assert!(matches!(
            bus.read_bit(8).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn tri_state_bit(mut bus: BusType) {
        bus.tri_state_in_bit(4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), PinState::TriState);
    }

    #[rstest]
    fn tri_state_bit_out_of_range(mut bus: BusType) {
        assert!(matches!(
            bus.tri_state_in_bit(8).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn undefine_bit(mut bus: BusType) {
        bus.undefine_in_bit(4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), PinState::Undefined);
    }

    #[rstest]
    fn undefine_bit_out_of_range(mut bus: BusType) {
        assert!(matches!(
            bus.undefine_in_bit(8).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ))
    }

    #[rstest]
    fn state(mut bus: BusType) {
        bus.signal_in_bit(2, PinState::High).unwrap();
        bus.signal_in_bit(3, PinState::Low).unwrap();
        bus.signal_in_bit(4, PinState::TriState).unwrap();
        bus.signal_in_bit(5, PinState::Undefined).unwrap();

        assert_eq!(
            bus.state(),
            vec![
                PinState::Undefined,
                PinState::Undefined,
                PinState::High,
                PinState::Low,
                PinState::TriState,
                PinState::Undefined,
                PinState::Undefined,
                PinState::Undefined,
            ]
        )
    }
}
