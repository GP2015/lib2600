use emutils::reg::MBitRegister;

pub struct Registers {
    pub ddra: MBitRegister,
    pub ddrb: MBitRegister,
    pub ora: MBitRegister,
    pub orb: MBitRegister,
    // pub edc_use_pos_edge: BitRegister,
    // pub edc_enable_irq: BitRegister,
    // pub edc_interrupt_flag: BitRegister,
    // pub timer: ValueRegister<u8>,
    // pub sub_timer: ValueRegister<usize>,
    // pub timer_inc: ValueRegister<usize>,
    // pub timer_flag: BitRegister,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitRegister::new("DDRA", 8),
            ddrb: MBitRegister::new("DDRB", 8),
            ora: MBitRegister::new("ORA", 8),
            orb: MBitRegister::new("ORB", 8),
            // edc_use_pos_edge: BitRegister::new("Edge-Detect Polarity"),
            // edc_enable_irq: BitRegister::new("Edge-Detect IRQ Control"),
            // edc_interrupt_flag: BitRegister::new("Edge-Detect Interrupt Flag"),
            // timer: ValueRegister::new("Timer"),
            // sub_timer: ValueRegister::new("Sub-Timer"),
            // timer_inc: ValueRegister::new("Timer Interval"),
            // timer_flag: BitRegister::new("Timer Interrupt Flag"),
        }
    }
}
