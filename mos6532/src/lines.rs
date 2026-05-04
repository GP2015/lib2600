use emutils::line::{Bus, Line, LineError};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct RiotLines<'a> {
    pub db: &'a mut Bus<8>,
    pub pa: &'a mut Bus<8>,
    pub pb: &'a mut Bus<8>,

    pub a: &'a Bus<7>,
    pub cs1: &'a Line,
    pub cs2: &'a Line,
    pub rs: &'a Line,
    pub rw: &'a Line,
}

impl RiotLines<'_> {
    pub(crate) fn check_possible(&self) -> Result<(), LineError> {
        self.db.check_possible()?;
        self.pa.check_possible()?;
        self.pb.check_possible()?;

        self.a.check_possible()?;
        self.cs1.check_possible()?;
        self.cs2.check_possible()?;
        self.rs.check_possible()?;
        self.rw.check_possible()
    }
}
