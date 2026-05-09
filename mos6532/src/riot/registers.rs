use emutils::reg::{BitReg, MBitReg};

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
            ddra: MBitReg::new("DDRA", true, false),
            ddrb: MBitReg::new("DDRB", true, false),
            ora: MBitReg::new("ORA", true, false),
            orb: MBitReg::new("ORB", true, false),

            edc_ir_flag: BitReg::new("EDC Interrupt Flag", true, true),
            timer_ir_flag: BitReg::new("Timer Interrupt Flag", true, true),

            edc_edge_type: BitReg::new("EDC Edge Type", true, false),

            timer: MBitReg::new("Timer", true, true),
            sub_timer: MBitReg::new("Sub-Timer", true, true),
            timer_interval: MBitReg::new("Timer Interval", true, true),
        }
    }
}
