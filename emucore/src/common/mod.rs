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

macro_rules! mux_matches {
    (($cond:expr, $arm:expr), $catch:expr) => {
        Combine::mux($cond, $catch, $arm)
    };

    (($cond:expr, $arm:expr), ($cond2:expr, $arm2:expr), $($rest:tt)*) => {
        Combine::mux($cond, &|| mux_matches!(($cond2, $arm2), $($rest)*), $arm)
    };
}
pub(crate) use mux_matches;

pub trait CheckIs<T> {
    fn is(&self, other: T) -> BaseCondition;

    fn is_any(&self, others: impl Iterator<Item = T>) -> BaseCondition {
        others
            .into_iter()
            .fold(BaseCondition::No, |acc, v| acc | self.is(v))
    }
}

pub trait Combine {
    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;

    fn mux<L, H>(cond: BaseCondition, low_opt: L, high_opt: H) -> Self
    where
        Self: Sized,
        L: FnOnce() -> Self,
        H: FnOnce() -> Self,
    {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(&high_opt()),
        }
    }
}
