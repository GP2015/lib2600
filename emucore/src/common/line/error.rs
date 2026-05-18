use thiserror::Error;

use crate::common::line::ident::LineIdent;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum LineError {
    #[error("line {bus_name}{bit} does not exist")]
    BitOutOfRange {
        bus_name: &'static str,
        bit: usize,
        size: usize,
    },

    #[error("cannot drive value {value} to {bus_name} bus without wrapping")]
    DriveValueTooLarge {
        bus_name: &'static str,
        value: usize,
        size: usize,
    },

    #[error("cannot perform operation on line {ident} without causing a short circuit")]
    ShortCircuit { ident: LineIdent },

    #[error("cannot read line {ident} as it has no possible signal through it")]
    ImpossibleLineSignal { ident: LineIdent },
}
