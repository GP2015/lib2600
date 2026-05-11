use crate::{
    common::{
        line::{bus::state::BusState, single::state::LineState},
        reg::{bit::state::BitRegState, mbit::state::MBitRegState},
    },
    riot::{core::registers::RiotRegs, lines::RiotLines},
};

pub struct RiotStates {
    pub a: BusState<7>,
    pub db: BusState<8>,
    pub pb: BusState<8>,
    pub cs1: LineState,
    pub cs2: LineState,
    pub rs: LineState,
    pub rw: LineState,

    pub ddra: MBitRegState<8>,
    pub ddrb: MBitRegState<8>,
    pub ora: MBitRegState<8>,
    pub orb: MBitRegState<8>,

    pub edc_ir_flag: BitRegState,
    pub timer_ir_flag: BitRegState,
}

impl RiotStates {
    pub fn new(lines: &RiotLines<'_>, regs: &RiotRegs) -> Self {
        Self {
            a: lines.a.state(),
            db: lines.db.state(),
            pb: lines.pb.state(),
            cs1: lines.cs1.state(),
            cs2: lines.cs2.state(),
            rs: lines.rs.state(),
            rw: lines.rw.state(),

            ddra: regs.ddra.state(),
            ddrb: regs.ddrb.state(),
            ora: regs.ora.state(),
            orb: regs.orb.state(),
            edc_ir_flag: regs.edc_ir_flag.state(),
            timer_ir_flag: regs.timer_ir_flag.state(),
        }
    }
}
