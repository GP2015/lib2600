use crate::{
    Riot,
    riot::lines::{RiotLineStates, RiotOutputLines},
};
use emutils::line::LineError;

impl Riot {
    pub(crate) fn read_timer(&mut self, lines: &mut RiotOutputLines) -> Result<(), LineError> {
        lines.db.copy_from_reg(self.con.db, &self.timer)?;
        self.timer_ir_flag.add(false, false);
        Ok(())
    }

    pub(crate) fn write_timer(&mut self, states: &RiotLineStates) {
        self.timer.copy_from_bus_state(&states.db, false);
        self.timer_ir_flag.add(false, false);

        self.timer_interval
            .bit_mut::<0>()
            .copy_from_line_state(&states.a.line_state::<0>(), false);

        self.timer_interval
            .bit_mut::<1>()
            .copy_from_line_state(&states.a.line_state::<1>(), false);

        let mut first_iter = true;

        #[allow(clippy::indexing_slicing)]
        for interval in self
            .timer_interval
            .iter_possible_reads()
            .map(|val| TIMER_INTERVALS[val] - 1)
        {
            #[allow(clippy::unwrap_used)]
            self.sub_timer.add(interval, false && first_iter).unwrap();

            first_iter = false;
        }
    }
}
