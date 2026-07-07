use crate::common::cond::{IsCondition, base::BaseCondition};

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

macro_rules! mux_matches {
    (($cond:expr, $arm:expr), $catch:expr) => {
        crate::common::combine::Combine::mux($cond, $catch, $arm)
    };

    (($cond:expr, $arm:expr), ($cond2:expr, $arm2:expr), $($rest:tt)*) => {
        crate::common::combine::Combine::mux(
            $cond,
            &|| mux_matches!(($cond2, $arm2), $($rest)*),
            $arm
        )
    };
}
pub(crate) use mux_matches;
