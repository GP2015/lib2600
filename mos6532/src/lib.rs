#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(
    not(test),
    warn(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::similar_names
)]

mod connections;
mod lines;
mod riot;

pub use crate::{connections::RiotConnectionIds, lines::RiotLines, riot::Riot};
pub use emutils::line::{Bus, BusConnectionId, Line, LineConnectionId, LineError};
