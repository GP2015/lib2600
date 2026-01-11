use crate::RiotError;
use crate::data::{Buffers, Ram, Registers};

// NOTE TO SELF:

// You should include info on the registers and their initialisation

// You should also include info on the output read operations,
// namely what inputs must be done to ensure those output pins are not uninitialised.

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

/// An emulated MOS 6532 RIOT chip.
///
/// Each of the pins/buses must be individually driven to control the chip.
/// Methods are provided for this.
///
/// To avoid emulating non-deterministic behaviour,
/// all pins start in an uninitialised state.
/// All inputs must be driven with some value before the chip can use them internally
/// and all outputs must be driven by the chip internally before they can be read.
/// Violations of this rule will return some [`RiotError`] error variant.
///
/// Similarly, all internal registers start in an uninitialised state
/// and must be driven with some value before they can be read.
pub struct Riot {
    pub(super) buf: Buffers,
    pub(super) reg: Registers,
    pub(super) ram: Ram,
}

impl Default for Riot {
    fn default() -> Self {
        Self::new()
    }
}

/// The core methods used to control the RIOT chip.
/// Each of the pins & buses are driven individually.
impl Riot {
    /// Create a new MOS 6532 RIOT chip, with all pins and registers uninitialised.
    pub fn new() -> Self {
        Self {
            buf: Buffers::new(),
            reg: Registers::new(),
            ram: Ram::new(),
        }
    }

    fn usize_res_to_u8_res<E>(res: Result<usize, E>) -> Result<u8, E> {
        match res {
            Ok(val) => Ok(u8::try_from(val).unwrap()),
            Err(e) => Err(e),
        }
    }

    // Address Bus operations

    /// Drive the address bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RiotError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the address bus without wrapping.
    pub fn write_a(&mut self, val: usize) -> Result<(), RiotError> {
        self.buf.a.write(val)
    }

    /// Drive the address bus with the value `val`,
    /// wrapping if necessary.
    pub fn write_a_wrap(&mut self, val: usize) {
        self.buf.a.write_wrap(val);
    }

    /// Drive bit `bit` of the address bus with state `state`.
    ///
    /// Returns a [`RiotError::MBitRegBitOutOfRange`]
    /// if the address bus has no bit `bit`.
    pub fn write_a_bit(&mut self, bit: usize, state: bool) -> Result<(), RiotError> {
        self.buf.a.write_bit(bit, state)
    }

    /// Returns true if the address bus is being driven with a value.
    pub fn a_driven(&self) -> bool {
        self.buf.a.is_written()
    }

    // Data Bus operations

    /// Read the value on the data bus.
    ///
    /// Returns a [`RiotError::UninitialisedMBitRegBit`]
    /// if any of the bits on the data bus are still uninitialised.
    pub fn read_db(&self) -> Result<u8, RiotError> {
        Self::usize_res_to_u8_res(self.buf.db.read())
    }

    /// Read bit `bit` of the data bus.
    ///
    /// Returns a [`RiotError::MBitRegBitOutOfRange`]
    /// if the data bus has no bit `bit`.
    ///
    /// Returns a [`RiotError::UninitialisedMBitRegBit`]
    /// if any of the bits on the data bus are still uninitialised.
    pub fn read_db_bit(&self, bit: u8) -> Result<bool, RiotError> {
        self.buf.db.read_bit(bit as usize)
    }

    /// Drive the data bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RiotError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the data bus without wrapping.
    pub fn write_db(&mut self, val: u8) {
        self.buf.db.write(val as usize).unwrap();
    }

    /// Drive bit `bit` of the data bus with state `state`.
    ///
    /// Returns a [`RiotError::MBitRegBitOutOfRange`]
    /// if the data bus has no bit `bit`.
    pub fn write_db_bit(&mut self, bit: u8, state: bool) {
        self.buf.db.write_bit(bit as usize, state).unwrap()
    }

    /// Returns true if the data bus is being driven with a value.
    pub fn db_driven(&self) -> bool {
        self.buf.db.is_written()
    }

    // Peripheral A Data operations

    pub fn read_pa(&self) -> Result<u8, RiotError> {
        Self::usize_res_to_u8_res(self.buf.pa.read())
    }

    pub fn read_pa_bit(&self, bit: u8) -> Result<bool, RiotError> {
        self.buf.pa.read_bit(bit as usize)
    }

    pub fn write_pa(&mut self, val: u8) {
        self.buf.pa.write(val as usize).unwrap();
    }

    pub fn write_pa_bit(&mut self, bit: u8, state: bool) {
        self.buf.pa.write_bit(bit as usize, state).unwrap()
    }

    pub fn pa_driven(&self) -> bool {
        self.buf.pa.is_written()
    }

    // Peripheral B Data operations

    pub fn read_pb(&self) -> Result<u8, RiotError> {
        Self::usize_res_to_u8_res(self.buf.pb.read())
    }

    pub fn read_pb_bit(&self, bit: u8) -> Result<bool, RiotError> {
        self.buf.pb.read_bit(bit as usize)
    }

    pub fn write_pb(&mut self, val: u8) {
        self.buf.pb.write(val as usize).unwrap();
    }

    pub fn write_pb_bit(&mut self, bit: u8, state: bool) {
        self.buf.pb.write_bit(bit as usize, state).unwrap()
    }

    pub fn pb_driven(&self) -> bool {
        self.buf.pb.is_written()
    }

    // Other pin operations

    /// Pulse the input clock pin (PHI2).
    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.tick()
    }

    /// Drive the Chip Select 1 pin with state `state`.
    pub fn write_cs1(&mut self, state: bool) {
        self.buf.cs1.write(state)
    }

    /// Returns true if the Chip Select 1 pin is being driven.
    pub fn cs1_driven(&self) -> bool {
        self.buf.cs1.is_written()
    }

    /// Drive the Chip Select 2 pin with state `state`.
    pub fn write_cs2(&mut self, state: bool) {
        self.buf.cs2.write(state)
    }

    /// Returns true if the Chip Select 2 pin is being driven.
    pub fn cs2_driven(&self) -> bool {
        self.buf.cs2.is_written()
    }

    /// Drive the Read/Write pin with state `state`.
    pub fn write_rw(&mut self, state: bool) {
        self.buf.rw.write(state)
    }

    /// Returns true if the Read/Write pin is being driven.
    pub fn rw_driven(&self) -> bool {
        self.buf.rw.is_written()
    }

    /// Drive the Reset pin with state `state`.
    pub fn write_res(&mut self, state: bool) {
        self.buf.res.write(state)
    }

    /// Returns true if the Reset pin is being driven.
    pub fn res_driven(&self) -> bool {
        self.buf.res.is_written()
    }

    /// Drive the Ram Select pin with state `state`.
    pub fn write_rs(&mut self, state: bool) {
        self.buf.rs.write(state)
    }

    /// Returns true if the Ram Select pin is being driven.
    pub fn rs_driven(&self) -> bool {
        self.buf.rs.is_written()
    }

    /// Read the Interrupt Request pin.
    ///
    /// Returns a [`RiotError::UninitialisedBitReg`]
    /// if the pin is still uninitialised.
    pub fn read_irq(&self) -> Result<bool, RiotError> {
        self.buf.irq.read()
    }

    /// Returns true if the Interrupt Request pin is being driven.
    pub fn irq_driven(&self) -> bool {
        self.buf.irq.is_written()
    }
}
