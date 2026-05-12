mod reads;
mod registers;

use crate::{
    common::{
        line::{
            drive_state::DriveState,
            error::LineError,
            multi::{Bus, BusConId},
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
            ram: array::from_fn(|_| MBitReg::new(SingleRead::Unknown)),
            phi2_signal: false,
            old_pa7_read: SingleRead::Unknown,
        };

        let high_z_out = [DriveState::from_only(LineSignal::HighZ); 8];
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
                    DriveState::mux(
                        &ddr_bit,
                        &|| DriveState::from_only(LineSignal::HighZ),
                        &|| DriveState::from_drive(or_bit),
                    ),
                )?;
            }
        }

        Ok(())
    }

    fn update_timer(&mut self, reads: &RiotReads) {
        let def = &|| reads.timer.clone();
        self.reg.timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| MultiRead::mux(&reads.sub_timer.is_val(0), def, &|| def().decremented()),
            &|| def().decremented(),
        ));

        let def = &|| reads.sub_timer.clone();
        let unknown = &|| MultiRead::new([SingleRead::Unknown; 10]);
        let interval_read = &|| {
            reads
                .timer_interval
                .iter_possible_reads()
                .map(|i| MultiRead::new_val(TIMER_INTERVALS[i]))
                .reduce(|acc, val| acc.combine_with(&val))
                .unwrap()
        };
        self.reg.sub_timer.set_to_read(&MultiRead::mux(
            &reads.timer_ir_flag,
            &|| {
                MultiRead::mux(&def().is_val(0), &|| def().decremented(), &|| {
                    MultiRead::mux(&reads.timer.is_val(0), interval_read, unknown)
                })
            },
            unknown,
        ));

        let def = &|| reads.timer_interval.clone();
        let unknown = &|| MultiRead::new([SingleRead::Unknown; 2]);
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

    fn cs_mux<T: HasMux>(reads: &RiotReads, def: &impl Fn() -> T, success: &impl Fn() -> T) -> T {
        T::mux(&reads.cs1, def, &|| T::mux(&reads.cs2, success, def))
    }

    fn update_ram_bytes(&mut self, reads: &RiotReads) {
        for addr in reads.a.iter_possible_reads() {
            let def = &|| self.ram[addr].read();
            self.ram[addr].set_to_read(&Self::cs_mux(reads, def, &|| {
                MultiRead::mux(
                    &reads.rs,
                    &|| MultiRead::mux(&reads.rw, &|| reads.db.clone(), def),
                    def,
                )
            }));
        }
    }

    fn io_a_branch<FnDef, FnSuccess, const A_BIT: usize, const A_STATE: bool>(
        reads: &RiotReads,
        def: &FnDef,
        success: &FnSuccess,
    ) -> MultiRead<8>
    where
        FnDef: Fn() -> MultiRead<8>,
        FnSuccess: Fn() -> MultiRead<8>,
    {
        MultiRead::mux(
            &reads.a.bit::<A_BIT>(),
            &|| {
                if A_STATE { def() } else { success() }
            },
            &|| {
                if A_STATE { success() } else { def() }
            },
        )
    }

    fn io_mux<F, const A0_STATE: bool, const A1_STATE: bool>(
        reads: &RiotReads,
        def: &F,
    ) -> MultiRead<8>
    where
        F: Fn() -> MultiRead<8>,
    {
        Self::cs_mux(reads, def, &|| {
            MultiRead::mux(
                &reads.a.bit::<2>(),
                &|| {
                    Self::io_a_branch::<_, _, 0, A0_STATE>(reads, def, &|| {
                        Self::io_a_branch::<_, _, 1, A1_STATE>(reads, def, &|| {
                            MultiRead::mux(&reads.rw, &|| reads.db.clone(), def)
                        })
                    })
                },
                def,
            )
        })
    }

    fn update_io_regs(&mut self, reads: &RiotReads) {
        macro_rules! set_io {
            ($(($reg:ident, $a0_state:literal, $a1_state:literal)),+ $(,)?) => {$(
                self.reg.$reg.set_to_read(
                    &Self::io_mux::<_, $a0_state, $a1_state>(reads, &|| {
                        reads.ora.clone()
                    })
                );
            )+};
        }

        set_io!(
            (ora, false, false),
            (orb, false, true),
            (ddra, true, false),
            (ddrb, true, true)
        );
    }

    fn handle_rising_edge(&mut self, lines: &mut RiotLines) -> Result<(), LineError> {
        let mut reads = RiotReads::new(lines, &self.reg);
        let RiotLines { db, pa, pb, .. } = lines;

        self.update_edc(&reads);
        reads.edc_ir_flag = self.reg.edc_ir_flag.read();

        self.update_ram_bytes(&reads);
        self.update_io_regs(&reads);

        self.update_timer(&reads);
        self.update_peripherals(&reads, pa, pb)?;
        self.old_pa7_read = pa.line::<7>().read();

        Ok(())
    }

    fn handle_falling_edge(&self, lines: &mut RiotLines) {
        let high_z_out = [DriveState::from_only(LineSignal::HighZ); 8];
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
