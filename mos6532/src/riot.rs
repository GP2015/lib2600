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

    pub fn read_a(&self) -> Result<usize, RiotError> {
        self.buf.a.read()
    }

    pub fn read_a_bit(&self, bit: usize) -> Result<bool, RiotError> {
        self.buf.a.read_bit(bit)
    }

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
    pub fn a_written(&self) -> bool {
        self.buf.a.is_written()
    }

    pub fn a_bit_written(&self, bit: usize) -> Result<bool, RiotError> {
        self.buf.a.is_bit_written(bit)
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
    pub fn db_written(&self) -> bool {
        self.buf.db.is_written()
    }

    pub fn db_bit_written(&self, bit: u8) -> bool {
        self.buf.db.is_bit_written(bit as usize).unwrap()
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

    pub fn pa_written(&self) -> bool {
        self.buf.pa.is_written()
    }

    pub fn pa_bit_written(&self, bit: u8) -> bool {
        self.buf.pa.is_bit_written(bit as usize).unwrap()
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

    pub fn pb_written(&self) -> bool {
        self.buf.pb.is_written()
    }

    pub fn pb_bit_written(&self, bit: u8) -> bool {
        self.buf.pb.is_bit_written(bit as usize).unwrap()
    }

    // Other pin operations

    /// Pulse the input clock pin (PHI2).
    pub fn pulse_phi2(&mut self) -> Result<(), RiotError> {
        self.tick()
    }

    pub fn read_cs1(&self) -> Result<bool, RiotError> {
        self.buf.cs1.read()
    }

    /// Drive the Chip Select 1 pin with state `state`.
    pub fn write_cs1(&mut self, state: bool) {
        self.buf.cs1.write(state)
    }

    /// Returns true if the Chip Select 1 pin is being driven.
    pub fn cs1_written(&self) -> bool {
        self.buf.cs1.is_written()
    }

    pub fn read_cs2(&self) -> Result<bool, RiotError> {
        self.buf.cs2.read()
    }

    /// Drive the Chip Select 2 pin with state `state`.
    pub fn write_cs2(&mut self, state: bool) {
        self.buf.cs2.write(state)
    }

    /// Returns true if the Chip Select 2 pin is being driven.
    pub fn cs2_written(&self) -> bool {
        self.buf.cs2.is_written()
    }

    pub fn read_rw(&self) -> Result<bool, RiotError> {
        self.buf.rw.read()
    }

    /// Drive the Read/Write pin with state `state`.
    pub fn write_rw(&mut self, state: bool) {
        self.buf.rw.write(state)
    }

    /// Returns true if the Read/Write pin is being driven.
    pub fn rw_written(&self) -> bool {
        self.buf.rw.is_written()
    }

    pub fn read_res(&self) -> Result<bool, RiotError> {
        self.buf.res.read()
    }

    /// Drive the Reset pin with state `state`.
    pub fn write_res(&mut self, state: bool) {
        self.buf.res.write(state)
    }

    /// Returns true if the Reset pin is being driven.
    pub fn res_written(&self) -> bool {
        self.buf.res.is_written()
    }

    pub fn read_rs(&self) -> Result<bool, RiotError> {
        self.buf.rs.read()
    }

    /// Drive the Ram Select pin with state `state`.
    pub fn write_rs(&mut self, state: bool) {
        self.buf.rs.write(state)
    }

    /// Returns true if the Ram Select pin is being driven.
    pub fn rs_written(&self) -> bool {
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
    pub fn irq_written(&self) -> bool {
        self.buf.irq.is_written()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn riot() -> Riot {
        Riot::new()
    }

    #[rstest]
    fn read_write_a(mut riot: Riot) {
        assert!(riot.read_a().is_err());
        assert!(!riot.a_written());
        riot.write_a(0x45).unwrap();
        assert_eq!(riot.read_a().unwrap(), 0x45);
        assert!(riot.a_written());
    }

    #[rstest]
    fn write_a_out_of_bounds(mut riot: Riot) {
        assert!(riot.write_a(0x80).is_err());
    }

    #[rstest]
    fn write_a_wrap(mut riot: Riot) {
        riot.write_a_wrap(0xFF);
        assert_eq!(riot.read_a().unwrap(), 0x7F);
    }

    #[rstest]
    fn read_write_a_bit(mut riot: Riot) {
        assert!(riot.read_a_bit(4).is_err());
        assert!(!riot.a_bit_written(4).unwrap());
        riot.write_a_bit(4, true).unwrap();
        assert!(riot.read_a_bit(4).unwrap());
        assert!(riot.a_bit_written(4).unwrap());
    }

    #[rstest]
    fn read_write_a_bit_out_of_bounds(mut riot: Riot) {
        assert!(riot.write_a_bit(7, true).is_err());
        riot.write_a_wrap(0xFF);
        assert!(riot.read_a_bit(7).is_err());
    }

    type ReadU8BusFn = fn(&Riot) -> Result<u8, RiotError>;
    type WriteU8BusFn = fn(&mut Riot, u8);
    type U8BusWrittenFn = fn(&Riot) -> bool;

    #[rstest]
    #[case(Riot::read_db, Riot::write_db, Riot::db_written)]
    #[case(Riot::read_pa, Riot::write_pa, Riot::pa_written)]
    #[case(Riot::read_pb, Riot::write_pb, Riot::pb_written)]
    fn read_write_u8_bus(
        mut riot: Riot,
        #[case] read: ReadU8BusFn,
        #[case] write: WriteU8BusFn,
        #[case] written: U8BusWrittenFn,
    ) {
        assert!(read(&riot).is_err());
        assert!(!written(&riot));
        write(&mut riot, 0x67);
        assert_eq!(read(&riot).unwrap(), 0x67);
        assert!(written(&riot));
    }

    type ReadU8BusBitFn = fn(&Riot, u8) -> Result<bool, RiotError>;
    type WriteU8BusBitFn = fn(&mut Riot, u8, bool);
    type U8BusBitWrittenFn = fn(&Riot, u8) -> bool;

    #[rstest]
    #[case(Riot::read_db_bit, Riot::write_db_bit, Riot::db_bit_written)]
    #[case(Riot::read_pa_bit, Riot::write_pa_bit, Riot::pa_bit_written)]
    #[case(Riot::read_pb_bit, Riot::write_pb_bit, Riot::pb_bit_written)]
    fn read_write_u8_bus_bit(
        mut riot: Riot,
        #[case] read: ReadU8BusBitFn,
        #[case] write: WriteU8BusBitFn,
        #[case] written: U8BusBitWrittenFn,
        #[values(false, true)] state: bool,
    ) {
        assert!(read(&riot, 6).is_err());
        assert!(!written(&riot, 6));
        write(&mut riot, 6, state);
        assert_eq!(read(&riot, 6).unwrap(), state);
        assert!(written(&riot, 6));
    }

    type ReadPinFn = fn(&Riot) -> Result<bool, RiotError>;
    type WritePinFn = fn(&mut Riot, bool);
    type PinWrittenFn = fn(&Riot) -> bool;

    #[rstest]
    #[case(Riot::read_res, Riot::write_res, Riot::res_written)]
    #[case(Riot::read_rw, Riot::write_rw, Riot::rw_written)]
    #[case(Riot::read_rs, Riot::write_rs, Riot::rs_written)]
    #[case(Riot::read_cs1, Riot::write_cs1, Riot::cs1_written)]
    #[case(Riot::read_cs2, Riot::write_cs2, Riot::cs2_written)]
    fn read_write_pin(
        mut riot: Riot,
        #[case] read: ReadPinFn,
        #[case] write: WritePinFn,
        #[case] written: PinWrittenFn,
        #[values(false, true)] state: bool,
    ) {
        assert!(read(&riot).is_err());
        assert!(!written(&riot));
        write(&mut riot, state);
        assert_eq!(read(&riot).unwrap(), state);
        assert!(written(&riot));
    }
}
