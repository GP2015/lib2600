#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]

mod error;
mod line_refs;
mod riot;

pub use crate::{error::RiotError, line_refs::RiotLineRefs, riot::Riot};
