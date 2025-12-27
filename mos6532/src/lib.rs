mod bus;
mod error;

use bus::Bus;
use error::RIOTError;

struct Pins {
    a: Bus,
    pa: Bus,
    pb: Bus,
    irq: bool,
    db: Bus,
    res: bool,
    rw: bool,
    rs: bool,
    cs2: bool,
    cs1: bool,
    phi2: bool,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            a: Bus::new(0, 7),
            db: Bus::new(0, 8),
            pa: Bus::new(0, 8),
            pb: Bus::new(0, 8),
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

pub struct RIOT {
    pin: Pins,
}

impl RIOT {
    pub fn new() -> Self {
        Self { pin: Pins::new() }
    }

    // Address Bus operations

    pub fn drv_a(&mut self, val: usize) -> Result<(), RIOTError> {
        self.pin.a.drive(val)
    }

    pub fn drv_a_wrap(&mut self, val: usize) {
        self.pin.a.drive_wrap(val);
    }

    pub fn drv_a_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.pin.a.drive_bit(bit, state)
    }

    // Data Bus operations

    pub fn rd_db(&self) -> usize {
        self.pin.db.read()
    }

    pub fn rd_db_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.pin.db.read_bit(bit)
    }

    pub fn drv_db(&mut self, val: usize) -> Result<(), RIOTError> {
        self.pin.db.drive(val)
    }

    pub fn drv_db_wrap(&mut self, val: usize) {
        self.pin.db.drive_wrap(val);
    }

    pub fn drv_db_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.pin.db.drive_bit(bit, state)
    }

    // Peripheral Data A operations

    pub fn rd_pa(&self) -> usize {
        self.pin.pa.read()
    }

    pub fn rd_pa_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.pin.pa.read_bit(bit)
    }

    pub fn drv_pa(&mut self, val: usize) -> Result<(), RIOTError> {
        self.pin.pa.drive(val)
    }

    pub fn drv_pa_wrap(&mut self, val: usize) {
        self.pin.pa.drive_wrap(val);
    }

    pub fn drv_pa_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.pin.pa.drive_bit(bit, state)
    }

    // Peripheral Data B operations

    pub fn rd_pb(&self) -> usize {
        self.pin.pb.read()
    }

    pub fn rd_pb_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.pin.pb.read_bit(bit)
    }

    pub fn drv_pb(&mut self, val: usize) -> Result<(), RIOTError> {
        self.pin.pb.drive(val)
    }

    pub fn drv_pb_wrap(&mut self, val: usize) {
        self.pin.pb.drive_wrap(val);
    }

    pub fn drv_pb_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.pin.pb.drive_bit(bit, state)
    }

    // Other pin operations

    pub fn drv_cs1(&mut self, state: bool) {
        self.pin.cs1 = state;
    }

    pub fn drv_cs2(&mut self, state: bool) {
        self.pin.cs2 = state;
    }

    pub fn drv_phi2(&mut self, state: bool) {
        self.pin.phi2 = state;
    }

    pub fn drv_rw(&mut self, state: bool) {
        self.pin.rw = state;
    }

    pub fn drv_res(&mut self, state: bool) {
        self.pin.res = state;
    }

    pub fn drv_rs(&mut self, state: bool) {
        self.pin.rs = state;
    }

    pub fn rd_irq(&self) -> bool {
        self.pin.irq
    }
}
