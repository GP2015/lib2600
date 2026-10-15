pub mod base;

use crate::common::cond::base::BaseCondition;

pub trait IsCondition {
    fn as_cond(&self) -> BaseCondition;
}

pub trait CheckIs<T> {
    fn is(&self, other: T) -> BaseCondition;
}
