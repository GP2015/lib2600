use crate::bus::Bus;

pub struct Pins {
    pub a: Bus,
    pub pa: Bus,
    pub pb: Bus,
    pub irq: bool,
    pub db: Bus,
    pub res: bool,
    pub rw: bool,
    pub rs: bool,
    pub cs2: bool,
    pub cs1: bool,
    pub phi2: bool,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            a: Bus::new(7),
            db: Bus::new(8),
            pa: Bus::new(8),
            pb: Bus::new(8),
            cs1: false,
            cs2: false,
            phi2: false,
            rw: false,
            res: false,
            rs: false,
            irq: false,
        }
    }
}
