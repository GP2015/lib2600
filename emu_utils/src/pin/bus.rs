use crate::{
    bit,
    pin::{PinError, PinState, SinglePinInput, SinglePinOutput, single::SinglePinSetup},
};

pub struct Bus<T> {
    name: String,
    size: usize,
    pins: Vec<T>,
}

impl<T> Bus<T> {
    fn bit_out_of_range_check(&self, bit: usize) -> Result<(), PinError> {
        if bit >= self.size {
            Err(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            })
        } else {
            Ok(())
        }
    }
}

impl<O, T: SinglePinSetup<O>> Bus<T> {
    fn new(name: String, size: usize) -> Self {
        Self {
            size,
            pins: (0..size)
                .map(|bit| T::new(format!("{}{}", name, bit)))
                .collect(),
            name,
        }
    }

    fn pin_setup(&self, bit: usize) -> Result<&impl SinglePinSetup<O>, PinError> {
        self.bit_out_of_range_check(bit)?;
        Ok(&self.pins[bit])
    }

    fn pin_setup_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinSetup<O>, PinError> {
        self.bit_out_of_range_check(bit)?;
        Ok(&mut self.pins[bit])
    }
}

impl<T: SinglePinInput> Bus<T> {
    fn pin(&self, bit: usize) -> Result<&impl SinglePinInput, PinError> {
        self.bit_out_of_range_check(bit)?;
        Ok(&self.pins[bit])
    }

    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinInput, PinError> {
        self.bit_out_of_range_check(bit)?;
        Ok(&mut self.pins[bit])
    }

    fn state(&self) -> Vec<PinState> {
        self.pins.iter().map(|pin| pin.state()).collect()
    }

    fn state_as_bool(&self) -> Vec<Option<bool>> {
        self.pins.iter().map(|pin| pin.state_as_bool()).collect()
    }

    fn read(&self) -> Result<usize, PinError> {
        let mut combined = 0;
        for bit in (0..self.size).rev() {
            let val = self.pins[bit].read()?;
            combined <<= 1;
            combined |= val as usize;
        }
        Ok(combined)
    }

    fn drive_in(&mut self, val: usize) -> Result<(), PinError> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(PinError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            });
        }

        for bit in 0..self.size {
            self.pins[bit].drive_in(bit::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn wrapping_drive_in(&mut self, val: usize) -> Result<(), PinError> {
        self.drive_in(bit::get_low_bits_of_usize(val, self.size))
    }

    fn tri_state_in(&mut self) -> Result<(), PinError> {
        for bit in 0..self.size {
            self.pins[bit].tri_state_in()?;
        }
        Ok(())
    }

    fn undefine_in(&mut self) -> Result<(), PinError> {
        for bit in 0..self.size {
            self.pins[bit].undefine_in()?;
        }
        Ok(())
    }
}

impl<T: SinglePinOutput> Bus<T> {
    fn pin_out(&self, bit: usize) -> Result<&impl SinglePinOutput, PinError> {
        if bit >= self.size {
            return Err(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }
        Ok(&self.pins[bit])
    }

    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl SinglePinOutput, PinError> {
        if bit >= self.size {
            return Err(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.size,
            });
        }
        Ok(&mut self.pins[bit])
    }

    fn drive_out(&mut self, val: usize) -> Result<(), PinError> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            return Err(PinError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            });
        }

        for bit in 0..self.size {
            self.pins[bit].drive_out(bit::get_bit_of_usize(val, bit))?;
        }

        Ok(())
    }

    fn tri_state_out(&mut self) -> Result<(), PinError> {
        for bit in 0..self.size {
            self.pins[bit].tri_state_out()?;
        }
        Ok(())
    }

    fn undefine_out(&mut self) -> Result<(), PinError> {
        for bit in 0..self.size {
            self.pins[bit].undefine_out()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::pin::single::mock_pin::MockPin;

    use super::*;
    use rstest::{fixture, rstest};

    type BusType = Bus<MockPin<PinError>>;

    #[fixture]
    fn bus() -> BusType {
        Bus::new(String::new(), 8)
    }

    type EmptyRes = Result<(), PinError>;
    type DriveValue = fn(&mut BusType, val: usize) -> EmptyRes;
    type TriState = fn(&mut BusType) -> EmptyRes;
    type Undefine = fn(&mut BusType) -> EmptyRes;

    #[rstest]
    fn state(mut bus: BusType) {
        let states = [
            PinState::Undefined,
            PinState::Undefined,
            PinState::High,
            PinState::Low,
            PinState::TriState,
            PinState::Undefined,
            PinState::Undefined,
            PinState::Undefined,
        ];

        for (i, state) in states.iter().enumerate() {
            bus.pin_mut(i).unwrap().signal_in(*state).unwrap();
        }

        assert_eq!(bus.state(), states);
    }

    #[rstest]
    fn state_as_bool(mut bus: BusType) {
        let states = [
            PinState::Undefined,
            PinState::Undefined,
            PinState::High,
            PinState::Low,
            PinState::TriState,
            PinState::Undefined,
            PinState::Undefined,
            PinState::Undefined,
        ];

        for (i, state) in states.iter().enumerate() {
            bus.pin_mut(i).unwrap().signal_in(*state).unwrap();
        }

        for (i, state) in bus.state_as_bool().iter().enumerate() {
            assert_eq!(*state, PinState::as_bool(&states[i]));
        }
    }

    #[rstest]
    fn read(mut bus: BusType) {
        bus.drive_in(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive(mut bus: BusType, #[values(Bus::drive_in, Bus::drive_out)] func: DriveValue) {
        func(&mut bus, 0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_too_large(
        mut bus: BusType,
        #[values(Bus::drive_in, Bus::drive_out)] func: DriveValue,
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
        #[values(Bus::tri_state_in, Bus::tri_state_out)] func: TriState,
    ) {
        func(&mut bus).unwrap();
        assert_eq!(bus.state(), vec![PinState::TriState; 8]);
    }

    #[rstest]
    fn undefine(mut bus: BusType, #[values(Bus::undefine_in, Bus::undefine_out)] func: Undefine) {
        func(&mut bus).unwrap();
        assert_eq!(bus.state(), vec![PinState::Undefined; 8]);
    }
}
