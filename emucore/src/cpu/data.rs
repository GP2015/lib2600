use crate::{BusOutput, LineOutput};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct CpuOutput {
    pub phi2: LineOutput,
    pub a: BusOutput<13>,
    pub db: BusOutput<8>,
    pub rw: LineOutput,
}
