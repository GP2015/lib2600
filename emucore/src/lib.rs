#![warn(clippy::pedantic, clippy::nursery)]
// #![cfg_attr(
//     not(test),
//     warn(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
// )]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

mod common;
mod core;
mod riot;

// pub use crate::{
//     common::{
//         line::{
//             drive_state::DriveState,
//             error::LineError,
//             multi::{Bus, BusConId},
//             single::{Line, LineConId},
//         },
//         mux::{HasMux, IsCondition},
//         read::{multi::MultiRead, single::SingleRead},
//         signal::LineSignal,
//     },
//     core::Emulator,
// };

pub use crate::core::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
