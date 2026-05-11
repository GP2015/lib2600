use crate::{
    common::line::{bus::Bus, error::LineError},
    riot::core::{Riot, TIMER_INTERVALS},
};
use itertools::izip;

const LOW_ONLY: (bool, bool) = (true, false);
const HIGH_ONLY: (bool, bool) = (false, true);

impl Riot {
    pub fn update_peripherals(&self, pa: &mut Bus<8>, pb: &mut Bus<8>) -> Result<(), LineError> {
        for (p, bus_con, ddr, or) in [
            (pa, self.pa_con, &self.reg.ddra, &self.reg.ora),
            (pb, self.pb_con, &self.reg.ddrb, &self.reg.orb),
        ] {
            for ((p_line, line_con), ddr_bit, or_bit) in
                izip!(p.iter_mut(bus_con)?, ddr.iter(), or.iter())
            {
                let (ddr_low, ddr_high) = ddr_bit.state().low_high_possible();

                if ddr_high {
                    p_line.copy_from_reg_state(line_con, or_bit.state())?;
                }

                if ddr_low {
                    p_line.add_high_z(line_con)?;
                }
            }
        }

        Ok(())
    }

    pub const fn update_edc(&mut self, pa: &Bus<8>) {
        let new_pa7_state = pa.line::<7>().state();

        match (
            self.reg.edc_edge_type.state().low_high_possible(),
            self.old_pa7_state.could_read_low_high(),
            new_pa7_state.could_read_low_high(),
        ) {
            (HIGH_ONLY, LOW_ONLY, HIGH_ONLY) | (LOW_ONLY, HIGH_ONLY, LOW_ONLY) => {
                self.reg.edc_ir_flag.set_all(true, false);
            }
            ((_, true), (true, _), (_, true)) | ((true, _), (_, true), (true, _)) => {
                self.reg.edc_ir_flag.add(true);
            }
            _ => (),
        }

        self.old_pa7_state = new_pa7_state;
    }

    pub fn update_timer(&mut self) {
        let timer_state = self.reg.timer.state();
        let sub_timer_state = self.reg.sub_timer.state();
        let flag_state = self.reg.timer_ir_flag.state();
        let interval_state = self.reg.timer_interval.state();

        let (timer_zero, timer_other) = timer_state.could_be_val_diff(0);
        let (sub_timer_zero, sub_timer_other) = sub_timer_state.could_be_val_diff(0);
        let (flag_low, flag_high) = flag_state.low_high_possible();

        self.reg.timer.remove_all();
        self.reg.sub_timer.remove_all();
        self.reg.timer_interval.remove_all();
        self.reg.timer_ir_flag.remove_all();

        if flag_low && sub_timer_other {
            self.reg.timer.copy_from_reg_state(&timer_state);
        }

        if flag_high || (flag_low && sub_timer_zero) {
            self.reg
                .timer
                .copy_from_reg_state(&timer_state.decremented());
        }

        if flag_high || (flag_low && sub_timer_zero && timer_zero) {
            for bit in self.reg.sub_timer.iter_mut() {
                bit.set_all(true, true);
            }
        } else if flag_low {
            if sub_timer_other {
                self.reg
                    .sub_timer
                    .copy_from_reg_state(&sub_timer_state.decremented());
            }

            if sub_timer_zero && timer_other {
                for interval in self
                    .reg
                    .timer_interval
                    .state()
                    .iter_possible_reads()
                    .map(|val| TIMER_INTERVALS[val] - 1)
                {
                    self.reg.sub_timer.add(interval);
                }
            }
        }

        if flag_high || (flag_low && sub_timer_zero && timer_zero) {
            for bit in self.reg.timer_interval.iter_mut() {
                bit.set_all(true, true);
            }
        } else if flag_low && (sub_timer_other || (sub_timer_zero && timer_other)) {
            self.reg.timer_interval.copy_from_reg_state(&interval_state);
        }

        if flag_high || (flag_low && sub_timer_zero && timer_zero) {
            self.reg.timer_ir_flag.add(true);
        }

        if flag_low && (sub_timer_other || (sub_timer_zero && timer_other)) {
            self.reg.timer_ir_flag.add(false);
        }
    }
}
