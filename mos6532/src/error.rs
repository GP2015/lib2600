use thiserror::Error;

#[derive(Error, Debug)]
pub enum RiotError {
    #[error("short circuit has occurred in MOS6532 chip")]
    ShortCircuit,
}
