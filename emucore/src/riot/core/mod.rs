mod reads;
mod registers;

use crate::{
    common::{
        line::{
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        mux::HasMux,
        read::{
            multi::{IsMultiRead, MultiRead},
            single::SingleRead,
        },
        reg::multi::MBitReg,
        signal::LineSignal,
    },
    riot::{
        core::{reads::RiotReads, registers::RiotRegs},
        lines::RiotLineReads,
    },
};
use core::array;
use itertools::izip;

const RAM_SIZE: usize = 128;
const TIMER_INTERVALS: [usize; 4] = [1, 8, 64, 1024];

macro_rules! only_on_cond {
    ($has_mux:ident, $def:expr, $success:expr, ($is_cond:expr, $cond_res:expr) $(,)?) => {
        if $cond_res {
            $has_mux::mux($is_cond, $def, $success)
        } else {
            $has_mux::mux($is_cond, $success, $def)
        }
    };

    (
        $has_mux:ident,
        $def:expr,
        $success:expr,
        ($is_cond:expr, $cond_res:expr),
        $(($is_cond_v:expr, $cond_res_v:expr)),+ $(,)?
    ) => {
        only_on_cond!(
            $has_mux,
            $def,
            &|| only_on_cond!(
                $has_mux,
                $def,
                $success,
                $(($is_cond_v, $cond_res_v)),+
            ),
            ($is_cond, $cond_res),
        )
    };
}

macro_rules! only_on_cs {
    (
        $reads:expr,
        $has_mux:ident,
        $def:expr,
        $success:expr,
        $(($is_cond:expr, $cond_res:expr)),+ $(,)?
    ) => {
        only_on_cond!(
            $has_mux,
            $def,
            $success,
            (&$reads.cs1, true),
            (&$reads.cs2, false),
        )
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Riot {
    pub db_out: BusDriveState<8>,
    pub pa_out: BusDriveState<8>,
    pub p0_out: DriveState,
    pub p1_out: DriveState,
    pub p3_out: DriveState,
    pub p6_out: DriveState,
    pub p7_out: DriveState,

    reg: RiotRegs,
    ram: [MBitReg<8>; RAM_SIZE],
    phi2_signal: bool,
    old_pa7_read: SingleRead,
}

impl Default for Riot {
    fn default() -> Self {
        Self {
            db_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            pa_out: BusDriveState::from_signals(&[LineSignal::HighZ; _]),
            p0_out: DriveState::from(LineSignal::HighZ),
            p1_out: DriveState::from(LineSignal::HighZ),
            p3_out: DriveState::from(LineSignal::HighZ),
            p6_out: DriveState::from(LineSignal::HighZ),
            p7_out: DriveState::from(LineSignal::HighZ),
            reg: RiotRegs::new(),
            ram: array::from_fn(|_| MBitReg::from([SingleRead::Unknown; 8])),
            phi2_signal: false,
            old_pa7_read: SingleRead::Unknown,
        }
    }
}

impl Riot {
    fn update_edc(&mut self, reads: &RiotReads, pa7_read: SingleRead) {
        let def = &|| reads.edc_ir_flag;
        self.reg.edc_ir_flag.set_to_read(SingleRead::mux(
            &reads.edc_edge_type,
            &|| {
                SingleRead::mux(&self.old_pa7_read, def, &|| {
                    SingleRead::mux(&pa7_read, &|| SingleRead::High, def)
                })
            },
            &|| {
                SingleRead::mux(
                    &self.old_pa7_read,
                    &|| SingleRead::mux(&pa7_read, def, &|| SingleRead::High),
                    def,
                )
            },
        ));
    }

    fn timer_interval_read(read: MultiRead<2>) -> MultiRead<10> {
        read.iter_possible_reads()
            .map(|i| MultiRead::from_usize(TIMER_INTERVALS[i]))
            .reduce(|acc, val| acc.combine_with(&val))
            .unwrap()
    }

    fn update_timer(&mut self, reads: &RiotReads) {
        let def = &|| reads.timer;
        self.reg.timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| MultiRead::mux(&reads.sub_timer.is_val(0), def, &|| def().decremented()),
            &|| def().decremented(),
        ));

        let def = &|| reads.sub_timer;
        let unknown = &|| [SingleRead::Unknown; 10];
        self.reg.sub_timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| {
                MultiRead::mux(&def().is_val(0), &|| def().decremented(), &|| {
                    MultiRead::mux(
                        &reads.timer.is_val(0),
                        &|| Self::timer_interval_read(reads.timer_interval),
                        unknown,
                    )
                })
            },
            unknown,
        ));

        let def = &|| reads.timer_interval;
        let unknown = &|| [SingleRead::Unknown; 2];
        self.reg.timer_interval.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| {
                MultiRead::mux(&reads.sub_timer.is_val(0), def, &|| {
                    MultiRead::mux(&reads.timer.is_val(0), def, unknown)
                })
            },
            unknown,
        ));

        let def = &|| reads.timer_ir_flag;
        self.reg.timer_ir_flag.set_to_read(SingleRead::mux(
            &def(),
            &|| {
                SingleRead::mux(&reads.sub_timer.is_val(0), &|| SingleRead::Low, &|| {
                    SingleRead::mux(&reads.timer.is_val(0), &|| SingleRead::Low, &|| {
                        SingleRead::High
                    })
                })
            },
            &|| SingleRead::High,
        ));
    }

    fn update_ram_bytes(&mut self, reads: &RiotReads) {
        for addr in reads.a.iter_possible_reads() {
            self.ram[addr].set_to_read(&only_on_cs!(
                reads,
                MultiRead,
                &|| self.ram[addr].read(),
                &|| reads.db,
                (&reads.rs, false),
                (&reads.rw, false),
            ));
        }
    }

    fn update_io_regs(&mut self, reads: &RiotReads) {
        macro_rules! set_io_reg {
            ($(($reg:ident, $a0:literal, $a1:literal)),+ $(,)?) => {$(
                self.reg.$reg.set_to_read(
                    &only_on_cs!(
                        reads,
                        MultiRead,
                        &|| reads.$reg.clone(),
                        &|| reads.db.clone(),
                        (&reads.rs, true),
                        (&reads.a.bit::<2>(), false),
                        (&reads.a.bit::<0>(), $a0),
                        (&reads.a.bit::<1>(), $a1),
                        (&reads.rw, false),
                    )
                );
            )+};
        }

        set_io_reg!(
            (ora, false, false),
            (orb, false, true),
            (ddra, true, false),
            (ddrb, true, true)
        );
    }

    fn update_edc_regs(&mut self, reads: &RiotReads) {
        macro_rules! set_edc_reg {
            ($reg:ident, $success:expr, $(($is_cond:expr, $cond_res:expr)),+ $(,)?) => {
                self.reg.$reg.set_to_read(
                    only_on_cs!(
                        reads,
                        SingleRead,
                        &|| reads.edc_ir_flag,
                        $success,
                        (&reads.rs, true),
                        (&reads.a.bit::<2>(), true),
                        $(($is_cond, $cond_res)),+
                    )
                );
            };
        }

        set_edc_reg!(
            edc_ir_flag,
            &|| SingleRead::Low,
            (&reads.rw, true),
            (&reads.a.bit::<0>(), true),
        );

        set_edc_reg!(
            edc_edge_type,
            &|| reads.a[0],
            (&reads.rw, false),
            (&reads.a.bit::<4>(), false),
        );
    }

    fn update_timer_regs(&mut self, reads: &RiotReads) {
        macro_rules! only_on_timer_write {
            ($has_mux:ident, $def:expr, $success:expr $(,)?) => {
                only_on_cs!(
                    reads,
                    $has_mux,
                    $def,
                    $success,
                    (&reads.rs, true),
                    (&reads.a.bit::<2>(), true),
                    (&reads.rw, false),
                    (&reads.a.bit::<4>(), true),
                )
            };
        }

        self.reg
            .timer
            .set_to_read(&only_on_timer_write!(MultiRead, &|| reads.timer, &|| reads.db));

        macro_rules! set_timer_interval {
            ($($bit:literal),+) => {$(
                self.reg
                    .timer_interval[$bit]
                    .set_to_read(only_on_timer_write!(
                        SingleRead,
                        &|| reads.timer_interval[$bit],
                        &|| reads.a[$bit],
                    ));
            )+};
        }

        set_timer_interval!(1, 0);

        self.reg.sub_timer.set_to_read(&only_on_timer_write!(
            MultiRead,
            &|| reads.sub_timer,
            &|| Self::timer_interval_read(self.reg.timer_interval.read())
        ));

        let def = &|| reads.timer_ir_flag;
        self.reg.timer_ir_flag.set_to_read(only_on_cs!(
            reads,
            SingleRead,
            def,
            &|| SingleRead::mux(
                &reads.rw,
                &|| SingleRead::mux(&reads.a[4], def, &|| SingleRead::Low),
                &|| SingleRead::mux(&reads.a[0], &|| SingleRead::Low, def),
            ),
            (&reads.rs, true),
            (&reads.a.bit::<2>(), true),
        ));
    }

    fn update_db_bus(&mut self, reads: &RiotReads) {
        let high_z_out = &|| BusDriveState::from_signals(&[LineSignal::HighZ; 8]);

        let ram_read = &|| {
            BusDriveState::from_multi_read(
                &reads
                    .a
                    .iter_possible_reads()
                    .map(|addr| self.ram[addr].read())
                    .reduce(|acc, byte| acc.combine_with(&byte))
                    .unwrap(),
            )
        };

        let ir_reg_read = &|| {
            BusDriveState::from(array::from_fn(|bit| match bit {
                7 => DriveState::from(reads.timer_ir_flag),
                6 => DriveState::from(reads.edc_ir_flag),
                _ => DriveState::from(LineSignal::HighZ),
            }))
        };

        let io_read = &|| {
            BusDriveState::mux(
                &reads.a[0],
                &|| {
                    BusDriveState::mux(
                        &reads.a[1],
                        &|| BusDriveState::from_multi_read(&reads.ora),
                        &|| BusDriveState::from_multi_read(&reads.orb),
                    )
                },
                &|| {
                    BusDriveState::mux(
                        &reads.a[1],
                        &|| BusDriveState::from_multi_read(&reads.ddra),
                        &|| BusDriveState::from_multi_read(&reads.ddrb),
                    )
                },
            )
        };

        let misc_read = &|| {
            BusDriveState::mux(
                &reads.a[0],
                &|| BusDriveState::from_multi_read(&reads.timer),
                ir_reg_read,
            )
        };

        let bus_out = &|| {
            BusDriveState::mux(&reads.rs, ram_read, &|| {
                BusDriveState::mux(&reads.a[2], io_read, misc_read)
            })
        };

        self.db_out = only_on_cs!(reads, BusDriveState, high_z_out, bus_out, (reads.rw, true));
    }

    fn update_peripherals(&mut self, reads: &RiotReads) {
        for (p_line, &ddr_bit, &or_bit) in
            izip!(self.pa_out.iter_mut(), reads.ddra.iter(), reads.ora.iter())
        {
            *p_line = DriveState::mux(&ddr_bit, &|| DriveState::from(LineSignal::HighZ), &|| {
                DriveState::from(or_bit)
            });
        }

        todo!("Update PB");
    }

    fn handle_rising_edge(&mut self, line_reads: &RiotLineReads) {
        let mut reads = RiotReads::new(line_reads, &self.reg);

        self.update_timer(&reads);
        self.update_edc(&reads, reads.pa[7]);

        reads.update(&self.reg);
        self.update_ram_bytes(&reads);
        self.update_io_regs(&reads);
        self.update_edc_regs(&reads);
        self.update_timer_regs(&reads);

        reads.update(&self.reg);
        self.update_db_bus(&reads);
        self.update_peripherals(&reads);

        let new_pa7_read = reads.pa[7].combine_with(self.old_pa7_read);
        reads.update(&self.reg);
        self.update_edc(&reads, new_pa7_read);

        self.old_pa7_read = new_pa7_read;
    }

    fn handle_falling_edge(&mut self) {
        self.db_out = BusDriveState::from_signals(&[LineSignal::HighZ; 8]);
    }

    pub fn drive_phi2(&mut self, line_reads: &RiotLineReads, bool_signal: bool) {
        match (self.phi2_signal, bool_signal) {
            (false, true) => self.handle_rising_edge(line_reads),
            (true, false) => self.handle_falling_edge(),
            _ => (),
        }

        self.phi2_signal = bool_signal;
    }
}
