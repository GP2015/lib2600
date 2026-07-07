pub mod reads;
pub mod regs;

use crate::{
    common::{
        combine::{Combine, mux_matches},
        cond::{IsCondition, base::BaseCondition, check::CheckIs},
        line::{multi::BusDriveState, single::DriveState},
        read::{multi::MultiRead, single::SingleRead},
        reg::{BitReg, MBitReg},
        signal::LineSignal,
    },
    riot::{
        reads::{RiotAllReads, RiotLineReads},
        regs::RiotRegs,
    },
};
use core::array;
use itertools::izip;

const RAM_SIZE: usize = 128;
const PB_CONNECTED_LINES: [u8; 5] = [0, 1, 3, 6, 7];
const TIMER_INTERVALS: [u16; 4] = [1, 8, 64, 1024];

fn cs_cond(r: &RiotAllReads) -> BaseCondition {
    r.line.cs1.as_cond() & !r.line.cs2.as_cond()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    pub db_out: BusDriveState<8>,
    pub pa_out: BusDriveState<8>,
    pub pb_out: BusDriveState<5>,
    reg: RiotRegs,
    ram: [MBitReg<8>; RAM_SIZE],
    old_pa7_read: SingleRead,
}

impl Riot {
    pub fn new() -> Self {
        Self {
            db_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            pa_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            pb_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            reg: RiotRegs::new(),
            ram: array::from_fn(|_| [BitReg::Unknown; _].into()),
            old_pa7_read: SingleRead::Unknown,
        }
    }

    fn update_edc(&mut self, r: &RiotAllReads, pa7_read: SingleRead) {
        let edge_type = r.reg.edc_edge_type.as_cond();
        let old_pa7 = self.old_pa7_read.as_cond();
        let new_pa7 = pa7_read.as_cond();

        self.reg.edc_ir_flag = Combine::mux(
            (edge_type & !old_pa7 & new_pa7) | (!edge_type & old_pa7 & !new_pa7),
            &|| r.reg.edc_ir_flag,
            &|| SingleRead::High,
        );
    }

    fn timer_interval_read(read: &MultiRead<2>) -> MultiRead<10> {
        read.iter_possible_reads()
            .map(|i| MultiRead::from_value(TIMER_INTERVALS[usize::from(i)]))
            .reduce(|acc, val| acc.combine_with(&val))
            .expect("MultiRead will always have at least one possible read")
    }

    fn update_timer(&mut self, r: &RiotAllReads) {
        let ir_flag = r.reg.timer_ir_flag.as_cond();
        let timer_0 = r.reg.timer.is(0);
        let sub_timer_0 = r.reg.sub_timer.is(0);

        self.reg.timer = Combine::mux(ir_flag | sub_timer_0, &|| r.reg.timer.clone(), &|| {
            r.reg.timer.decremented()
        });

        let def = &r.reg.sub_timer;
        self.reg.sub_timer = mux_matches!(
            (!ir_flag & !def.is(0), &|| def.decremented()),
            (!ir_flag & def.is(0) & !timer_0, &|| {
                Self::timer_interval_read(&r.reg.timer_interval)
            }),
            &|| [SingleRead::Unknown; _].into()
        );

        self.reg.timer_interval = Combine::mux(
            ir_flag | (sub_timer_0 & timer_0),
            &|| r.reg.timer_interval.clone(),
            &|| [SingleRead::Unknown; _].into(),
        );

        self.reg.timer_ir_flag = Combine::mux(
            ir_flag | (sub_timer_0 & timer_0),
            &|| SingleRead::Low,
            &|| SingleRead::High,
        );
    }

    fn update_ram_bytes(&mut self, r: &RiotAllReads) {
        for addr in r.line.a.iter_possible_reads() {
            let ram_byte = &mut self.ram[usize::from(addr)];
            *ram_byte = Combine::mux(
                cs_cond(r) & !r.line.rs.as_cond() & !r.line.rw.as_cond(),
                &|| ram_byte.clone(),
                &|| r.line.db.clone(),
            );
        }
    }

    fn update_io_regs(&mut self, r: &RiotAllReads) {
        let set_io_reg = |reg: &mut MBitReg<8>, read: &MultiRead<8>, a0, a1| {
            *reg = Combine::mux(
                cs_cond(r)
                    & r.line.rs.as_cond()
                    & !r.line.a[2].as_cond()
                    & r.line.a[0].as_cond().is(a0)
                    & r.line.a[1].as_cond().is(a1)
                    & !r.line.rw.as_cond(),
                &|| read.clone(),
                &|| r.line.db.clone(),
            );
        };

        macro_rules! set_io_reg {
            ($(($reg:ident, $a0:literal, $a1:literal)),+ $(,)?) => {$(
                set_io_reg(&mut self.reg.$reg, &r.reg.$reg, $a0, $a1);
            )+};
        }

        set_io_reg!(
            (ora, false, false),
            (orb, false, true),
            (ddra, true, false),
            (ddrb, true, true),
        );
    }

