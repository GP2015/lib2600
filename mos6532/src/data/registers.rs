mod bit;
mod mbit;
mod val;

use crate::data::registers::{bit::BitReg, mbit::MBitReg, val::ValueReg};

pub struct Registers {
    pub ddra: MBitReg,
    pub ddrb: MBitReg,
    pub ora: MBitReg,
    pub orb: MBitReg,

    pub edc_use_pos_edge: BitReg,
    pub edc_enable_irq: BitReg,
    pub edc_interrupt_flag: BitReg,
    pub old_pa7: BitReg,

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

            edc_use_pos_edge: BitReg::new(String::from("Edge-Detect Polarity")),
            edc_enable_irq: BitReg::new(String::from("Edge-Detect IRQ Control")),
            edc_interrupt_flag: BitReg::new(String::from("Edge-Detect Interrupt Flag")),
            old_pa7: BitReg::new(String::from("Old PA7 State")),

            timer: ValueReg::new(String::from("Timer")),
            sub_timer: ValueReg::new(String::from("Sub-Timer")),
            timer_inc: ValueReg::new(String::from("Timer Interval")),
            timer_flag: BitReg::new(String::from("Timer Interrupt Flag")),
        }
    }
}
