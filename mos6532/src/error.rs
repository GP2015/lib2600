use thiserror::Error;

#[derive(Error, Debug)]
pub enum RIOTError {
    #[error("pin is uninitialised")]
    UninitialisedPin,

    #[error("cannot read bit {bit} of {bus_size}-bit bus")]
    BusBitOutOfRange { bit: usize, bus_size: usize },

    #[error("cannot drive value {value} to a {bus_size}-bit bus without wrapping")]
    BusDriveValueTooLarge { value: usize, bus_size: usize },

    #[error("bit {bit} of bus is uninitialised")]
    UninitialisedBusBit { bit: usize },
}
