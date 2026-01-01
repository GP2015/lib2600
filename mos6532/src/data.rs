mod bitreg;
mod buffers;
mod mbitreg;
mod ram;
mod registers;

pub enum AOrB {
    A,
    B,
}

pub use buffers::Buffers;
pub use ram::RAM;
pub use registers::Registers;
