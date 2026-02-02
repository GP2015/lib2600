use crate::{
    bit,
    pin::{Bus, BusOutput, PinError, PinState, SinglePin, SinglePinOutput, single::SinglePinNew},
};

pub struct StandardBus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T> StandardBus<T> {
    fn pin<E: From<PinError>>(&self, bit: usize) -> Result<&T, E> {
        if bit >= self.size {
            return Err(E::from(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }
        Ok(&self.pins[bit])
    }

    fn pin_mut<E: From<PinError>>(&mut self, bit: usize) -> Result<&mut T, E> {
        if bit >= self.size {
            return Err(E::from(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            }));
        }
        Ok(&mut self.pins[bit])
    }
}

impl<T: SinglePinNew> StandardBus<T> {
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

impl<T: SinglePin> Bus for StandardBus<T> {
    type Error = T::Error;

    fn state(&self) -> Vec<PinState> {
        self.pins.iter().map(|pin| pin.state()).collect()
    }

    fn state_as_bool(&self) -> Vec<Option<bool>> {
        self.pins.iter().map(|pin| pin.state_as_bool()).collect()
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

    fn bit_state(&self, bit: usize) -> Result<PinState, Self::Error> {
        Ok(self.pin(bit)?.state())
    }

    fn bit_state_as_bool(&self, bit: usize) -> Result<Option<bool>, Self::Error> {
        Ok(self.pin(bit)?.state_as_bool())
    }

    fn read_bit(&self, bit: usize) -> Result<bool, Self::Error> {
        self.pin(bit)?.read()
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
        self.pins.iter_mut().for_each(|pin| pin.tri_state_in());
    }

    fn undefine_in(&mut self) -> Result<(), Self::Error> {
        for bit in 0..self.size {
            self.pins[bit].undefine_in()?;
        }
        Ok(())
    }

    fn signal_in_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error> {
        self.pin_mut(bit)?.signal_in(state)
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

impl<T: SinglePinOutput> BusOutput for StandardBus<T> {
    type Error = T::Error;

    fn drive_out(&mut self, val: usize) -> Result<(), Self::Error> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(Self::Error::from(PinError::DriveValueTooLarge {
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

    fn undefine_out(&mut self) -> Result<(), Self::Error> {
        for bit in 0..self.size {
            self.pins[bit].undefine_out()?;
        }
        Ok(())
    }

    fn signal_out_bit(&mut self, bit: usize, state: PinState) -> Result<(), Self::Error> {
        self.pin_mut(bit)?.signal_out(state)?;
        Ok(())
    }

    fn drive_out_bit(&mut self, bit: usize, state: bool) -> Result<(), Self::Error> {
        self.signal_out_bit(bit, PinState::from_bool(state))
    }

    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.signal_out_bit(bit, PinState::TriState)
    }

    fn undefine_out_bit(&mut self, bit: usize) -> Result<(), Self::Error> {
        self.signal_out_bit(bit, PinState::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use crate::pin::single::mock_pin::MockPin;

    use super::*;
    use rstest::{fixture, rstest};

    type BusType = StandardBus<MockPin<PinError>>;

    #[fixture]
    fn bus() -> BusType {
        StandardBus::new(String::new(), 8)
    }

    fn expect_out_of_range(err: Option<PinError>) {
        assert!(matches!(err.unwrap(), PinError::BitOutOfRange { .. }))
    }

    type EmptyRes = Result<(), PinError>;
    type DriveValue = fn(&mut BusType, val: usize) -> EmptyRes;
    type TriState = fn(&mut BusType);
    type Undefine = fn(&mut BusType) -> EmptyRes;
    type SetSignalBit = fn(&mut BusType, bit: usize, state: PinState) -> EmptyRes;
    type DriveBit = fn(&mut BusType, bit: usize, state: bool) -> EmptyRes;
    type TriStateBit = fn(&mut BusType, bit: usize) -> EmptyRes;
    type UndefineBit = fn(&mut BusType, bit: usize) -> EmptyRes;

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

    #[rstest]
    fn state_as_bool(mut bus: BusType) {
        bus.signal_in_bit(2, PinState::High).unwrap();
        bus.signal_in_bit(3, PinState::Low).unwrap();
        bus.signal_in_bit(4, PinState::TriState).unwrap();
        bus.signal_in_bit(5, PinState::Undefined).unwrap();

        assert_eq!(
            bus.state_as_bool(),
            vec![None, None, Some(true), Some(false), None, None, None, None,]
        )
    }

    #[rstest]
    fn read(mut bus: BusType) {
        bus.drive_in(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn bit_state(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        bus.signal_in_bit(4, state).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), state);
    }

    #[rstest]
    fn bit_state_out_of_range(bus: BusType) {
        expect_out_of_range(bus.bit_state(8).err());
    }

    #[rstest]
    fn bit_state_as_bool(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
    ) {
        bus.signal_in_bit(4, state).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), state);
    }

    #[rstest]
    fn bit_state_as_bool_out_of_range(bus: BusType) {
        expect_out_of_range(bus.bit_state_as_bool(8).err());
    }

    #[rstest]
    fn read_bit(mut bus: BusType, #[values(true, false)] state: bool) {
        bus.drive_in_bit(4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn read_bit_out_of_range(bus: BusType) {
        expect_out_of_range(bus.read_bit(8).err());
    }

    #[rstest]
    fn drive(
        mut bus: BusType,
        #[values(StandardBus::drive_in, StandardBus::drive_out)] func: DriveValue,
    ) {
        func(&mut bus, 0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_too_large(
        mut bus: BusType,
        #[values(StandardBus::drive_in, StandardBus::drive_out)] func: DriveValue,
    ) {
        assert!(matches!(
            func(&mut bus, 0x167).err().unwrap(),
            PinError::DriveValueTooLarge { .. }
        ))
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0x167, 0x67)]
    fn wrapping_drive_in(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
        bus.wrapping_drive_in(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn tri_state(
        mut bus: BusType,
        #[values(StandardBus::tri_state_in, StandardBus::tri_state_out)] func: TriState,
    ) {
        func(&mut bus);
        assert_eq!(bus.state(), vec![PinState::TriState; 8]);
    }

    #[rstest]
    fn undefine(
        mut bus: BusType,
        #[values(StandardBus::undefine_in, StandardBus::undefine_out)] func: Undefine,
    ) {
        func(&mut bus).unwrap();
        assert_eq!(bus.state(), vec![PinState::Undefined; 8]);
    }

    #[rstest]
    fn signal_bit(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
        #[values(StandardBus::signal_in_bit, StandardBus::signal_out_bit)] func: SetSignalBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), state);
    }

    #[rstest]
    fn signal_bit_out_of_range(
        mut bus: BusType,
        #[values(PinState::High, PinState::Low, PinState::TriState, PinState::Undefined)]
        state: PinState,
        #[values(StandardBus::signal_in_bit, StandardBus::signal_out_bit)] func: SetSignalBit,
    ) {
        expect_out_of_range(func(&mut bus, 8, state).err());
    }

    #[rstest]
    fn drive_bit(
        mut bus: BusType,
        #[values(true, false)] state: bool,
        #[values(StandardBus::drive_in_bit, StandardBus::drive_out_bit)] func: DriveBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn drive_bit_out_of_range(
        mut bus: BusType,
        #[values(true, false)] state: bool,
        #[values(StandardBus::drive_in_bit, StandardBus::drive_out_bit)] func: DriveBit,
    ) {
        expect_out_of_range(func(&mut bus, 8, state).err());
    }

    #[rstest]
    fn tri_state_bit(
        mut bus: BusType,
        #[values(StandardBus::tri_state_in_bit, StandardBus::tri_state_out_bit)] func: TriStateBit,
    ) {
        func(&mut bus, 4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), PinState::TriState);
    }

    #[rstest]
    fn tri_state_bit_out_of_range(
        mut bus: BusType,
        #[values(StandardBus::tri_state_in_bit, StandardBus::tri_state_out_bit)] func: TriStateBit,
    ) {
        expect_out_of_range(func(&mut bus, 8).err());
    }

    #[rstest]
    fn undefine_bit(
        mut bus: BusType,
        #[values(StandardBus::undefine_in_bit, StandardBus::undefine_out_bit)] func: UndefineBit,
    ) {
        func(&mut bus, 4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), PinState::Undefined);
    }

    #[rstest]
    fn undefine_bit_out_of_range(
        mut bus: BusType,
        #[values(StandardBus::undefine_in_bit, StandardBus::undefine_out_bit)] func: TriStateBit,
    ) {
        expect_out_of_range(func(&mut bus, 8).err());
    }
}
