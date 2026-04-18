use crate::{
    bit,
    pin::{BusCore, BusInputUI, BusOutput, PinCore, PinError, PinInputUI, PinOutput, PinSignal},
};
use itertools::Itertools;

pub struct StandardBus<P> {
    name: String,
    pins: Box<[P]>,
}

impl<P> StandardBus<P> {
    fn check_for_bit_out_of_range(&self, bit: usize) -> Result<(), PinError> {
        if bit >= self.pins.len() {
            Err(PinError::BitOutOfRange {
                name: self.name.clone(),
                bit,
                size: self.pins.len(),
            })
        } else {
            Ok(())
        }
    }

    fn check_if_drive_val_too_large(&self, val: usize) -> Result<(), PinError> {
        if bit::usize_exceeds_bit_count(val, self.pins.len()) {
            Err(PinError::DriveValueTooLarge {
                name: self.name.clone(),
                value: val,
                size: self.pins.len(),
            })
        } else {
            Ok(())
        }
    }

    fn collapsed_as_usize(collapsed: &[Option<PinSignal>]) -> Option<usize> {
        let mut combined = 0;
        for &bit in collapsed.iter().rev() {
            let b = bit?.as_bool()?;
            combined = (combined << 1) | usize::from(b);
        }
        Some(combined)
    }

    fn bools_as_usize(bools: &[bool]) -> usize {
        let mut combined = 0;
        for b in bools.iter().rev().map(|&b| usize::from(b)) {
            combined = (combined << 1) | b;
        }
        combined
    }
}

impl<P> BusCore for StandardBus<P>
where
    P: PinCore,
{
    fn new(name: String, size: usize) -> Self {
        Self {
            pins: (0..size)
                .map(|bit| P::new(format!("{name}{bit}")))
                .collect(),
            name,
        }
    }

    fn post_tick_update(&mut self) {
        self.pins.iter_mut().for_each(P::post_tick_update);
    }
}

impl<P> BusInputUI for StandardBus<P>
where
    P: PinInputUI,
{
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> usize {
        self.pins.len()
    }

    fn pin(&self, bit: usize) -> Result<&impl PinInputUI, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&self.pins[bit])
    }

    fn pin_mut(&mut self, bit: usize) -> Result<&mut impl PinInputUI, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&mut self.pins[bit])
    }

    fn iter(&self) -> impl Iterator<Item = &impl PinInputUI> {
        self.pins.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut impl PinInputUI> {
        self.pins.iter_mut()
    }

    fn read(&self) -> Option<usize> {
        let collapsed = self
            .pins
            .iter()
            .map(P::collapsed)
            .collect::<Vec<Option<PinSignal>>>();
        Self::collapsed_as_usize(&collapsed)
    }

    fn read_prev(&self) -> Option<usize> {
        let collapsed = self
            .pins
            .iter()
            .map(P::prev_collapsed)
            .collect::<Vec<Option<PinSignal>>>();
        Self::collapsed_as_usize(&collapsed)
    }

    fn only_one_possible_read(&self) -> bool {
        for pin in &self.pins {
            if pin.could_read_high() && pin.could_read_low() {
                return false;
            }
        }
        true
    }

    fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.pins
            .iter()
            .map(P::possible_reads)
            .multi_cartesian_product()
            .map(|bools| Self::bools_as_usize(&bools))
    }

    fn iter_prev_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.pins
            .iter()
            .map(P::prev_possible_reads)
            .multi_cartesian_product()
            .map(|bools| Self::bools_as_usize(&bools))
    }

    fn add_drive_in_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), PinError> {
        for (bit, pin) in self.pins.iter_mut().enumerate() {
            pin.add_drive_in(bit::get_bit_of_usize(val, bit), only_possible)?;
        }
        Ok(())
    }

    fn add_drive_in(&mut self, val: usize, only_possible: bool) -> Result<(), PinError> {
        self.check_if_drive_val_too_large(val)?;
        self.add_drive_in_wrapping(
            bit::get_low_bits_of_usize(val, self.pins.len()),
            only_possible,
        )
    }
}

impl<P> BusOutput for StandardBus<P>
where
    P: PinOutput,
{
    fn pin_out(&self, bit: usize) -> Result<&impl PinOutput, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&self.pins[bit])
    }

    fn pin_out_mut(&mut self, bit: usize) -> Result<&mut impl PinOutput, PinError> {
        self.check_for_bit_out_of_range(bit)?;
        Ok(&mut self.pins[bit])
    }

    fn iter_out(&self) -> impl Iterator<Item = &impl PinOutput> {
        self.pins.iter()
    }

    fn iter_out_mut(&mut self) -> impl Iterator<Item = &mut impl PinOutput> {
        self.pins.iter_mut()
    }

    fn add_drive_out_wrapping(&mut self, val: usize, only_possible: bool) -> Result<(), PinError> {
        for (bit, pin) in self.pins.iter_mut().enumerate() {
            pin.add_drive_out(bit::get_bit_of_usize(val, bit), only_possible)?;
        }
        Ok(())
    }

    fn add_drive_out(&mut self, val: usize, only_possible: bool) -> Result<(), PinError> {
        self.check_if_drive_val_too_large(val)?;
        self.add_drive_out_wrapping(
            bit::get_low_bits_of_usize(val, self.pins.len()),
            only_possible,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pin::{PinError, single::mock::MockPin};
    use rstest::{fixture, rstest};

    type BusType = StandardBus<MockPin>;
    const BUS_NAME: &str = "bus";

    type DriveFn = fn(&mut BusType, usize, bool) -> Result<(), PinError>;

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
        bus.add_drive_in(0x67, false).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
        bus.post_tick_update();
        bus.add_drive_in(0x89, false).unwrap();
        assert_eq!(bus.read_prev().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_in(
        mut bus: BusType,
        #[values(StandardBus::add_drive_in, StandardBus::add_drive_out)] func: DriveFn,
    ) {
        func(&mut bus, 0x67, false).unwrap();
        assert_eq!(bus.read().unwrap(), 0x67);
    }

    #[rstest]
    fn drive_in_too_large(
        mut bus: BusType,
        #[values(StandardBus::add_drive_in, StandardBus::add_drive_out)] func: DriveFn,
    ) {
        assert!(matches!(
            func(&mut bus, 0x167, false).err().unwrap(),
            PinError::DriveValueTooLarge { .. }
        ));
    }

    #[rstest]
    #[case(0x67, 0x67)]
    #[case(0x167, 0x67)]
    fn wrapping_drive_in(mut bus: BusType, #[case] ival: usize, #[case] oval: usize) {
        bus.add_drive_in_wrapping(ival, false).unwrap();
        assert_eq!(bus.read().unwrap(), oval);
    }
}
