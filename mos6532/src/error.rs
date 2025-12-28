use thiserror::Error;

#[derive(Error, Debug)]
pub enum RIOTError {
    #[error("the specified bit ({0}) does not exist")]
    BusBitOutOfRange(usize),

    #[error("the provided value ({0}) cannot fit in the bus")]
    BusDriveValueTooLarge(usize),

    #[error("bit {0} of bus is uninitialised")]
    UninitialisedBusBit(usize),
}
