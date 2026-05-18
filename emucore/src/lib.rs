#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
// #![cfg_attr(
//     not(test),
//     warn(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
// )]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

mod common;
mod core;
mod riot;

pub use crate::{
    common::{
        line::{
            error::LineError,
            ident::LineIdent,
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        mux::{BaseCondition, HasMux, IsCondition},
        read::{
            multi::{IsMultiRead, MultiRead},
            single::SingleRead,
        },
        signal::LineSignal,
    },
    core::Emulator,
};
