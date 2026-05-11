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
            ddra: MBitReg::new(SingleRead::Low),
            ddrb: MBitReg::new(SingleRead::Low),
            ora: MBitReg::new(SingleRead::Low),
            orb: MBitReg::new(SingleRead::Low),

            edc_ir_flag: BitReg::new(SingleRead::Unknown),
            timer_ir_flag: BitReg::new(SingleRead::Unknown),

            edc_edge_type: BitReg::new(SingleRead::Low),

            timer: MBitReg::new(SingleRead::Unknown),
            sub_timer: MBitReg::new(SingleRead::Unknown),
            timer_interval: MBitReg::new(SingleRead::Unknown),
        }
    }
}
