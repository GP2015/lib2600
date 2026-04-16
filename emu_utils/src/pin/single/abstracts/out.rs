use crate::{
    pin::{PinSignal, SinglePinCore},
    register::BitRegister,
};

pub trait SinglePinOutput<'a>: SinglePinCore<'a> {
    fn add_signal_out(
        &mut self,
        signal: PinSignal,
        only_possible: bool,
    ) -> Result<(), Self::ErrType>;
    fn remove_signal_out(&mut self, signal: PinSignal);

    fn add_high_out(&mut self, only_possible: bool) -> Result<(), Self::ErrType> {
        self.add_signal_out(PinSignal::High, only_possible)
    }

    fn add_low_out(&mut self, only_possible: bool) -> Result<(), Self::ErrType> {
        self.add_signal_out(PinSignal::Low, only_possible)
    }

    fn add_high_z_out(&mut self, only_possible: bool) {
        self.add_signal_out(PinSignal::HighZ, only_possible)
            .expect("setting high impedance in cannot cause a short-circuit");
    }

    fn remove_high_out(&mut self) {
        self.remove_signal_out(PinSignal::High);
    }

    fn remove_low_out(&mut self) {
        self.remove_signal_out(PinSignal::Low);
    }

    fn remove_high_z_out(&mut self) {
        self.remove_signal_out(PinSignal::HighZ);
    }

    fn add_drive_out(
        &mut self,
        bool_signal: bool,
        only_possible: bool,
    ) -> Result<(), Self::ErrType> {
        if bool_signal {
            self.add_high_out(only_possible)
        } else {
            self.add_low_out(only_possible)
        }
    }

    fn remove_drive_out(&mut self, bool_signal: bool) {
        if bool_signal {
            self.remove_high_out();
        } else {
            self.remove_low_out();
        }
    }

    fn set_all_out(&mut self, high: bool, low: bool, high_z: bool) -> Result<(), Self::ErrType> {
        if high {
            self.add_high_out(false)?;
        } else {
            self.remove_high_out();
        }

        if low {
            self.add_low_out(false)?;
        } else {
            self.remove_low_out();
        }

        if high_z {
            self.add_high_z_out(false);
        } else {
            self.remove_high_z_out();
        }

        Ok(())
    }

    fn output_from_pin<P>(&mut self, pin: &P, only_possible: bool) -> Result<(), Self::ErrType>
    where
        P: SinglePinCore<'a>,
    {
        if only_possible {
            self.set_all_out(
                pin.high_possible(),
                pin.low_possible(),
                pin.high_z_possible(),
            )?;
        } else {
            if pin.high_possible() {
                self.add_high_out(false)?;
            }

            if pin.low_possible() {
                self.add_low_out(false)?;
            }

            if pin.high_z_possible() {
                self.add_high_z_out(false);
            }
        }
        Ok(())
    }

    fn output_from_reg(
        &mut self,
        reg: &BitRegister,
        only_possible: bool,
    ) -> Result<(), Self::ErrType> {
        if only_possible {
            self.set_all_out(reg.high_possible(), reg.low_possible(), false)?;
        } else {
            if reg.high_possible() {
                self.add_high_out(false)?;
            }

            if reg.low_possible() {
                self.add_low_out(false)?;
            }
        }
        Ok(())
    }
}
