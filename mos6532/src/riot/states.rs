use crate::RiotLines;
use emutils::line::{BusState, LineState};

const INITIAL_PREV_LINE_STATE: LineState = LineState::new(false, false, true);

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

impl RiotLineStates {
    pub const fn new_prev() -> Self {
        Self {
            a: BusState::new([INITIAL_PREV_LINE_STATE; 7]),
            db: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            pa: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            pb: BusState::new([INITIAL_PREV_LINE_STATE; 8]),
            phi2: INITIAL_PREV_LINE_STATE,
            res: INITIAL_PREV_LINE_STATE,
            cs1: INITIAL_PREV_LINE_STATE,
            cs2: INITIAL_PREV_LINE_STATE,
            rs: INITIAL_PREV_LINE_STATE,
            rw: INITIAL_PREV_LINE_STATE,
        }
    }
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
