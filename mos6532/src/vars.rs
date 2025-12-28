use crate::bus::Bus;
use crate::pin::Pin;

pub struct Pins {
    pub a: Bus,
    pub pa: Bus,
    pub pb: Bus,
    pub irq: Pin,
    pub db: Bus,
    pub res: Pin,
    pub rw: Pin,
    pub rs: Pin,
    pub cs2: Pin,
    pub cs1: Pin,
    pub phi2: Pin,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            a: Bus::new(7),
            db: Bus::new(8),
            pa: Bus::new(8),
            pb: Bus::new(8),
            cs1: Pin::new(),
            cs2: Pin::new(),
            phi2: Pin::new(),
            rw: Pin::new(),
            res: Pin::new(),
            rs: Pin::new(),
            irq: Pin::new(),
        }
    }
}
