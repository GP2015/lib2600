use crate::common::{
    read::single::SingleRead,
    reg::{multi::MBitReg, single::BitReg},
};

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
            ddra: MBitReg::from([SingleRead::Low; 8]),
            ddrb: MBitReg::from([SingleRead::Low; 8]),
            ora: MBitReg::from([SingleRead::Low; 8]),
            orb: MBitReg::from([SingleRead::Low; 8]),

            edc_ir_flag: BitReg::from(SingleRead::Unknown),
            timer_ir_flag: BitReg::from(SingleRead::Unknown),

            edc_edge_type: BitReg::from(SingleRead::Low),

            timer: MBitReg::from([SingleRead::Unknown; 8]),
            sub_timer: MBitReg::from([SingleRead::Unknown; 10]),
            timer_interval: MBitReg::from([SingleRead::Unknown; 2]),
        }
    }
}
