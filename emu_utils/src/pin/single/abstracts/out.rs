use crate::{
    pin::{PinSignal, SinglePinCore},
    register::BitRegister,
};

pub trait SinglePinOutput<'a>: SinglePinCore<'a> {
    fn set_high_out(&mut self, possible: bool) -> Result<(), Self::ErrType>;
    fn set_low_out(&mut self, possible: bool) -> Result<(), Self::ErrType>;
    fn set_high_z_out(&mut self, possible: bool);

    fn add_high_out(&mut self) -> Result<(), Self::ErrType> {
        self.set_high_out(true)
    }

    fn add_low_out(&mut self) -> Result<(), Self::ErrType> {
        self.set_low_out(true)
    }

    fn add_high_z_out(&mut self) {
        self.set_high_z_out(true);
    }

    fn set_out(&mut self, signal: PinSignal, possible: bool) -> Result<(), Self::ErrType> {
        match signal {
            PinSignal::High => self.set_high_out(possible)?,
            PinSignal::Low => self.set_low_out(possible)?,
            PinSignal::HighZ => self.set_high_z_out(possible),
        }
        Ok(())
    }

    fn add_out(&mut self, signal: PinSignal) -> Result<(), Self::ErrType> {
        self.set_out(signal, true)
    }

    fn set_drive_out(&mut self, bool_signal: bool, possible: bool) -> Result<(), Self::ErrType> {
        if bool_signal {
            self.set_high_out(possible)
        } else {
            self.set_low_out(possible)
        }
    }

    fn add_drive_out(&mut self, bool_signal: bool) -> Result<(), Self::ErrType> {
        self.set_drive_out(bool_signal, true)
    }

    fn set_all_out(&mut self, possible: bool) -> Result<(), Self::ErrType> {
        self.set_high_out(possible)?;
        self.set_low_out(possible)?;
        self.set_high_z_out(possible);
        Ok(())
    }

    fn add_all_out(&mut self) -> Result<(), Self::ErrType> {
        self.set_all_out(true)
    }

    fn output_from_reg(
        &mut self,
        reg: &BitRegister,
        only_possible: bool,
    ) -> Result<(), Self::ErrType> {
        if only_possible {
            self.set_high_out(reg.high_possible())?;
            self.set_low_out(reg.low_possible())?;
        } else {
            if reg.high_possible() {
                self.add_high_out()?;
            }

            if reg.low_possible() {
                self.add_low_out()?;
            }
        }
        Ok(())
    }
}
