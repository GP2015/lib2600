use emutils::line::{BusConnectionId, LineConnectionId};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RiotConnectionIds {
    pub db: BusConnectionId,
    pub pa: BusConnectionId,
    pub pb: BusConnectionId,
    pub irq: LineConnectionId,
}
