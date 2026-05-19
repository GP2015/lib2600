use crate::common::{BitReg, MBitReg};

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
    pub const fn new() -> Self {
        Self {
            ddra: [BitReg::Low; 8],
            ddrb: [BitReg::Low; 8],
            ora: [BitReg::Low; 8],
            orb: [BitReg::Low; 8],

            edc_ir_flag: BitReg::Unknown,
            timer_ir_flag: BitReg::Unknown,

            edc_edge_type: BitReg::Low,

            timer: [BitReg::Unknown; 8],
            sub_timer: [BitReg::Unknown; 10],
            timer_interval: [BitReg::Unknown; 2],
        }
    }
}
