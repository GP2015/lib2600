use emutils::{
    line::{BusState, LineState},
    reg::{BitRegister, MBitRegister},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Registers {
    pub ddra: MBitRegister<8>,
    pub ddrb: MBitRegister<8>,
    pub ora: MBitRegister<8>,
    pub orb: MBitRegister<8>,

    pub edc_ir_flag: BitRegister,
    pub timer_ir_flag: BitRegister,

    pub edc_edge_type: BitRegister,

    pub timer: MBitRegister<8>,
    pub sub_timer: MBitRegister<10>,
    pub timer_interval: MBitRegister<2>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ddra: MBitRegister::new("DDRA", true, false),
            ddrb: MBitRegister::new("DDRB", true, false),
            ora: MBitRegister::new("ORA", true, false),
            orb: MBitRegister::new("ORB", true, false),

            edc_ir_flag: BitRegister::new("EDC Interrupt Flag", true, true),
            timer_ir_flag: BitRegister::new("Timer Interrupt Flag", true, true),

            edc_edge_type: BitRegister::new("EDC Edge Type", true, false),

            timer: MBitRegister::new("Timer", true, true),
            sub_timer: MBitRegister::new("Sub-Timer", true, true),
            timer_interval: MBitRegister::new("Timer Interval", true, true),
        }
    }
}

#[derive(Clone, Copy)]
pub enum BitRegId {
    EdcIrFlag,
    TimerIrFlag,
    EdcEdgeType,
}

#[derive(Clone, Copy)]
pub enum MBitRegId {
    Ddra,
    Ddrb,
    Ora,
    Orb,
    Timer,
    SubTimer,
    TimerInterval,
}

pub enum RegChangeId {
    BitRegAdd(BitRegId, bool),
    BitRegFromLine(BitRegId, LineState),
    MBitRegAdd(MBitRegId, usize),
    MBitRegDecrement(MBitRegId),
    MBitRegFromBus(MBitRegId, BusState<8>),
}

#[derive(Default)]
pub struct RegisterChanges {
    inner: Vec<Vec<RegChangeId>>,
}

impl RegisterChanges {
    pub fn add_new_option(&mut self) {
        self.inner.push(Vec::new());
    }

    pub fn push_to_newest(&mut self, change: RegChangeId) {
        self.inner.last_mut().unwrap().push(change);
    }
}
