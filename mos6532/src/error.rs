use thiserror::Error;

/// Error indicating some failure within the emulated MOS 6532 RIOT chip.
#[derive(Error, Debug)]
pub enum RIOTError {
    /// Indicates an attempt to access a pin's value before the pin was initialised.
    ///
    /// Pins must first be driven with a particular value
    /// before their value can be accessed (internally or externally).
    #[error("pin is uninitialised")]
    UninitialisedPin,

    /// Indicates an attempt to read/drive an invalid pin of a bus.
    ///
    /// Each bus has a predefined number of bits/pins,
    /// so attempting to use a non-existent pin will return this error.
    #[error("cannot use bit {bit} of {bus_size}-bit bus")]
    BusBitOutOfRange {
        /// The bit/pin in question (where the least-significant bit would be 0).
        bit: usize,

        /// The predefined number of bits in the bus.
        bus_size: usize,
    },

    /// Indicates an attempt to drive a value to a bus which cannot fit cleanly without wrapping.
    ///
    /// Each bus has a predefined number of bits,
    /// so if the value being driven requires more bits to represent than the bus has,
    /// this error will be returned when wrapping is not used.
    #[error("cannot drive value {value} to a {bus_size}-bit bus without wrapping")]
    BusDriveValueTooLarge {
        /// The value attempting to be driven.
        value: usize,

        /// The predefined number of bits in the bus.
        bus_size: usize,
    },

    /// Indicates an attempt to access a bus pin's value before the pin was initialised.
    ///
    /// Bus pins must first be driven with a particular value
    /// before their value can be accessed (internally or externally).
    ///
    /// Only pins being accessed need to be initialised.
    /// Other pins in the bus can be left uninitialised without this error being returned.
    #[error("bit {bit} of bus is uninitialised")]
    UninitialisedBusBit {
        /// The bit/pin in question (where the least-significant bit would be 0).
        bit: usize,
    },
}
