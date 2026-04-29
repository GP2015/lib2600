#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]

mod control;
mod error;
mod refs;
mod riot;

pub use crate::{error::RiotError, refs::RiotLineRefs, riot::Riot};
