use strum::Display;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum LineIdent {
    A(u8),
    Db(u8),
    Inp1(u8),
    Inp2(u8),
    RDiff,
    LDiff,
    Col,
    Sel,
    Res,

    Phi2,
    Rw,
}

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
    #[error("failed to perform operation on line {0} due to short circuit")]
    Short(LineIdent),

    #[error("failed to read line {0} due to no possible signal flowing through it")]
    NoSignal(LineIdent),
}
