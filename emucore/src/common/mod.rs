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

#[macro_export]
macro_rules! mux_matches {
    (($cond:expr, $arm:expr), $catch:expr) => {
        HasMux::mux($cond, $catch, $arm)
    };

    (($cond:expr, $arm:expr), ($cond2:expr, $arm2:expr), $($rest:tt)*) => {
        HasMux::mux($cond, &|| mux_matches!(($cond2, $arm2), $($rest)*), $arm)
    };
}

pub trait CheckIs<T> {
    fn is(&self, other: T) -> BaseCondition;

    fn is_any(&self, others: impl IntoIterator<Item = T>) -> BaseCondition {
        others
            .into_iter()
            .fold(BaseCondition::No, |acc, v| acc | self.is(v))
    }
}
