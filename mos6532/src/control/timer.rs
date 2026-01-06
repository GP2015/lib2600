use crate::{RIOT, RIOTError};

impl RIOT {
    pub(super) fn tick_timer(&mut self) -> Result<(), RIOTError> {
        match self.reg.timer_flag.read()? {
            false => {
                self.reg.sub_timer.decrement()?;

                if self.reg.sub_timer.read()? == 0 {
                    if self.reg.timer.read()? == 0 {
                        //
                    } else {
                        self.reg.timer.decrement()?;
                    }

                    self.reg.sub_timer.drive(self.reg.timer_inc.read()?)?;
                }
            }
            true => {
                //
            }
        }

        Ok(())
    }

    pub(super) fn read_timer(&mut self) -> Result<(), RIOTError> {
        Ok(())
    }
}
