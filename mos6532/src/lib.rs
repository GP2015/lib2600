#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]

mod clk_cycle;
mod control;
mod error;
mod line_refs;
mod riot;

pub use crate::{clk_cycle::ClkCycle, error::RiotError, line_refs::RiotLineRefs, riot::Riot};
