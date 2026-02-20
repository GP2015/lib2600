use crate::{
    bit,
    pin::{BusCore, BusOutput, PinError, PinSignal, SinglePinCore, SinglePinOutput},
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

    fn add_possible_tri_state_out(&mut self) {
        self.pins
            .iter_mut()
            .for_each(|pin| pin.set_tri_state_out(true));
    }

    fn remove_all_possible_out(&mut self) {
        self.pins
            .iter_mut()
            .for_each(|pin| pin.set_all_signals_out(false).unwrap());
    }

    fn set_all_possible_out_to_prev(&mut self) -> Result<(), PinError> {
        for bit in 0..self.size {
            self.pins[bit].set_possible_out_to_prev()?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::pin::InputPin;

//     type BusType = StandardBus<InputPin>;

//     fn test() {
//         let mut bus = BusType::new(String::from("hi"), 8);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::pin::{PinError, single::mock_pin::MockPin};

//     use super::*;
//     use rstest::{fixture, rstest};

//     type BusType = StandardBus<MockPin<PinError>>;

//     #[fixture]
//     fn bus() -> BusType {
//         StandardBus::new(String::new(), 8)
//     }

//     type EmptyRes = Result<(), PinError>;
//     type DriveValue = fn(&mut BusType, val: usize) -> EmptyRes;
//     type TriState = fn(&mut BusType);
//     type Undefine = fn(&mut BusType) -> EmptyRes;

//     #[rstest]
//     fn state(mut bus: BusType) {
//         let states = [
//             PinState::Undefined,
//             PinState::Undefined,
//             PinState::High,
//             PinState::Low,
//             PinState::TriState,
//             PinState::Undefined,
//             PinState::Undefined,
//             PinState::Undefined,
//         ];

//         for (i, state) in states.iter().enumerate() {
//             bus.pin_mut(i).unwrap().signal_in(*state).unwrap();
//         }

//         assert_eq!(bus.state(), states);
//         bus.post_tick_update();
//         bus.drive_in(0x67).unwrap();
//         assert_eq!(bus.prev_state(), states);
//     }

//     #[rstest]
//     fn state_as_bool(mut bus: BusType) {
//         let states = [
//             PinState::Undefined,
//             PinState::Undefined,
//             PinState::High,
//             PinState::Low,
//             PinState::TriState,
//             PinState::Undefined,
//             PinState::Undefined,
//             PinState::Undefined,
//         ];

//         for (i, state) in states.iter().enumerate() {
//             bus.pin_mut(i).unwrap().signal_in(*state).unwrap();
//         }
//         for (i, state) in bus.state_as_bool().iter().enumerate() {
//             assert_eq!(*state, PinState::as_bool(&states[i]));
//         }
//         bus.post_tick_update();
//         bus.drive_in(0x67).unwrap();
//         for (i, state) in bus.prev_state_as_bool().iter().enumerate() {
//             assert_eq!(*state, PinState::as_bool(&states[i]));
//         }
//     }

//     #[rstest]
//     fn read(mut bus: BusType) {
//         bus.drive_in(0x67).unwrap();
//         assert_eq!(bus.read().unwrap(), 0x67);
//         bus.post_tick_update();
//         bus.drive_in(0x89).unwrap();
//         assert_eq!(bus.read_prev().unwrap(), 0x67);
//     }

//     #[rstest]
//     fn drive(
//         mut bus: BusType,
//         #[values(StandardBus::drive_in, StandardBus::drive_out)] func: DriveValue,
//     ) {
//         func(&mut bus, 0x67).unwrap();
//         assert_eq!(bus.read().unwrap(), 0x67);
//     }

//     #[rstest]
//     fn drive_too_large(
//         mut bus: BusType,
//         #[values(StandardBus::drive_in, StandardBus::drive_out)] func: DriveValue,
//     ) {
//         assert!(matches!(
//             func(&mut bus, 0x167).err().unwrap(),
//             PinError::DriveValueTooLarge { .. }
//         ))
//     }

//     #[rstest]
//     #[case(0x67, 0x67)]
//     #[case(0x167, 0x67)]
//     fn wrapping_drive_in(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
//         bus.wrapping_drive_in(ival).unwrap();
//         assert_eq!(bus.read().unwrap(), oval);
//     }

//     #[rstest]
//     fn tri_state(
//         mut bus: BusType,
//         #[values(StandardBus::tri_state_in, StandardBus::tri_state_out)] func: TriState,
//     ) {
//         func(&mut bus);
//         assert_eq!(bus.state(), vec![PinState::TriState; 8]);
//     }

//     #[rstest]
//     fn undefine(
//         mut bus: BusType,
//         #[values(StandardBus::undefine_in, StandardBus::undefine_out)] func: Undefine,
//     ) {
//         func(&mut bus).unwrap();
//         assert_eq!(bus.state(), vec![PinState::Undefined; 8]);
//     }
// }
