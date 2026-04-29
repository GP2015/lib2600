use emutils::line::{Bus, Line};

pub struct RiotLineRefs<'a> {
    pub a: &'a Bus,
    pub db: &'a mut Bus,
    pub pa: &'a mut Bus,
    pub pb: &'a mut Bus,
    pub res: &'a Line,
    pub cs1: &'a Line,
    pub cs2: &'a Line,
    pub rs: &'a Line,
    pub rw: &'a Line,
    pub irq: &'a mut Line,
}