    fn update_edc_regs(&mut self, r: &RiotAllReads) {
        let def_cond = cs_cond(r) & r.line.rs.as_cond() & r.line.a[2].as_cond();

        self.reg.edc_ir_flag = Combine::mux(
            def_cond & r.line.rw.as_cond() & r.line.a[0].as_cond(),
            &|| r.reg.edc_ir_flag,
            &|| SingleRead::Low,
        );

        self.reg.edc_edge_type = Combine::mux(
            def_cond & !r.line.rw.as_cond() & !r.line.a[4].as_cond(),
            &|| r.reg.edc_edge_type,
            &|| r.line.a[0],
        );
    }

    fn update_timer_regs(&mut self, r: &RiotAllReads) {
        let cond = cs_cond(r) & r.line.rs.as_cond() & r.line.a[2].as_cond();
        let tw_cond = cond & !r.line.rw.as_cond() & r.line.a[4].as_cond();

        self.reg.timer = Combine::mux(tw_cond, &|| r.reg.timer.clone(), &|| r.line.db.clone());

        for (bit, reg) in self.reg.timer_interval.iter_mut().enumerate() {
            *reg = Combine::mux(tw_cond, &|| r.reg.timer_interval[bit], &|| r.line.a[bit]);
        }

        self.reg.sub_timer = Combine::mux(tw_cond, &|| r.reg.sub_timer.clone(), &|| {
            Self::timer_interval_read(
                // This must use the new timer_interval value
                &self.reg.timer_interval,
            )
        });

        self.reg.timer_ir_flag = Combine::mux(
            cond & ((!r.line.rw.as_cond() & r.line.a[4].as_cond())
                | (r.line.rw.as_cond() & !r.line.a[0].as_cond())),
            &|| r.reg.timer_ir_flag,
            &|| SingleRead::Low,
        );
    }

    fn update_db_bus(&mut self, r: &RiotAllReads) {
        let high_z_out = &|| BusDriveState::from_signals(&[LineSignal::HighZ; 8]);

        let ram_read = &|| {
            BusDriveState::from_multi_read(
                &r.line
                    .a
                    .iter_possible_reads()
                    .map(|addr| self.ram[usize::from(addr)].clone())
                    .reduce(|acc, byte| acc.combine_with(&byte))
                    .expect("SingleRead will always have at least one possible read"),
            )
        };

        let ir_reg_read = &|| {
            array::from_fn(|bit| match bit {
                7 => DriveState::from(r.reg.timer_ir_flag),
                6 => DriveState::from(r.reg.edc_ir_flag),
                _ => DriveState::from(LineSignal::HighZ),
            })
            .into()
        };

        let io_read = &|| {
            Combine::mux(
                r.line.a[0].as_cond(),
                &|| {
                    Combine::mux(
                        r.line.a[1].as_cond(),
                        &|| BusDriveState::from_multi_read(&r.reg.ora),
                        &|| BusDriveState::from_multi_read(&r.reg.orb),
                    )
                },
                &|| {
                    Combine::mux(
                        r.line.a[1].as_cond(),
                        &|| BusDriveState::from_multi_read(&r.reg.ddra),
                        &|| BusDriveState::from_multi_read(&r.reg.ddrb),
                    )
                },
            )
        };

        let timer_read = &|| BusDriveState::from_multi_read(&r.reg.timer);

        self.db_out = Combine::mux(cs_cond(r) & r.line.rw.as_cond(), high_z_out, &|| {
            mux_matches!(
                (!r.line.rs.as_cond(), ram_read),
                (!r.line.a[2].as_cond(), io_read),
                (r.line.a[0].as_cond(), ir_reg_read),
                timer_read
            )
        });
    }

    fn update_peripherals(&mut self, r: &RiotAllReads) {
        for (pa_line, &ddra_bit, &ora_bit) in
            izip!(self.pa_out.iter_mut(), r.reg.ddra.iter(), r.reg.ora.iter())
        {
            *pa_line = Combine::mux(
                ddra_bit.as_cond(),
                &|| DriveState::from(LineSignal::HighZ),
                &|| DriveState::from(ora_bit),
            );
        }

        for (pb_out_state, &pb_con_index) in self.pb_out.iter_mut().zip(PB_CONNECTED_LINES.iter()) {
            *pb_out_state = Combine::mux(
                r.reg.ddrb[usize::from(pb_con_index)].as_cond(),
                &|| DriveState::from(LineSignal::HighZ),
                &|| DriveState::from(r.reg.orb[usize::from(pb_con_index)]),
            );
        }
    }

    pub fn handle_rising_edge(&mut self, line_reads: RiotLineReads) {
        let mut r = RiotAllReads::new(line_reads, self.reg.clone());

        self.update_timer(&r);
        self.update_edc(&r, r.line.pa[7]);

        r.update(self.reg.clone());
        self.update_ram_bytes(&r);
        self.update_io_regs(&r);
        self.update_edc_regs(&r);
        self.update_timer_regs(&r);

        r.update(self.reg.clone());
        self.update_db_bus(&r);
        self.update_peripherals(&r);

        let new_pa7_read = r.line.pa[7].combine_with(&self.old_pa7_read);
        r.update(self.reg.clone());
        self.update_edc(&r, new_pa7_read);

        self.old_pa7_read = new_pa7_read;
    }

    pub fn handle_falling_edge(&mut self) {
        self.db_out = BusDriveState::from_signals(&[LineSignal::HighZ; 8]);
    }
}
