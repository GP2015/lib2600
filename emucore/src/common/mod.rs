pub mod condition;
pub mod line;
pub mod read;
pub mod signal;

use crate::common::{
    condition::{BaseCondition, IsCondition},
    read::{multi::MultiRead, single::SingleRead},
};

pub type BitReg = SingleRead;
pub type MBitReg<const SIZE: usize> = MultiRead<SIZE>;

pub trait HasMux {
    fn mux(cond: BaseCondition, low_opt: &impl Fn() -> Self, high_opt: &impl Fn() -> Self) -> Self;
}
