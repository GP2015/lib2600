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
        macro_rules! check_valid {
            ($($p:ident),+) => {$(
                self.$p.check_valid()?;
            )+};
        }

        check_valid!(a, db, pa, pb, cs1, cs2, rs, rw);
        Ok(())
    }
}
