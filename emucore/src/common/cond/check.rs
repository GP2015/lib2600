use crate::common::cond::base::BaseCondition;

pub trait CheckIs<T> {
    fn is(&self, other: T) -> BaseCondition;

    fn is_any(&self, others: impl Iterator<Item = T>) -> BaseCondition {
        others
            .into_iter()
            .fold(BaseCondition::No, |acc, v| acc | self.is(v))
    }
}
