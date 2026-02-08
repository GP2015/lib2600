use emu_utils::register::{BitRegister, MBitRegister, ValueRegister};

pub struct Registers {
    pub ddra: MBitRegister,
    pub ddrb: MBitRegister,
    pub ora: MBitRegister,
    pub orb: MBitRegister,

    pub edc_use_pos_edge: BitRegister,
    pub edc_enable_irq: BitRegister,
    pub edc_interrupt_flag: BitRegister,

    pub timer: ValueRegister<u8>,
    pub sub_timer: ValueRegister<usize>,
    pub timer_inc: ValueRegister<usize>,
    pub timer_flag: BitRegister,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitRegister::new(8, String::from("DDRA")),
            ddrb: MBitRegister::new(8, String::from("DDRB")),
            ora: MBitRegister::new(8, String::from("ORA")),
            orb: MBitRegister::new(8, String::from("ORB")),

            edc_use_pos_edge: BitRegister::new(String::from("Edge-Detect Polarity")),
            edc_enable_irq: BitRegister::new(String::from("Edge-Detect IRQ Control")),
            edc_interrupt_flag: BitRegister::new(String::from("Edge-Detect Interrupt Flag")),

            timer: ValueRegister::new(String::from("Timer")),
            sub_timer: ValueRegister::new(String::from("Sub-Timer")),
            timer_inc: ValueRegister::new(String::from("Timer Interval")),
            timer_flag: BitRegister::new(String::from("Timer Interrupt Flag")),
        }
    }
}
