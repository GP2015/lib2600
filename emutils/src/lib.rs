#![warn(clippy::pedantic)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
#![allow(clippy::missing_errors_doc)]

mod bit;
pub mod pin;
pub mod reg;
