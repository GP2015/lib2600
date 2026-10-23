use crate::BaseCond;

pub trait Combine {
    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;

    fn update_val<F>(&mut self, cond: BaseCond, new_val: F)
    where
        Self: Sized,
        F: FnOnce() -> Self,
    {
        match cond {
            BaseCond::No => (),
            BaseCond::Yes => *self = new_val(),
            BaseCond::Unknown => *self = self.combine_with(&new_val()),
        }
    }

    fn set_to_val(&mut self, must_change: bool, new_val: Self)
    where
        Self: Sized,
    {
        if must_change {
            *self = new_val;
        } else {
            *self = self.combine_with(&new_val);
        }
    }

    fn mux<C, L, H>(cond: C, low_opt: L, high_opt: H) -> Self
    where
        Self: Sized,
        C: Into<BaseCond>,
        L: FnOnce() -> Self,
        H: FnOnce() -> Self,
    {
        match cond.into() {
            BaseCond::No => low_opt(),
            BaseCond::Yes => high_opt(),
            BaseCond::Unknown => low_opt().combine_with(&high_opt()),
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
