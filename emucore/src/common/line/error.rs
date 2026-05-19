use crate::common::line::ident::LineIdent;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum LineError {
    #[error("cannot perform operation on line {ident} without causing a short circuit")]
    ShortCircuit { ident: LineIdent },

    #[error("cannot read line {ident} as it has no possible signal through it")]
    ImpossibleLineSignal { ident: LineIdent },
}
