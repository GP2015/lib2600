use crate::{
    common::read::{multi::MultiRead, single::SingleRead},
    riot::{core::registers::RiotRegs, lines::RiotLines},
};

pub struct RiotReads {
    pub a: MultiRead<7>,
    pub db: MultiRead<8>,
    pub pa: MultiRead<8>,
    pub pb: MultiRead<8>,
    pub cs1: SingleRead,
    pub cs2: SingleRead,
    pub rs: SingleRead,
    pub rw: SingleRead,

    pub ddra: MultiRead<8>,
    pub ddrb: MultiRead<8>,
    pub ora: MultiRead<8>,
    pub orb: MultiRead<8>,

    pub edc_ir_flag: SingleRead,
    pub timer_ir_flag: SingleRead,

    pub edc_edge_type: SingleRead,

    pub timer: MultiRead<8>,
    pub sub_timer: MultiRead<10>,
    pub timer_interval: MultiRead<2>,
}

impl RiotReads {
    pub fn new(lines: &RiotLines<'_>, regs: &RiotRegs) -> Self {
        Self {
            a: lines.a.read(),
            db: lines.db.read(),
            pa: lines.pa.read(),
            pb: lines.pb.read(),
            cs1: lines.cs1.read(),
            cs2: lines.cs2.read(),
            rs: lines.rs.read(),
            rw: lines.rw.read(),

            ddra: regs.ddra.read(),
            ddrb: regs.ddrb.read(),
            ora: regs.ora.read(),
            orb: regs.orb.read(),

            edc_ir_flag: regs.edc_ir_flag.read(),
            timer_ir_flag: regs.timer_ir_flag.read(),

            edc_edge_type: regs.edc_edge_type.read(),

            timer: regs.timer.read(),
            sub_timer: regs.sub_timer.read(),
            timer_interval: regs.timer_interval.read(),
        }
    }
}
