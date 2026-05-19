pub mod line;
pub mod read;
pub mod signal;

pub trait IsCondition {
    fn as_cond(&self) -> BaseCondition;
}

pub trait HasMux {
    fn mux(
        cond: &impl IsCondition,
        low_opt: &impl Fn() -> Self,
        high_opt: &impl Fn() -> Self,
    ) -> Self;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BaseCondition {
    No,
    Yes,
    Unknown,
}

impl IsCondition for BaseCondition {
    fn as_cond(&self) -> Self {
        *self
    }
}
