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
    pub(crate) fn check_valid(&self) -> Result<(), LineError> {
        self.db.check_valid()?;
        self.pa.check_valid()?;
        self.pb.check_valid()?;

        self.a.check_valid()?;
        self.cs1.check_valid()?;
        self.cs2.check_valid()?;
        self.rs.check_valid()?;
        self.rw.check_valid()
    }
}
