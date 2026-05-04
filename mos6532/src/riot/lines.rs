use crate::RiotLines;
use emutils::line::Bus;

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
