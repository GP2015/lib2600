use thiserror::Error;

#[derive(Error, Debug)]
pub enum RIOTError {
    #[error("the specified bit ({0}) does not exist")]
    InvalidBit(usize),

    #[error("the provided value ({0}) cannot fit in the bus")]
    ValueTooLarge(usize),
}
