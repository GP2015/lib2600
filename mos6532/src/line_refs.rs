use emutils::line::{Bus, Line};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RiotLineRefs<'a> {
    pub a: &'a Bus<7>,
    pub db: &'a mut Bus<8>,
    pub pa: &'a mut Bus<8>,
    pub pb: &'a mut Bus<8>,
    pub phi2: &'a Line,
    pub res: &'a Line,
    pub cs1: &'a Line,
    pub cs2: &'a Line,
    pub rs: &'a Line,
    pub rw: &'a Line,
    pub irq: &'a mut Line,
}
