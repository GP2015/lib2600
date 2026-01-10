mod buffers;
mod ram;
mod registers;
mod regtype;

pub enum AOrB {
    A,
    B,
}

pub use buffers::Buffers;
pub use ram::Ram;
pub use registers::Registers;
