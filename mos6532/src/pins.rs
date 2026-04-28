use emutils::line::{Bus, BusConnection, Line};

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

pub struct RiotLineInitRefs<'a> {
    pub db: &'a mut Bus,
    pub pa: &'a mut Bus,
    pub pb: &'a mut Bus,
}

pub struct PinConnections {
    pub db: BusConnection,
    pub pa: BusConnection,
    pub pb: BusConnection,
}

impl PinConnections {
    pub(crate) fn new(inits: &mut RiotLineInitRefs) -> Self {
        Self {
            db: inits.db.create_connection(),
            pa: inits.pa.create_connection(),
            pb: inits.pb.create_connection(),
        }
    }
}
