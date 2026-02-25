use crate::{
    bit,
    pin::{BusCore, BusInterface, BusOutput, PinError, PinSignal, SinglePinCore, SinglePinOutput},
};

pub struct StandardBus<P> {
    name: String,
    size: usize,
    pins: Vec<P>,
}

impl<P> StandardBus<P> {
    fn check_for_bit_out_of_range(&self, bit: usize) -> Result<(), PinError> {
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

    fn check_if_drive_val_too_large(&self, val: usize) -> Result<(), PinError> {
        if bit::usize_exceeds_bit_count(val, self.size) {
            Err(PinError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.size,
            })
        } else {
            Ok(())
        }
    }

    fn collapsed_as_usize(&self, collapsed: Vec<Option<PinSignal>>) -> Option<usize> {
        let mut combined = 0;
        for bit in (0..self.size).rev() {
            let b = collapsed[bit]?.as_bool()?;
            combined <<= 1;
            combined |= b as usize;
        }
        Some(combined)
    }
}

impl<P: SinglePinCore> BusCore<P> for StandardBus<P> {
    fn new(name: String, size: usize) -> Self {
        Self {
            size,
            pins: (0..size)
                .map(|bit| P::new(format!("{}{}", name, bit)))
                .collect(),
            name,
        }
    }

    fn post_tick_update(&mut self) {
        self.pins.iter_mut().for_each(|pin| pin.post_tick_update());
    }

    fn interface<E>(&self) -> BusInterface<'_, Self, E, P, false> {
        BusInterface::from_ref(self)
    }

    fn interface_mut<E>(&mut self) -> BusInterface<'_, Self, E, P, true> {
        BusInterface::from_mut(self)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn pin(&self, bit: usize) -> Result<&P, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&self.pins[bit])
    }

    fn pin_mut(&mut self, bit: usize) -> Result<&mut P, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&mut self.pins[bit])
    }

    fn for_each_pin_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut P),
    {
        self.pins.iter_mut().for_each(f);
    }

    fn read(&self) -> Option<usize> {
        let collapsed = self.pins.iter().map(|pin| pin.collapsed()).collect();
        self.collapsed_as_usize(collapsed)
    }

    fn read_prev(&self) -> Option<usize> {
        let collapsed = self.pins.iter().map(|pin| pin.prev_collapsed()).collect();
        self.collapsed_as_usize(collapsed)
    }

    fn add_possible_drive_in(&mut self, val: usize) -> Result<(), PinError> {
        self.check_if_drive_val_too_large(val)?;
        for bit in 0..self.size {
            self.pins[bit].set_drive_in(bit::get_bit_of_usize(val, bit), true)?;
        }
        Ok(())
    }

    fn add_possible_drive_in_wrapping(&mut self, val: usize) -> Result<(), PinError> {
        self.add_possible_drive_in(bit::get_low_bits_of_usize(val, self.size))
    }
}

impl<P: SinglePinOutput> BusOutput<P> for StandardBus<P> {
    fn pin_out(&mut self, bit: usize) -> Result<&mut P, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&mut self.pins[bit])
    }

    fn add_possible_drive_out(&mut self, val: usize) -> Result<(), PinError> {
        self.check_if_drive_val_too_large(val)?;
        for bit in 0..self.size {
            self.pins[bit].set_drive_out(bit::get_bit_of_usize(val, bit), true)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pin::{PinError, single::mock_pin::MockPin};
    use rstest::{fixture, rstest};

    type BusType = StandardBus<MockPin>;
    const BUS_NAME: &str = "bus";

    type DriveFn = fn(&mut BusType, usize) -> Result<(), PinError>;

    #[fixture]
    fn bus() -> BusType {
        StandardBus::new(String::from(BUS_NAME), 8)
    }

    #[rstest]
    fn name(bus: BusType) {
        assert_eq!(bus.name(), BUS_NAME);
    }

    #[rstest]
    fn pin_out_of_range(mut bus: BusType) {
        assert!(matches!(
            bus.pin(8).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ));
        assert!(matches!(
            bus.pin_mut(9).err().unwrap(),
            PinError::BitOutOfRange { .. }
        ));
    }

    #[rstest]
    fn read_success(mut bus: BusType) {
        bus.add_possible_drive_in(0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
        bus.post_tick_update();
        bus.add_possible_drive_in(0x89).unwrap();
        assert_eq!(bus.read_prev().unwrap(), 0x67);
    }

    #[rstest]
    fn read_fail(mut bus: BusType) {
        bus.add_possible_drive_in(0x67).unwrap();
        bus.pin_mut(6).unwrap().set_all_signals_in(false).unwrap();
        assert!(bus.read().is_none());
    }

    #[rstest]
    fn drive_in(
        mut bus: BusType,
        #[values(
            StandardBus::add_possible_drive_in,
            StandardBus::add_possible_drive_out
        )]
        func: DriveFn,
    ) {
        func(&mut bus, 0x67).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_in_too_large(
        mut bus: BusType,
        #[values(
            StandardBus::add_possible_drive_in,
            StandardBus::add_possible_drive_out
        )]
        func: DriveFn,
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
        bus.add_possible_drive_in_wrapping(ival).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }

    #[rstest]
    fn for_each_pin(mut bus: BusType) {
        bus.for_each_pin_mut(|pin| pin.add_drive_in(false).unwrap());
        assert_eq!(bus.read().unwrap(), 0);
    }
}
