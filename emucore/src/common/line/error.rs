use strum::Display;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum LineIdent {
    #[strum(to_string = "{name}")]
    UniqueLine { name: &'static str },

    #[strum(to_string = "{bus_name} bit {bit}")]
    BusLine { bus_name: &'static str, bit: usize },
}

impl From<&'static str> for LineIdent {
    fn from(value: &'static str) -> Self {
        Self::UniqueLine { name: value }
    }
}

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum LineError {
    #[error("cannot perform operation on line {ident} without causing a short circuit")]
    ShortCircuit { ident: LineIdent },

    #[error("cannot read line {ident} as it has no possible signal through it")]
    ImpossibleLineSignal { ident: LineIdent },
}
