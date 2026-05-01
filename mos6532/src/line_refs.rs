use crate::RiotError;
use emutils::line::{Bus, Line};

const A_SIZE: usize = 7;
const DB_SIZE: usize = 8;
const PA_SIZE: usize = 8;
const PB_SIZE: usize = 8;

pub struct RiotLineRefs<'a> {
    pub a: &'a Bus,
    pub db: &'a mut Bus,
    pub pa: &'a mut Bus,
    pub pb: &'a mut Bus,
    pub phi2: &'a Line,
    pub res: &'a Line,
    pub cs1: &'a Line,
    pub cs2: &'a Line,
    pub rs: &'a Line,
    pub rw: &'a Line,
    pub irq: &'a mut Line,
}

impl RiotLineRefs<'_> {
    pub(crate) fn check_bus_sizes(&self) -> Result<(), RiotError> {
        for (bus, required_size) in [
            (self.a, A_SIZE),
            (self.db, DB_SIZE),
            (self.pa, PA_SIZE),
            (self.pb, PB_SIZE),
        ] {
            let actual_size = bus.size();
            if actual_size != required_size {
                return Err(RiotError::InvalidBusSize {
                    name: bus.name().to_string(),
                    required_size,
                    actual_size,
                });
            }
        }
        Ok(())
    }
}
