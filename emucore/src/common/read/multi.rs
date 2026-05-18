use crate::common::{BaseCondition, HasMux, IsCondition, read::single::SingleRead};
use core::array;

pub type MultiRead<const SIZE: usize> = [SingleRead; SIZE];

pub trait IsMultiRead {
    fn from_usize(value: usize) -> Self;
    fn iter_possible_reads(&self) -> impl Iterator<Item = usize>;
    fn is_val(&self, val: usize) -> BaseCondition;
    #[must_use]
    fn decremented(&self) -> Self;
    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;
}

impl<const SIZE: usize> IsMultiRead for MultiRead<SIZE> {
    fn from_usize(value: usize) -> Self {
        array::from_fn(|bit| SingleRead::from(value >> bit & 1 == 1))
    }

    fn iter_possible_reads(&self) -> impl Iterator<Item = usize> {
        // self.iter()
        //     .map(|line_state| line_state.possible_reads().iter().copied())
        //     .multi_cartesian_product()
        //     .map(|bits| bit::bits_to_usize(bits.into_iter()))

        (0..(1 << SIZE)).filter(|&val| {
            let mut b = true;
            for (i, state) in self.iter().enumerate() {
                let bit = (val >> i) & 1 == 1;
                if state.could_read(bit) {
                    b = false;
                    break;
                }
            }
            b
        })
    }

    fn is_val(&self, val: usize) -> BaseCondition {
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

    fn decremented(&self) -> Self {
        let mut res = *self;
        let mut must_carry = true;

        for bit in &mut res {
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

    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| {
            #[expect(clippy::indexing_slicing)]
            self[bit].combine_with(other[bit])
        })
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
