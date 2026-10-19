pub mod data;

use crate::{
    common::{
        bit::{multi::BitStringState, single::BitState},
        combine::{Combine, mux_matches},
        cond::BaseCondition,
        line::{
            output::{multi::BusOutput, single::LineOutput},
            signal::LineSignal,
        },
    },
    riot::data::{EdcRegValues, IoRegValues, RiotLineReads, RiotOutput, TimerRegValues},
};
use core::array;

const PB_CONNECTED_LINES: [usize; 5] = [0, 1, 3, 6, 7];

const TIMER_INTERVALS: [u16; 4] = [1, 8, 64, 1024];

fn cs_cond(r: &RiotLineReads) -> BaseCondition {
    r.cs1.as_cond() & !r.cs2.as_cond()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    ddra: BitStringState<8>,
    ddrb: BitStringState<8>,
    ora: BitStringState<8>,
    orb: BitStringState<8>,

    edc_edge_type: BitState,
    edc_ir_flag: BitState,

    timer: BitStringState<8>,
    sub_timer: BitStringState<10>,
    timer_interval: BitStringState<2>,
    timer_ir_flag: BitState,

    ram: [BitStringState<8>; 128],
}

impl Default for Riot {
    fn default() -> Self {
        Self::new()
    }
}

impl Riot {
    pub fn new() -> Self {
        Self {
            ddra: [BitState::Low; _].into(),
            ddrb: [BitState::Low; _].into(),
            ora: [BitState::Low; _].into(),
            orb: [BitState::Low; _].into(),

            edc_edge_type: BitState::Low,
            edc_ir_flag: BitState::Unknown,

            timer: [BitState::Unknown; _].into(),
            sub_timer: [BitState::Unknown; _].into(),
            timer_interval: [BitState::Unknown; _].into(),
            timer_ir_flag: BitState::Unknown,

            ram: array::from_fn(|_| [BitState::Unknown; _].into()),
        }
    }

    pub fn tick(&mut self, r: &RiotLineReads) -> RiotOutput {
        let new_timer_regs = self.timer_ticked();

        self.timer = new_timer_regs.timer;
        self.sub_timer = new_timer_regs.sub_timer;
        self.timer_interval = new_timer_regs.timer_interval;
        self.timer_ir_flag = new_timer_regs.timer_ir_flag;

        // This only modifies RAM, no registers
        self.update_ram_bytes(r);

        let new_io_regs = self.updated_io_regs(r);
        let new_edc_regs = self.updated_edc_regs(r);
        let new_timer_regs = self.updated_timer_regs(r);

        self.ddra = new_io_regs.ddra;
        self.ddrb = new_io_regs.ddrb;
        self.ora = new_io_regs.ora;
        self.orb = new_io_regs.orb;
        self.edc_ir_flag = new_edc_regs.edc_ir_flag;
        self.edc_edge_type = new_edc_regs.edc_edge_type;
        self.timer = new_timer_regs.timer;
        self.sub_timer = new_timer_regs.sub_timer;
        self.timer_interval = new_timer_regs.timer_interval;
        self.timer_ir_flag = new_timer_regs.timer_ir_flag;

        RiotOutput {
            db: self.db_out(r),
            pa: self.pa_out(),
            pb: self.pb_out(),
        }
    }

    // fn edc_ticked(&self, pa7_read: BitState) -> BitState {
    //     let edge_type = self.edc_edge_type.as_cond();
    //     let old_pa7 = self.old_pa7_read.as_cond();
    //     let new_pa7 = pa7_read.as_cond();

    //     Combine::mux(
    //         (edge_type & !old_pa7 & new_pa7) | (!edge_type & old_pa7 & !new_pa7),
    //         &|| self.edc_ir_flag,
    //         &|| BitState::High,
    //     )
    // }

    fn timer_interval_read(idx: BitStringState<2>) -> BitStringState<10> {
        idx.iter_possible_uints()
            .map(|i| BitStringState::from(TIMER_INTERVALS[usize::from(i)]))
            .reduce(|acc, val| acc.combine_with(&val))
            .expect("BitStringState will always have at least one possible read")
    }

    fn timer_ticked(&self) -> TimerRegValues {
        let ir_flag = self.timer_ir_flag.as_cond();
        let timer_0 = self.timer.is_uint(0);
        let sub_timer_0 = self.sub_timer.is_uint(0);

        TimerRegValues {
            timer: Combine::mux(ir_flag | sub_timer_0, &|| self.timer, &|| {
                self.timer.decremented()
            }),
            sub_timer: {
                let def = &self.sub_timer;
                mux_matches!(
                    (!ir_flag & !def.is_uint(0), &|| def.decremented()),
                    (!ir_flag & def.is_uint(0) & !timer_0, &|| {
                        Self::timer_interval_read(self.timer_interval)
                    }),
                    &|| [BitState::Unknown; _].into()
                )
            },
            timer_interval: Combine::mux(
                ir_flag | (sub_timer_0 & timer_0),
                &|| self.timer_interval,
                &|| [BitState::Unknown; _].into(),
            ),
            timer_ir_flag: Combine::mux(
                ir_flag | (sub_timer_0 & timer_0),
                &|| BitState::Low,
                &|| BitState::High,
            ),
        }
    }

