use crate::data::regtype::{BitReg, MBitReg, ValueReg};

pub struct Registers {
    pub ddra: MBitReg,
    pub ddrb: MBitReg,
    pub ora: MBitReg,
    pub orb: MBitReg,
    pub timer: ValueReg<u8>,
    pub sub_timer: ValueReg<usize>,
    pub timer_inc: ValueReg<usize>,
    pub timer_flag: BitReg,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitReg::new(8, String::from("DDRA")),
            ddrb: MBitReg::new(8, String::from("DDRB")),
            ora: MBitReg::new(8, String::from("ORA")),
            orb: MBitReg::new(8, String::from("ORB")),
            timer: ValueReg::new(String::from("Timer")),
            sub_timer: ValueReg::new(String::from("Sub-Timer")),
            timer_inc: ValueReg::new(String::from("Timer Interval")),
            timer_flag: BitReg::new(String::from("Timer Interrupt Flag")),
        }
    }
}
