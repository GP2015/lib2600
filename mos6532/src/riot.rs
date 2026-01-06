use crate::RIOTError;
use crate::data::{Buffers, RAM, Registers};

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
/// Violations of this rule will return some [`RIOTError`] error variant.
///
/// Similarly, all internal registers start in an uninitialised state
/// and must be driven with some value before they can be read.
pub struct RIOT {
    pub(super) buf: Buffers,
    pub(super) reg: Registers,
    pub(super) ram: RAM,
}

/// The core methods used to control the RIOT chip.
/// Each of the pins & buses are driven individually.
impl RIOT {
    /// Create a new MOS 6532 RIOT chip, with all pins and registers uninitialised.
    pub fn new() -> Self {
        Self {
            buf: Buffers::new(),
            reg: Registers::new(),
            ram: RAM::new(),
        }
    }

    // Address Bus operations

    /// Drive the address bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RIOTError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the address bus without wrapping.
    pub fn drive_a(&mut self, val: usize) -> Result<(), RIOTError> {
        self.buf.a.drive(val)
    }

    /// Drive the address bus with the value `val`,
    /// wrapping if necessary.
    pub fn drive_a_wrap(&mut self, val: usize) {
        self.buf.a.drive_wrap(val);
    }

    /// Drive bit `bit` of the address bus with state `state`.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the address bus has no bit `bit`.
    pub fn drive_a_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.buf.a.drive_bit(bit, state)
    }

    /// Returns true if the address bus is being driven with a value.
    pub fn a_driven(&self) -> bool {
        self.buf.a.is_driven()
    }

    // Data Bus operations

    /// Read the value on the data bus.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the data bus are still uninitialised.
    pub fn read_db(&self) -> Result<usize, RIOTError> {
        self.buf.db.read()
    }

    /// Read bit `bit` of the data bus.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the data bus has no bit `bit`.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the data bus are still uninitialised.
    pub fn read_db_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.buf.db.read_bit(bit)
    }

    /// Drive the data bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RIOTError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the data bus without wrapping.
    pub fn drive_db(&mut self, val: usize) -> Result<(), RIOTError> {
        self.buf.db.drive(val)
    }

    /// Drive the data bus with the value `val`,
    /// wrapping if necessary.
    pub fn drive_db_wrap(&mut self, val: usize) {
        self.buf.db.drive_wrap(val);
    }

    /// Drive bit `bit` of the data bus with state `state`.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the data bus has no bit `bit`.
    pub fn drive_db_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.buf.db.drive_bit(bit, state)
    }

    /// Returns true if the data bus is being driven with a value.
    pub fn db_driven(&self) -> bool {
        self.buf.db.is_driven()
    }

    // Peripheral A Data operations

    /// Read the value on the Peripheral A data bus.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the Peripheral A data bus are still uninitialised.
    pub fn read_pa(&self) -> Result<usize, RIOTError> {
        self.buf.pa.read()
    }

    /// Read bit `bit` of the Peripheral A data bus.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the Peripheral A data bus has no bit `bit`.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the Peripheral A data bus are still uninitialised.
    pub fn read_pa_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.buf.pa.read_bit(bit)
    }

    /// Drive the Peripheral A data bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RIOTError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the Peripheral A data bus without wrapping.
    pub fn drive_pa(&mut self, val: usize) -> Result<(), RIOTError> {
        self.buf.pa.drive(val)
    }

    /// Drive the Peripheral A data bus with the value `val`,
    /// wrapping if necessary.
    pub fn drive_pa_wrap(&mut self, val: usize) {
        self.buf.pa.drive_wrap(val);
    }

    /// Drive bit `bit` of the Peripheral A data bus with state `state`.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the Peripheral A data bus has no bit `bit`.
    pub fn drive_pa_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.buf.pa.drive_bit(bit, state)
    }

    /// Returns true if the Peripheral A data bus is being driven with a value.
    pub fn pa_driven(&self) -> bool {
        self.buf.pa.is_driven()
    }

    // Peripheral B Data operations

    /// Read the value on the Peripheral B data bus.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the Peripheral B data bus are still uninitialised.
    pub fn read_pb(&self) -> Result<usize, RIOTError> {
        self.buf.pb.read()
    }

    /// Read bit `bit` of the Peripheral B data bus.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the Peripheral B data bus has no bit `bit`.
    ///
    /// Returns a [`RIOTError::UninitialisedMBitRegBit`]
    /// if any of the bits on the Peripheral B data bus are still uninitialised.
    pub fn read_pb_bit(&self, bit: usize) -> Result<bool, RIOTError> {
        self.buf.pb.read_bit(bit)
    }

    /// Drive the Peripheral B data bus with the value `val`,
    /// without wrapping.
    ///
    /// Returns a [`RIOTError::MBitRegDriveValueTooLarge`]
    /// if `val` cannot fit in the Peripheral B data bus without wrapping.
    pub fn drive_pb(&mut self, val: usize) -> Result<(), RIOTError> {
        self.buf.pb.drive(val)
    }

    /// Drive the Peripheral B data bus with the value `val`,
    /// wrapping if necessary.
    pub fn drive_pb_wrap(&mut self, val: usize) {
        self.buf.pb.drive_wrap(val);
    }

    /// Drive bit `bit` of the Peripheral B data bus with state `state`.
    ///
    /// Returns a [`RIOTError::MBitRegBitOutOfRange`]
    /// if the Peripheral B data bus has no bit `bit`.
    pub fn drive_pb_bit(&mut self, bit: usize, state: bool) -> Result<(), RIOTError> {
        self.buf.pb.drive_bit(bit, state)
    }

    /// Returns true if the Peripheral B data bus is being driven with a value.
    pub fn pb_driven(&self) -> bool {
        self.buf.pb.is_driven()
    }

    // Other pin operations

    /// Pulse the input clock pin (PHI2).
    pub fn pulse_phi2(&mut self) -> Result<(), RIOTError> {
        self.tick()
    }

    /// Drive the Chip Select 1 pin with state `state`.
    pub fn drive_cs1(&mut self, state: bool) {
        self.buf.cs1.drive(state)
    }

    /// Returns true if the Chip Select 1 pin is being driven.
    pub fn cs1_driven(&self) -> bool {
        self.buf.cs1.is_driven()
    }

    /// Drive the Chip Select 2 pin with state `state`.
    pub fn drive_cs2(&mut self, state: bool) {
        self.buf.cs2.drive(state)
    }

    /// Returns true if the Chip Select 2 pin is being driven.
    pub fn cs2_driven(&self) -> bool {
        self.buf.cs2.is_driven()
    }

    /// Drive the Read/Write pin with state `state`.
    pub fn drive_rw(&mut self, state: bool) {
        self.buf.rw.drive(state)
    }

    /// Returns true if the Read/Write pin is being driven.
    pub fn rw_driven(&self) -> bool {
        self.buf.rw.is_driven()
    }

    /// Drive the Reset pin with state `state`.
    pub fn drive_res(&mut self, state: bool) {
        self.buf.res.drive(state)
    }

    /// Returns true if the Reset pin is being driven.
    pub fn res_driven(&self) -> bool {
        self.buf.res.is_driven()
    }

    /// Drive the Ram Select pin with state `state`.
    pub fn drive_rs(&mut self, state: bool) {
        self.buf.rs.drive(state)
    }

    /// Returns true if the Ram Select pin is being driven.
    pub fn rs_driven(&self) -> bool {
        self.buf.rs.is_driven()
    }

    /// Read the Interrupt Request pin.
    ///
    /// Returns a [`RIOTError::UninitialisedBitReg`]
    /// if the pin is still uninitialised.
    pub fn read_irq(&self) -> Result<bool, RIOTError> {
        self.buf.irq.read()
    }

    /// Returns true if the Interrupt Request pin is being driven.
    pub fn irq_driven(&self) -> bool {
        self.buf.irq.is_driven()
    }
}
