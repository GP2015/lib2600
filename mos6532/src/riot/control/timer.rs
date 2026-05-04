use crate::{
    Riot,
    riot::{lines::RiotOutputLines, states::RiotLineStates},
};
use emutils::line::LineError;

impl Riot {
    pub(crate) fn write_timer(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_instruction: bool,
    ) -> Result<(), LineError> {
        todo!()
    }

    pub(crate) fn read_timer(
        &mut self,
        lines: &mut RiotOutputLines,
        states: &RiotLineStates,
        only_instruction: bool,
    ) -> Result<(), LineError> {
        todo!()
    }

    // pub(super) fn tick_timer(&mut self) -> Result<(), LineError> {
    //     match self.timer_flag.read()? {
    //         false => {
    //             self.sub_timer.decrement()?;

    //             if self.sub_timer.read()? == 0 {
    //                 if self.timer.read()? == 0 {
    //                     //
    //                 } else {
    //                     self.timer.decrement()?;
    //                 }

    //                 self.sub_timer.write(self.timer_inc.read()?)?;
    //             }
    //         }
    //         true => {
    //             //
    //         }
    //     }

    //     Ok(())
    // }

    // pub(super) fn read_timer(&mut self, enable_irq: bool) -> Result<(), LineError> {
    //     Ok(())
    // }

    // pub(super) fn write_timer_1t(&mut self, enable_irq: bool) -> Result<(), LineError> {
    //     Ok(())
    // }

    // pub(super) fn write_timer_8t(&mut self, enable_irq: bool) -> Result<(), LineError> {
    //     Ok(())
    // }

    // pub(super) fn write_timer_64t(&mut self, enable_irq: bool) -> Result<(), LineError> {
    //     Ok(())
    // }

    // pub(super) fn write_timer_1024t(&mut self, enable_irq: bool) -> Result<(), LineError> {
    //     Ok(())
    // }
}
