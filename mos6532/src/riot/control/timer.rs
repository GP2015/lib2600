use crate::{
    Riot,
    riot::{TIMER_INTERVALS, states::RiotStates},
};
use emutils::line::{Bus, LineError};

impl Riot {
    // Only (pseudo-)instruction that writes to timer_ir_flag
    pub(crate) const fn clear_timer_ir_flag(&mut self, only_possible: bool) {
        if only_possible {
            self.reg.timer_ir_flag.remove_all();
        }

        self.reg.timer_ir_flag.add(false);
    }

    pub(crate) fn read_timer(&self, db: &mut Bus<8>) -> Result<(), LineError> {
        let timer_state = self.reg.timer.state();
        db.copy_from_reg_state(self.db_con, &timer_state)
    }

    // Only instruction that writes to timer, sub-timer, timer_interval
    pub(crate) fn write_timer(&mut self, states: &RiotStates, only_possible: bool) {
        if only_possible {
            self.reg.timer.remove_all();
            self.reg.sub_timer.remove_all();
            self.reg.timer_interval.remove_all();
        }

        self.reg.timer.copy_from_bus_state(&states.db);

        macro_rules! set_timer_interval {
            ($($size:literal),+) => {$(
                let a_bit = &states.a.line_state::<$size>();
                self.reg.timer_interval.bit_mut::<$size>().copy_from_line_state(&a_bit);
            )+};
        }

        set_timer_interval!(0, 1);

        for interval in self
            .reg
            .timer_interval
            .state()
            .iter_possible_reads()
            .map(|val| TIMER_INTERVALS[val] - 1)
        {
            self.reg.sub_timer.add(interval).unwrap();
        }
    }
}
