use crate::RiotLines;
use emutils::line::{Bus, BusState, LineState};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RiotOutputLines<'a> {
    pub db: &'a mut Bus<8>,
    pub pa: &'a mut Bus<8>,
    pub pb: &'a mut Bus<8>,
}

impl<'a> From<RiotLines<'a>> for RiotOutputLines<'a> {
    fn from(value: RiotLines<'a>) -> Self {
        Self {
            db: value.db,
            pa: value.pa,
            pb: value.pb,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotLineStates {
    pub a: BusState<7>,
    pub db: BusState<8>,
    pub pa: BusState<8>,
    pub pb: BusState<8>,
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
            cs1: value.cs1.state(),
            cs2: value.cs2.state(),
            rs: value.rs.state(),
            rw: value.rw.state(),
        }
    }
}
