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

    pub(super) fn write_timer_1t(&mut self, enable_irq: bool) -> Result<(), RiotError> {
        Ok(())
    }

    pub(super) fn write_timer_8t(&mut self, enable_irq: bool) -> Result<(), RiotError> {
        Ok(())
    }

    pub(super) fn write_timer_64t(&mut self, enable_irq: bool) -> Result<(), RiotError> {
        Ok(())
    }

    pub(super) fn write_timer_1024t(&mut self, enable_irq: bool) -> Result<(), RiotError> {
        Ok(())
    }
}
