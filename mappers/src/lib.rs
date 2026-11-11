#![no_std]
#![warn(clippy::pedantic, clippy::nursery)]
#![cfg_attr(not(test), warn(clippy::unwrap_used))]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

mod m2k;
mod m4k;

pub use m2k::M2K;
pub use m4k::M4K;

use emucore::{BitStringState, BusOutput};

pub struct MapperDrive {
    pub a: BusOutput<13>,
    pub db: BusOutput<8>,
}

pub trait Mapper {
    fn tick(&mut self, a: BitStringState<13>, db: BitStringState<8>) -> MapperDrive;
}
