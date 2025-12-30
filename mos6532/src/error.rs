use thiserror::Error;

// NOTE TO SELF:

// These comments are all wrong.

// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

/// Error indicating some failure within the emulated MOS 6532 RIOT chip.
#[derive(Error, Debug)]
pub enum RIOTError {
    /// Indicates an attempt to access a pin's value before the pin was initialised.
    ///
    /// Pins must first be driven with a particular value
    /// before their value can be accessed (internally or externally).
    #[error("cannot access {reg_name} as it is uninitialised")]
    UninitialisedBitReg {
        /// The name of the pin in question.
        reg_name: String,
    },

    /// Indicates an attempt to read/drive an invalid pin of a bus.
    ///
    /// Each bus has a predefined number of bits/pins,
    /// so attempting to use a non-existent pin will return this error.
    #[error("pin {reg_name}{bit} does not exist")]
    MBitRegBitOutOfRange {
        /// The name of the bus in question.
        reg_name: String,

        /// The bit/pin in question (where the least-significant bit would be 0).
        bit: usize,

        /// The predefined number of bits in the bus.
        reg_size: usize,
    },

    /// Indicates an attempt to drive a value to a bus which cannot fit cleanly without wrapping.
    ///
    /// Each bus has a predefined number of bits,
    /// so if the value being driven requires more bits to represent than the bus has,
    /// this error will be returned when wrapping is not used.
    #[error("cannot drive value {value} to {reg_name} bus without wrapping")]
    MBitRegDriveValueTooLarge {
        /// The name of the bus in question.
        reg_name: String,

        /// The value attempting to be driven.
        value: usize,

        /// The predefined number of bits in the bus.
        reg_size: usize,
    },

    /// Indicates an attempt to access a bus pin's value before the pin was initialised.
    ///
    /// Bus pins must first be driven with a particular value
    /// before their value can be accessed (internally or externally).
    ///
    /// Only pins being accessed need to be initialised.
    /// Other pins in the bus can be left uninitialised without this error being returned.
    #[error("cannot access {reg_name}{bit} as it is uninitialised ")]
    UninitialisedMBitRegBit {
        /// The name of the bus in question.
        reg_name: String,

        /// The bit/pin in question (where the least-significant bit would be 0).
        bit: usize,

        /// The predefined number of bits in the bus.
        reg_size: usize,
    },

    /// Indicates an attempt to read a byte from RAM before the byte was initialised.
    ///
    /// Bytes of RAM must first have a value written to them before they can be read.
    #[error("cannot read RAM byte at {address:x} as it is uninitialised ")]
    UninitialisedRAMByte {
        /// The address being read from.
        address: usize,
    },
}
