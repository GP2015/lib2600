mod reads;
mod registers;

use crate::{
    common::{
        line::{
            error::LineError,
            multi::{Bus, BusConId, state::BusDriveState},
            single::state::DriveState,
        },
        mux::HasMux,
        read::{multi::MultiRead, single::SingleRead},
        reg::multi::MBitReg,
        signal::LineSignal,
    },
    riot::{
        core::{reads::RiotReads, registers::RiotRegs},
        lines::RiotLines,
    },
};
use itertools::izip;
use std::array;

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
    db_con: BusConId,
    pa_con: BusConId,
    pb_con: BusConId,
    reg: RiotRegs,
    ram: [MBitReg<8>; RAM_SIZE],
    phi2_signal: bool,
    old_pa7_read: SingleRead,
}

impl Riot {
    pub fn new(db: &mut Bus<8>, pa: &mut Bus<8>, pb: &mut Bus<8>) -> Self {
        let riot = Self {
            db_con: db.create_connection(),
            pa_con: pa.create_connection(),
            pb_con: pb.create_connection(),
            reg: RiotRegs::new(),
            ram: array::from_fn(|_| MBitReg::from([SingleRead::Unknown; 8])),
            phi2_signal: false,
            old_pa7_read: SingleRead::Unknown,
        };

        let high_z_out = BusDriveState::from([LineSignal::HighZ; 8]);
        pa.set_drive_state(riot.pa_con, &high_z_out).unwrap();
        pb.set_drive_state(riot.pb_con, &high_z_out).unwrap();

        riot
    }

    fn update_edc(&mut self, reads: &RiotReads) {
        let def = &|| reads.edc_ir_flag;
        let new_pa7 = &reads.pa.bit::<7>();
        self.reg.edc_ir_flag.set_to_read(SingleRead::mux(
            &reads.edc_edge_type,
            &|| {
                SingleRead::mux(&self.old_pa7_read, def, &|| {
                    SingleRead::mux(new_pa7, &|| SingleRead::High, def)
                })
            },
            &|| {
                SingleRead::mux(
                    &self.old_pa7_read,
                    &|| SingleRead::mux(new_pa7, def, &|| SingleRead::High),
                    def,
                )
            },
        ));
    }

    fn timer_interval_read(read: &MultiRead<2>) -> MultiRead<10> {
        read.iter_possible_reads()
            .map(|i| MultiRead::from(TIMER_INTERVALS[i]))
            .reduce(|acc, val| acc.combine_with(&val))
            .unwrap()
    }

