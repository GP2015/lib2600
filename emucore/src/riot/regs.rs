use crate::common::reg::{BitReg, MBitReg};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotRegs {
    pub ddra: MBitReg<8>,
    pub ddrb: MBitReg<8>,
    pub ora: MBitReg<8>,
    pub orb: MBitReg<8>,

    pub edc_ir_flag: BitReg,
    pub timer_ir_flag: BitReg,

    pub edc_edge_type: BitReg,

    pub timer: MBitReg<8>,
    pub sub_timer: MBitReg<10>,
    pub timer_interval: MBitReg<2>,
}

impl RiotRegs {
    pub fn new() -> Self {
        Self {
            ddra: [BitReg::Low; _].into(),
            ddrb: [BitReg::Low; _].into(),
            ora: [BitReg::Low; _].into(),
            orb: [BitReg::Low; _].into(),

            edc_ir_flag: BitReg::Unknown,
            timer_ir_flag: BitReg::Unknown,

            edc_edge_type: BitReg::Low,

            timer: [BitReg::Unknown; _].into(),
            sub_timer: [BitReg::Unknown; _].into(),
            timer_interval: [BitReg::Unknown; _].into(),
        }
    }
}
