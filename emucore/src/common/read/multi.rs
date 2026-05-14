use std::array;

use crate::common::{
    bit,
    mux::{BaseCondition, HasMux, IsCondition},
    read::single::SingleRead,
};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MultiRead<const SIZE: usize> {
    inner: [SingleRead; SIZE],
}

impl<const SIZE: usize> MultiRead<SIZE> {
    pub const fn bit<const BIT: usize>(&self) -> SingleRead {
        const { assert!(BIT < SIZE) }
        self.inner[BIT]
    }

    pub fn try_bit(&self, bit: usize) -> Option<SingleRead> {
        self.inner.get(bit).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = SingleRead> {
        self.inner.iter().copied()
    }

    pub fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        self.inner
            .iter()
            .map(|line_state| line_state.possible_reads().iter().copied())
            .multi_cartesian_product()
            .map(|bits| bit::bits_to_usize(bits.into_iter()))
    }

    pub fn is_val(&self, val: usize) -> BaseCondition {
        let check = |b: usize| {
            self.iter()
                .enumerate()
                .all(|(bit, bit_state)| bit_state.could_read((val >> bit) & 1 == b))
        };

        match (check(1), check(0)) {
            (false, false) => unreachable!(),
            (true, false) => BaseCondition::Yes,
            (false, true) => BaseCondition::No,
            (true, true) => BaseCondition::Unknown,
        }
    }

    pub fn decremented(&self) -> Self {
        let mut res = self.clone();
        let mut must_carry = true;

        for bit in &mut res.inner {
            match bit {
                SingleRead::High => {
                    *bit = if must_carry {
                        SingleRead::Low
                    } else {
                        SingleRead::Unknown
                    };
                    break;
                }
                SingleRead::Low => {
                    *bit = if must_carry {
                        SingleRead::High
                    } else {
                        SingleRead::Unknown
                    };
                }
                SingleRead::Unknown => {
                    if must_carry {
                        *bit = SingleRead::Low;
                    }
                    must_carry = false;
                }
            }
        }

        res
    }

    pub fn combine_with(&self, other: &Self) -> Self {
        Self {
            inner: array::from_fn(|bit| self.inner[bit].combine_with(other.inner[bit])),
        }
    }
}

impl<const SIZE: usize> From<[SingleRead; SIZE]> for MultiRead<SIZE> {
    fn from(value: [SingleRead; SIZE]) -> Self {
        Self { inner: value }
    }
}

impl<const SIZE: usize> From<usize> for MultiRead<SIZE> {
    fn from(value: usize) -> Self {
        Self {
            inner: array::from_fn(|bit| SingleRead::from(value >> bit & 1 == 1)),
        }
    }
}

impl<const SIZE: usize> HasMux for MultiRead<SIZE> {
    fn mux(
        cond: &impl IsCondition,
        low_opt: &impl Fn() -> Self,
        high_opt: &impl Fn() -> Self,
    ) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(&high_opt()),
        }
    }
}
