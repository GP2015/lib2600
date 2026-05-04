use crate::RiotLines;
use emutils::line::{BusState, LineState};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotLineStates {
    pub a: BusState<7>,
    pub db: BusState<8>,
    pub pa: BusState<8>,
    pub pb: BusState<8>,
    pub phi2: LineState,
    pub res: LineState,
    pub cs1: LineState,
    pub cs2: LineState,
    pub rs: LineState,
    pub rw: LineState,
}

impl From<&RiotLines<'_>> for RiotLineStates {
    fn from(value: &RiotLines<'_>) -> Self {
        Self {
            a: value.a.state(),
            db: value.db.state(),
            pa: value.pa.state(),
            pb: value.pb.state(),
            phi2: value.phi2.state(),
            res: value.res.state(),
            cs1: value.cs1.state(),
            cs2: value.cs2.state(),
            rs: value.rs.state(),
            rw: value.rw.state(),
        }
    }
}
