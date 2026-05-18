#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used, clippy::indexing_slicing))]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::similar_names
)]

mod common;
mod core;
mod riot;

pub use crate::{
    common::{
        BaseCondition, HasMux, IsCondition,
        line::{
            error::LineError,
            ident::LineIdent,
            multi::{BusDriveState, IsBusDriveState},
            single::DriveState,
        },
        read::{
            multi::{IsMultiRead, MultiRead},
            single::SingleRead,
        },
        signal::LineSignal,
    },
    core::{Emulator, ext_drives::ExtDrives},
};
