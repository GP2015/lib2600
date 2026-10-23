use crate::common::line::output::{multi::BusOutput, single::LineOutput};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct InputDrives {
    pub a: BusOutput<13>,
    pub db: BusOutput<8>,
    pub inp1: BusOutput<7>,
    pub inp2: BusOutput<7>,
    pub rdiff: LineOutput,
    pub ldiff: LineOutput,
    pub col: LineOutput,
    pub sel: LineOutput,
    pub res: LineOutput,
}

pub struct OutputDrives {
    pub a: BusOutput<13>,
    pub db: BusOutput<8>,
    pub inp1: BusOutput<7>,
    pub inp2: BusOutput<7>,
    pub rdiff: LineOutput,
    pub ldiff: LineOutput,
    pub col: LineOutput,
    pub sel: LineOutput,
    pub res: LineOutput,
}
