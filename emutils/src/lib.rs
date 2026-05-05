#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(
    not(test),
    warn(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]
#![allow(clippy::missing_errors_doc)]

pub mod bit;
pub mod line;
pub mod reg;
