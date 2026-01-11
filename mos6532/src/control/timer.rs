use crate::{Riot, RiotError};

impl Riot {
    pub(super) fn tick_timer(&mut self) -> Result<(), RiotError> {
        match self.reg.timer_flag.read()? {
            false => {
                self.reg.sub_timer.decrement()?;

                if self.reg.sub_timer.read()? == 0 {
                    if self.reg.timer.read()? == 0 {
                        //
                    } else {
                        self.reg.timer.decrement()?;
                    }

                    self.reg.sub_timer.write(self.reg.timer_inc.read()?)?;
                }
            }
            true => {
                //
            }
        }

        Ok(())
    }

    pub(super) fn read_timer(&mut self) -> Result<(), RiotError> {
        Ok(())
    }
}
