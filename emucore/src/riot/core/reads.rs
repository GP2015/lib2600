use crate::{
    common::read::{multi::MultiRead, single::SingleRead},
    riot::{core::registers::RiotRegs, lines::RiotLineReads},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotReads {
    pub a: MultiRead<7>,
    pub db: MultiRead<8>,
    pub pa: MultiRead<8>,

    pub pb0: SingleRead,
    pub pb1: SingleRead,
    pub pb3: SingleRead,
    pub pb6: SingleRead,
    pub pb7: SingleRead,

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
    pub fn new(line_reads: &RiotLineReads, regs: &RiotRegs) -> Self {
        Self {
            a: line_reads.a,
            db: line_reads.db,
            pa: line_reads.pa,

            pb0: line_reads.pb0,
            pb1: line_reads.pb1,
            pb3: line_reads.pb3,
            pb6: line_reads.pb6,
            pb7: line_reads.pb7,

            cs1: line_reads.cs1,
            cs2: line_reads.cs2,
            rs: line_reads.rs,
            rw: line_reads.rw,

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

    pub fn update(&mut self, regs: &RiotRegs) {
        self.ddra = regs.ddra.read();
        self.ddrb = regs.ddrb.read();
        self.ora = regs.ora.read();
        self.orb = regs.orb.read();
        self.edc_ir_flag = regs.edc_ir_flag.read();
        self.timer_ir_flag = regs.timer_ir_flag.read();
        self.edc_edge_type = regs.edc_edge_type.read();
        self.timer = regs.timer.read();
        self.sub_timer = regs.sub_timer.read();
        self.timer_interval = regs.timer_interval.read();
    }
}
