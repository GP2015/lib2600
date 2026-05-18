use crate::common::read::{multi::MultiRead, single::SingleRead};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RiotLineReads {
    pub db: MultiRead<8>,
    pub pa: MultiRead<8>,

    pub pb0: SingleRead,
    pub pb1: SingleRead,
    pub pb3: SingleRead,
    pub pb6: SingleRead,
    pub pb7: SingleRead,

    pub a: MultiRead<7>,
    pub cs1: SingleRead,
    pub cs2: SingleRead,
    pub rs: SingleRead,
    pub rw: SingleRead,
}
