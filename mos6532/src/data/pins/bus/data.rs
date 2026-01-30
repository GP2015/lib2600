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

pub struct DataBus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T> DataBus<T>
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

impl<T> Bus for DataBus<T>
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
            self.pins[7].state(),
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
        if bit_utils::usize_exceeds_bit_count(val, self.size) {
            return Err(RiotError::BusDriveValueTooLarge {
                name: self.name.clone(),
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

    fn tri_state_in_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_in_bit(bit, PinState::TriState)
    }
}

impl<T> BusOutput for DataBus<T>
where
    T: SinglePinOutput,
{
    fn drive_value_out(&mut self, val: usize) -> Result<(), RiotError> {
        if bit_utils::usize_exceeds_bit_count(val, self.size) {
            return Err(RiotError::BusDriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            });
        }

        for bit in 0..self.size {
            self.pins[bit].drive_out(bit_utils::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn tri_state_out(&mut self) {
        for bit in 0..self.size {
            self.pins[bit].tri_state_out();
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

    fn tri_state_out_bit(&mut self, bit: usize) -> Result<(), RiotError> {
        self.set_signal_out_bit(bit, PinState::TriState)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::pins::bus::mock_pin::MockPin;
    use rstest::{fixture, rstest};

    #[fixture]
    fn bus() -> DataBus<MockPin> {
        DataBus::new(String::new())
    }

    type EmptyRes = Result<(), RiotError>;
    type DriveValue = fn(&mut DataBus<MockPin>, val: usize) -> EmptyRes;
    type TriState = fn(&mut DataBus<MockPin>);
    type SetSignalBit = fn(&mut DataBus<MockPin>, bit: usize, state: PinState) -> EmptyRes;
    type DriveBit = fn(&mut DataBus<MockPin>, bit: usize, state: bool) -> EmptyRes;
    type TriStateBit = fn(&mut DataBus<MockPin>, bit: usize) -> EmptyRes;

    #[rstest]
    fn drive_value_and_read(
        mut bus: DataBus<MockPin>,
        #[values(DataBus::drive_value_in, DataBus::drive_value_out)] func: DriveValue,
    ) {
        func(&mut bus, 0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_value_large(
        mut bus: DataBus<MockPin>,
        #[values(DataBus::drive_value_in, DataBus::drive_value_out)] func: DriveValue,
    ) {
        assert!(matches!(
            func(&mut bus, 0x167).err().unwrap(),
            RiotError::BusDriveValueTooLarge { .. }
        ))
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0x167, 0x67)]
    fn drive_value_in_wrapped_and_read(
        mut bus: DataBus<MockPin>,
        #[case] ival: usize,
        #[case] oval: usize,
    ) {
        bus.drive_value_in_wrapped(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn tri_state(
        mut bus: DataBus<MockPin>,
        #[values(DataBus::tri_state_in, DataBus::tri_state_out)] func: TriState,
    ) {
        func(&mut bus);
        assert_eq!(bus.state(), vec![Some(PinState::TriState); 8]);
    }

    #[rstest]
    fn set_signal_and_state_bit(
        mut bus: DataBus<MockPin>,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
        #[values(DataBus::set_signal_in_bit, DataBus::set_signal_out_bit)] func: SetSignalBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.bit_state(3).unwrap(), None);
        assert_eq!(bus.bit_state(4).unwrap(), Some(state));
    }

    #[rstest]
    fn set_signal_bit_out_of_range(
        mut bus: DataBus<MockPin>,
        #[values(PinState::High, PinState::Low, PinState::TriState)] state: PinState,
        #[values(DataBus::set_signal_in_bit, DataBus::set_signal_out_bit)] func: SetSignalBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8, state).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn bit_state_out_of_range(bus: DataBus<MockPin>) {
        assert!(matches!(
            bus.bit_state(8).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn drive_and_read_bit(
        mut bus: DataBus<MockPin>,
        #[values(true, false)] state: bool,
        #[values(DataBus::drive_in_bit, DataBus::drive_out_bit)] func: DriveBit,
    ) {
        func(&mut bus, 4, state).unwrap();
        assert_eq!(bus.read_bit(4).unwrap(), state);
    }

    #[rstest]
    fn drive_bit_out_of_range(
        mut bus: DataBus<MockPin>,
        #[values(true, false)] state: bool,
        #[values(DataBus::drive_in_bit, DataBus::drive_out_bit)] func: DriveBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8, state).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn read_bit_out_of_range(bus: DataBus<MockPin>) {
        assert!(matches!(
            bus.read_bit(8).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn tri_state_bit(mut bus: DataBus<MockPin>) {
        bus.tri_state_in_bit(4).unwrap();
        assert_eq!(bus.bit_state(4).unwrap(), Some(PinState::TriState));
    }

    #[rstest]
    fn tri_state_bit_out_of_range(
        mut bus: DataBus<MockPin>,
        #[values(DataBus::tri_state_in_bit, DataBus::tri_state_out_bit)] func: TriStateBit,
    ) {
        assert!(matches!(
            func(&mut bus, 8).err().unwrap(),
            RiotError::BusPinOutOfRange { .. }
        ))
    }

    #[rstest]
    fn state(mut bus: DataBus<MockPin>) {
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
