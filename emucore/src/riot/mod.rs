pub mod reads;
pub mod regs;

use crate::{
    common::{
        BitReg, HasMux, MBitReg,
        condition::{BaseCondition, IsCondition},
        line::{
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        read::{
            multi::{IsMultiRead, MultiRead},
            single::SingleRead,
        },
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

fn cs_cond(reads: &RiotAllReads) -> BaseCondition {
    reads.line.cs1.as_cond() & !reads.line.cs2.as_cond()
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
            ram: [[BitReg::Unknown; _]; _],
            old_pa7_read: SingleRead::Unknown,
        }
    }

    fn update_edc(&mut self, reads: &RiotAllReads, pa7_read: SingleRead) {
        let def = &|| reads.reg.edc_ir_flag;
        self.reg.edc_ir_flag = SingleRead::mux(
            reads.reg.edc_edge_type.as_cond(),
            &|| {
                SingleRead::mux(self.old_pa7_read.as_cond(), def, &|| {
                    SingleRead::mux(pa7_read.as_cond(), &|| SingleRead::High, def)
                })
            },
            &|| {
                SingleRead::mux(
                    self.old_pa7_read.as_cond(),
                    &|| SingleRead::mux(pa7_read.as_cond(), def, &|| SingleRead::High),
                    def,
                )
            },
        );
    }

    fn timer_interval_read(read: MultiRead<2>) -> MultiRead<10> {
        read.iter_possible_reads()
            .map(|i| MultiRead::from_value(TIMER_INTERVALS[usize::from(i)]))
            .reduce(|acc, val| acc.combine_with(&val))
            .expect("MultiRead will always have at least one possible read")
    }

    fn update_timer(&mut self, reads: &RiotAllReads) {
        let def = &|| reads.reg.timer;
        self.reg.timer = MultiRead::mux(
            reads.reg.timer_ir_flag.as_cond(),
            &|| {
                MultiRead::mux(reads.reg.sub_timer.is_value(0), def, &|| {
                    def().decremented()
                })
            },
            &|| def().decremented(),
        );

        let def = &|| reads.reg.sub_timer;
        let unknown = &|| [SingleRead::Unknown; _];
        self.reg.sub_timer = MultiRead::mux(
            reads.reg.timer_ir_flag.as_cond(),
            &|| {
                MultiRead::mux(def().is_value(0), &|| def().decremented(), &|| {
                    MultiRead::mux(
                        reads.reg.timer.is_value(0),
                        &|| Self::timer_interval_read(reads.reg.timer_interval),
                        unknown,
                    )
                })
            },
            unknown,
        );

        let def = &|| reads.reg.timer_interval;
        let unknown = &|| [SingleRead::Unknown; _];
        self.reg.timer_interval = MultiRead::mux(
            reads.reg.timer_ir_flag.as_cond(),
            &|| {
                MultiRead::mux(reads.reg.sub_timer.is_value(0), def, &|| {
                    MultiRead::mux(reads.reg.timer.is_value(0), def, unknown)
                })
            },
            unknown,
        );

        let def = &|| reads.reg.timer_ir_flag;
        self.reg.timer_ir_flag = SingleRead::mux(
            def().as_cond(),
            &|| {
                SingleRead::mux(
                    reads.reg.sub_timer.is_value(0),
                    &|| SingleRead::Low,
                    &|| {
                        SingleRead::mux(reads.reg.timer.is_value(0), &|| SingleRead::Low, &|| {
                            SingleRead::High
                        })
                    },
                )
            },
            &|| SingleRead::High,
        );
    }

    fn update_ram_bytes(&mut self, reads: &RiotAllReads) {
        for addr in reads.line.a.iter_possible_reads() {
            let ram_byte = &mut self.ram[usize::from(addr)];

            let cond = cs_cond(reads) & !reads.line.rs.as_cond() & !reads.line.rw.as_cond();
            *ram_byte = MBitReg::mux(cond, &|| *ram_byte, &|| reads.line.db);
        }
    }

    fn update_io_regs(&mut self, reads: &RiotAllReads) {
        let set_io_reg = |reg: &mut MBitReg<8>, read: &MultiRead<8>, a0, a1| {
            let cond = cs_cond(reads)
                & reads.line.rs.as_cond()
                & !reads.line.a[2].as_cond()
                & reads.line.a[0].as_cond().is_bool(a0)
                & reads.line.a[1].as_cond().is_bool(a1)
                & !reads.line.rw.as_cond();

            *reg = MBitReg::mux(cond, &|| *read, &|| reads.line.db);
        };

        macro_rules! set_io_reg {
            ($(($reg:ident, $a0:literal, $a1:literal)),+ $(,)?) => {$(
                set_io_reg(&mut self.reg.$reg, &reads.reg.$reg, $a0, $a1);
            )+};
        }

        set_io_reg!(
            (ora, false, false),
            (orb, false, true),
            (ddra, true, false),
            (ddrb, true, true),
        );
    }

    fn update_edc_regs(&mut self, reads: &RiotAllReads) {
        let def_cond = cs_cond(reads) & reads.line.rs.as_cond() & reads.line.a[2].as_cond();

        let cond = def_cond & reads.line.rw.as_cond() & reads.line.a[0].as_cond();
        self.reg.edc_ir_flag = BitReg::mux(cond, &|| reads.reg.edc_ir_flag, &|| SingleRead::Low);

        let cond = def_cond & !reads.line.rw.as_cond() & !reads.line.a[4].as_cond();
        self.reg.edc_edge_type =
            BitReg::mux(cond, &|| reads.reg.edc_edge_type, &|| reads.line.a[0]);
    }

    fn update_timer_regs(&mut self, reads: &RiotAllReads) {
        let timer_write_cond = cs_cond(reads)
            & reads.line.rs.as_cond()
            & reads.line.a[2].as_cond()
            & !reads.line.rw.as_cond()
            & reads.line.a[4].as_cond();

        self.reg.timer = MBitReg::mux(timer_write_cond, &|| reads.reg.timer, &|| reads.line.db);

        for bit in 0..2 {
            self.reg.timer_interval[bit] =
                SingleRead::mux(timer_write_cond, &|| reads.reg.timer_interval[bit], &|| {
                    reads.line.a[bit]
                });
        }

        self.reg.sub_timer = MultiRead::mux(timer_write_cond, &|| reads.reg.sub_timer, &|| {
            Self::timer_interval_read(self.reg.timer_interval)
        });

        let def = &|| reads.reg.timer_ir_flag;
        let cond = cs_cond(reads) & reads.line.rs.as_cond() & reads.line.a[2].as_cond();
        self.reg.timer_ir_flag = BitReg::mux(cond, def, &|| {
            SingleRead::mux(
                reads.line.rw.as_cond(),
                &|| SingleRead::mux(reads.line.a[4].as_cond(), def, &|| SingleRead::Low),
                &|| SingleRead::mux(reads.line.a[0].as_cond(), &|| SingleRead::Low, def),
            )
        });
    }

    fn update_db_bus(&mut self, reads: &RiotAllReads) {
        let high_z_out = &|| BusDriveState::from_signals(&[LineSignal::HighZ; 8]);

        let ram_read = &|| {
            BusDriveState::from_multi_read(
                &reads
                    .line
                    .a
                    .iter_possible_reads()
                    .map(|addr| self.ram[usize::from(addr)])
                    .reduce(|acc, byte| acc.combine_with(&byte))
                    .expect("SingleRead will always have at least one possible read"),
            )
        };

        let ir_reg_read = &|| {
            array::from_fn(|bit| match bit {
                7 => DriveState::from(reads.reg.timer_ir_flag),
                6 => DriveState::from(reads.reg.edc_ir_flag),
                _ => DriveState::from(LineSignal::HighZ),
            })
        };

        let io_read = &|| {
            BusDriveState::mux(
                reads.line.a[0].as_cond(),
                &|| {
                    BusDriveState::mux(
                        reads.line.a[1].as_cond(),
                        &|| BusDriveState::from_multi_read(&reads.reg.ora),
                        &|| BusDriveState::from_multi_read(&reads.reg.orb),
                    )
                },
                &|| {
                    BusDriveState::mux(
                        reads.line.a[1].as_cond(),
                        &|| BusDriveState::from_multi_read(&reads.reg.ddra),
                        &|| BusDriveState::from_multi_read(&reads.reg.ddrb),
                    )
                },
            )
        };

        let misc_read = &|| {
            BusDriveState::mux(
                reads.line.a[0].as_cond(),
                &|| BusDriveState::from_multi_read(&reads.reg.timer),
                ir_reg_read,
            )
        };

        let bus_out = &|| {
            BusDriveState::mux(reads.line.rs.as_cond(), ram_read, &|| {
                BusDriveState::mux(reads.line.a[2].as_cond(), io_read, misc_read)
            })
        };

        let cond = cs_cond(reads) & reads.line.rw.as_cond();
        self.db_out = BusDriveState::mux(cond, high_z_out, bus_out);
    }

    fn update_peripherals(&mut self, reads: &RiotAllReads) {
        for (pa_line, &ddra_bit, &ora_bit) in izip!(
            self.pa_out.iter_mut(),
            reads.reg.ddra.iter(),
            reads.reg.ora.iter()
        ) {
            *pa_line = DriveState::mux(
                ddra_bit.as_cond(),
                &|| DriveState::from(LineSignal::HighZ),
                &|| DriveState::from(ora_bit),
            );
        }

        for (pb_out_state, &pb_con_index) in self.pb_out.iter_mut().zip(PB_CONNECTED_LINES.iter()) {
            *pb_out_state = DriveState::mux(
                reads.reg.ddrb[usize::from(pb_con_index)].as_cond(),
                &|| DriveState::from(LineSignal::HighZ),
                &|| DriveState::from(reads.reg.orb[usize::from(pb_con_index)]),
            );
        }
    }

    pub fn handle_rising_edge(&mut self, line_reads: RiotLineReads) {
        let mut reads = RiotAllReads::new(line_reads, self.reg.clone());

        self.update_timer(&reads);
        self.update_edc(&reads, reads.line.pa[7]);

        reads.update(self.reg.clone());
        self.update_ram_bytes(&reads);
        self.update_io_regs(&reads);
        self.update_edc_regs(&reads);
        self.update_timer_regs(&reads);

        reads.update(self.reg.clone());
        self.update_db_bus(&reads);
        self.update_peripherals(&reads);

        let new_pa7_read = reads.line.pa[7].combine_with(self.old_pa7_read);
        reads.update(self.reg.clone());
        self.update_edc(&reads, new_pa7_read);

        self.old_pa7_read = new_pa7_read;
    }

    pub fn handle_falling_edge(&mut self) {
        self.db_out = BusDriveState::from_signals(&[LineSignal::HighZ; 8]);
    }
}
