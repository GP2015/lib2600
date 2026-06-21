#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unreachable, clippy::unwrap_used,))]
#![allow(clippy::missing_errors_doc)]

mod common;
mod cpu;
mod full;
mod riot;

#[cfg(not(test))]
#[panic_handler]
const fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub use crate::full::Emulator;

// pub use crate::{
//     common::{
//         HasMux,
//         condition::{BaseCondition, IsCondition},
//         line::{
//             error::LineError,
//             ident::LineIdent,
//             multi::{BusDriveState, IsBusDriveState},
//             single::DriveState,
//         },
//         read::{
//             multi::{IsMultiRead, MultiRead},
//             single::SingleRead,
//         },
//         signal::LineSignal,
//     },
//     full::{Emulator, ext_drives::ExtDrives},
// };
