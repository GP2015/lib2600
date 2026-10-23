use crate::common::{
    bit::{multi::BitStringState, single::BitState},
    line::output::{multi::BusOutput, single::LineOutput},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RiotLineReads {
    pub a: BitStringState<7>,
    pub db: BitStringState<8>,
    pub pa: BitStringState<8>,
    pub pb: [BitState; 5],
    pub cs1: BitState,
    pub cs2: BitState,
    pub rs: BitState,
    pub rw: BitState,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RiotOutput {
    pub db: BusOutput<8>,
    pub pa: BusOutput<8>,
    pub pb: [LineOutput; 5],
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TimerRegValues {
    pub timer: BitStringState<8>,
    pub sub_timer: BitStringState<10>,
    pub timer_interval: BitStringState<2>,
    pub timer_ir_flag: BitState,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct IoRegValues {
    pub ddra: BitStringState<8>,
    pub ddrb: BitStringState<8>,
    pub ora: BitStringState<8>,
    pub orb: BitStringState<8>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct EdcRegValues {
    pub edc_ir_flag: BitState,
    pub edc_edge_type: BitState,
}
