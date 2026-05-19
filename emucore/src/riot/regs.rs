use crate::common::read::{multi::MultiRead, single::SingleRead};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotRegs {
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

impl RiotRegs {
    pub fn new() -> Self {
        Self {
            ddra: MultiRead::from([SingleRead::Low; 8]),
            ddrb: MultiRead::from([SingleRead::Low; 8]),
            ora: MultiRead::from([SingleRead::Low; 8]),
            orb: MultiRead::from([SingleRead::Low; 8]),

            edc_ir_flag: SingleRead::Unknown,
            timer_ir_flag: SingleRead::Unknown,

            edc_edge_type: SingleRead::Low,

            timer: MultiRead::from([SingleRead::Unknown; 8]),
            sub_timer: MultiRead::from([SingleRead::Unknown; 10]),
            timer_interval: MultiRead::from([SingleRead::Unknown; 2]),
        }
    }
}
