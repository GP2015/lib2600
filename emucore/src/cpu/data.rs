use crate::common::line::output::{multi::BusOutput, single::LineOutput};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CpuOutput {
    pub a: BusOutput<13>,
    pub db: BusOutput<8>,
    pub rw: LineOutput,
}