    fn update_ram_bytes(&mut self, r: &RiotLineReads) {
        let ram_write_cond = cs_cond(r) & !r.rs.as_cond() & !r.rw.as_cond();

        let must_be = match ram_write_cond {
            BaseCondition::No => false,
            BaseCondition::Yes => true,
            BaseCondition::Unknown => return,
        };

        if let Some(addr) = r.a.value() {
            self.ram[usize::from(addr)].set_to_val(must_be, r.db);
        } else {
            for addr in r.a.iter_possible_uints() {
                let ram_byte = &mut self.ram[usize::from(addr)];
                *ram_byte = ram_byte.combine_with(&r.db);
            }
        }
    }

    fn updated_io_regs(&self, r: &RiotLineReads) -> IoRegValues {
        let def_cond = cs_cond(r) & r.rs.as_cond() & !r.a[2].as_cond() & !r.rw.as_cond();

        let io_reg_val = |reg, a0, a1| {
            Combine::mux(
                def_cond & r.a[0].as_cond().is_bool(a0) & r.a[1].as_cond().is_bool(a1),
                &|| reg,
                &|| r.db,
            )
        };

        IoRegValues {
            ddra: io_reg_val(self.ddra, true, false),
            ddrb: io_reg_val(self.ddrb, true, true),
            ora: io_reg_val(self.ora, false, false),
            orb: io_reg_val(self.orb, false, true),
        }
    }

    fn updated_edc_regs(&self, r: &RiotLineReads) -> EdcRegValues {
        let def_cond = cs_cond(r) & r.rs.as_cond() & r.a[2].as_cond();

        EdcRegValues {
            edc_ir_flag: Combine::mux(
                def_cond & r.rw.as_cond() & r.a[0].as_cond(),
                &|| self.edc_ir_flag,
                &|| BitState::Low,
            ),
            edc_edge_type: Combine::mux(
                def_cond & !r.rw.as_cond() & !r.a[4].as_cond(),
                &|| self.edc_edge_type,
                &|| r.a[0],
            ),
        }
    }

    fn updated_timer_regs(&self, r: &RiotLineReads) -> TimerRegValues {
        let def_cond = cs_cond(r) & r.rs.as_cond() & r.a[2].as_cond();
        let tw_cond = def_cond & !r.rw.as_cond() & r.a[4].as_cond();

        // This is used to calculate sub_timer.
        let new_timer_interval =
            array::from_fn(|bit| Combine::mux(tw_cond, &|| self.timer_interval[bit], &|| r.a[bit]))
                .into();

        TimerRegValues {
            timer: Combine::mux(tw_cond, &|| self.timer, &|| r.db),
            sub_timer: Combine::mux(tw_cond, &|| self.sub_timer, &|| {
                Self::timer_interval_read(new_timer_interval)
            }),
            timer_interval: new_timer_interval,
            timer_ir_flag: Combine::mux(
                def_cond
                    & ((!r.rw.as_cond() & r.a[4].as_cond()) | (r.rw.as_cond() & !r.a[0].as_cond())),
                &|| self.timer_ir_flag,
                &|| BitState::Low,
            ),
        }
    }

    fn db_out(&self, r: &RiotLineReads) -> BusOutput<8> {
        let high_z_out = &|| [LineSignal::HighZ; 8].into();

        let ram_out = &|| {
            r.a.iter_possible_uints()
                .map(|addr| self.ram[usize::from(addr)])
                .reduce(|acc, byte| acc.combine_with(&byte))
                .expect("BitState will always have at least one possible read")
                .into()
        };

        let ir_reg_out = &|| {
            array::from_fn(|bit| match bit {
                7 => LineOutput::from(self.timer_ir_flag),
                6 => LineOutput::from(self.edc_ir_flag),
                _ => LineOutput::from(LineSignal::HighZ),
            })
            .into()
        };

        let orb_out = &|| {
            array::from_fn(|bit| {
                Combine::mux(
                    self.ddrb[bit],
                    &|| {
                        PB_CONNECTED_LINES
                            .binary_search(&bit)
                            .map_or(BitState::Unknown, |idx| r.pb[idx])
                    },
                    &|| self.orb[bit],
                )
            })
            .into()
        };

        let io_out = &|| {
            Combine::mux(
                r.a[0],
                &|| Combine::mux(r.a[1], &|| r.pa.into(), orb_out),
                &|| Combine::mux(r.a[1], &|| self.ddra.into(), &|| self.ddrb.into()),
            )
        };

        let timer_out = &|| self.timer.into();

        Combine::mux(cs_cond(r) & r.rw.as_cond(), high_z_out, &|| {
            mux_matches!(
                (!r.rs.as_cond(), ram_out),
                (!r.a[2].as_cond(), io_out),
                (r.a[0], ir_reg_out),
                timer_out
            )
        })
    }

    fn pa_out(&self) -> BusOutput<8> {
        array::from_fn(|bit| {
            Combine::mux(
                self.ddra[bit],
                &|| LineOutput::from(LineSignal::HighZ),
                &|| LineOutput::from(self.ora[bit]),
            )
        })
        .into()
    }

    fn pb_out(&self) -> [LineOutput; 5] {
        array::from_fn(|bit| {
            Combine::mux(self.ddrb[bit], &|| LineSignal::HighZ.into(), &|| {
                self.orb[PB_CONNECTED_LINES[bit]].into()
            })
        })
    }
}