    fn update_timer(&mut self, reads: &RiotReads) {
        let def = &|| reads.timer.clone();
        self.reg.timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| MultiRead::mux(&reads.sub_timer.is_val(0), def, &|| def().decremented()),
            &|| def().decremented(),
        ));

        let def = &|| reads.sub_timer.clone();
        let unknown = &|| MultiRead::from([SingleRead::Unknown; 10]);
        self.reg.sub_timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| {
                MultiRead::mux(&def().is_val(0), &|| def().decremented(), &|| {
                    MultiRead::mux(
                        &reads.timer.is_val(0),
                        &|| Self::timer_interval_read(&reads.timer_interval),
                        unknown,
                    )
                })
            },
            unknown,
        ));

        let def = &|| reads.timer_interval.clone();
        let unknown = &|| MultiRead::from([SingleRead::Unknown; 2]);
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
                &|| reads.db.clone(),
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
            &|| reads.a.bit::<0>(),
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

        self.reg.timer.set_to_read(&only_on_timer_write!(
            MultiRead,
            &|| reads.timer.clone(),
            &|| reads.db.clone(),
        ));

        macro_rules! set_timer_interval {
            ($($bit:literal),+) => {$(
                self.reg
                    .timer_interval
                    .bit_mut::<$bit>()
                    .set_to_read(only_on_timer_write!(
                        SingleRead,
                        &|| reads.timer_interval.bit::<$bit>(),
                        &|| reads.a.bit::<$bit>(),
                    ));
            )+};
        }

        set_timer_interval!(1, 0);

        self.reg.sub_timer.set_to_read(&only_on_timer_write!(
            MultiRead,
            &|| reads.sub_timer.clone(),
            &|| Self::timer_interval_read(&self.reg.timer_interval.read())
        ));

        let def = &|| reads.timer_ir_flag;
        self.reg.timer_ir_flag.set_to_read(only_on_cs!(
            reads,
            SingleRead,
            def,
            &|| SingleRead::mux(
                &reads.rw,
                &|| SingleRead::mux(&reads.a.bit::<4>(), def, &|| SingleRead::Low),
                &|| SingleRead::mux(&reads.a.bit::<0>(), &|| SingleRead::Low, def),
            ),
            (&reads.rs, true),
            (&reads.a.bit::<2>(), true),
        ));
    }

    fn update_db_bus(&self, reads: &RiotReads, db: &mut Bus<8>) -> Result<(), LineError> {
        let high_z_out = &|| BusDriveState::from([LineSignal::HighZ; 8]);

        let ram_read = &|| {
            BusDriveState::from(
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
                &reads.a.bit::<0>(),
                &|| {
                    BusDriveState::mux(
                        &reads.a.bit::<1>(),
                        &|| BusDriveState::from(&reads.ora),
                        &|| BusDriveState::from(&reads.orb),
                    )
                },
                &|| {
                    BusDriveState::mux(
                        &reads.a.bit::<1>(),
                        &|| BusDriveState::from(&reads.ddra),
                        &|| BusDriveState::from(&reads.ddrb),
                    )
                },
            )
        };

        let misc_read = &|| {
            BusDriveState::mux(
                &reads.a.bit::<0>(),
                &|| BusDriveState::from(&reads.timer),
                ir_reg_read,
            )
        };

        let bus_out = &|| {
            BusDriveState::mux(&reads.rs, ram_read, &|| {
                BusDriveState::mux(&reads.a.bit::<2>(), io_read, misc_read)
            })
        };

        db.set_drive_state(
            self.db_con,
            &only_on_cs!(reads, BusDriveState, high_z_out, bus_out, (reads.rw, true)),
        )
    }

    fn update_peripherals(
        &self,
        reads: &RiotReads,
        pa: &mut Bus<8>,
        pb: &mut Bus<8>,
    ) -> Result<(), LineError> {
        for (p, bus_con, ddr, or) in [
            (pa, self.pa_con, &reads.ddra, &reads.ora),
            (pb, self.pb_con, &reads.ddrb, &reads.orb),
        ] {
            for ((p_line, line_con), ddr_bit, or_bit) in
                izip!(p.try_iter_mut(bus_con)?, ddr.iter(), or.iter())
            {
                p_line.set_drive_state(
                    line_con,
                    DriveState::mux(&ddr_bit, &|| DriveState::from(LineSignal::HighZ), &|| {
                        DriveState::from(or_bit)
                    }),
                )?;
            }
        }

        Ok(())
    }

    fn handle_rising_edge(&mut self, lines: &mut RiotLines) -> Result<(), LineError> {
        let mut reads = RiotReads::new(lines, &self.reg);
        let RiotLines { db, pa, pb, .. } = lines;

        self.update_timer(&reads);
        self.update_edc(&reads);

        reads.update(&self.reg);
        self.update_ram_bytes(&reads);
        self.update_io_regs(&reads);
        self.update_edc_regs(&reads);
        self.update_timer_regs(&reads);

        reads.update(&self.reg);
        self.update_db_bus(&reads, db)?;
        self.update_peripherals(&reads, pa, pb)?;

        reads.update(&self.reg);
        self.update_edc(&reads);

        self.old_pa7_read = pa.line::<7>().read();

        Ok(())
    }

    fn handle_falling_edge(&self, lines: &mut RiotLines) {
        let high_z_out = BusDriveState::from([LineSignal::HighZ; 8]);
        lines.db.set_drive_state(self.db_con, &high_z_out).unwrap();
    }

    pub fn drive_phi2(
        &mut self,
        lines: &mut RiotLines,
        bool_signal: bool,
    ) -> Result<(), LineError> {
        lines.check_valid()?;

        match (self.phi2_signal, bool_signal) {
            (false, true) => self.handle_rising_edge(lines)?,
            (true, false) => self.handle_falling_edge(lines),
            _ => return Ok(()),
        }

        self.phi2_signal = bool_signal;
        Ok(())
    }
}
