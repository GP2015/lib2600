use crate::common::{
    HasCouldBe, HasMux, IsCondition, condition::BaseCondition, read::single::SingleRead,
};
use arrayvec::ArrayVec;
use core::array;

pub type MultiRead<const SIZE: usize> = [SingleRead; SIZE];

pub trait IsMultiRead {
    fn from_value(value: u16) -> Self;
    fn iter_possible_reads(&self) -> impl Iterator<Item = u16>;

    #[must_use]
    fn incremented(&self) -> Self;
    #[must_use]
    fn decremented(&self) -> Self;

    #[must_use]
    fn combine_with(&self, other: &Self) -> Self;
}

impl<const SIZE: usize> IsMultiRead for MultiRead<SIZE> {
    fn from_value(value: u16) -> Self {
        array::from_fn(|bit| SingleRead::from(value >> bit & 1 == 1))
    }

    fn iter_possible_reads(&self) -> impl Iterator<Item = u16> {
        let mut count = ArrayVec::<_, SIZE>::new();
        let mut mask = 0;

        for (i, &read) in self.iter().enumerate() {
            match read.as_bool() {
                Some(b) => mask |= u16::from(b) << i,
                None => count.push(i),
            }
        }

        (0..(1 << count.len())).map(move |id| {
            let mut val = mask;

            for (src_bit, &dst_bit) in count.iter().enumerate() {
                val |= ((id >> src_bit) & 1) << dst_bit;
            }

            val
        })
    }

    fn incremented(&self) -> Self {
        let mut res = *self;
        let mut must_carry = true;

        for bit in &mut res {
            match bit {
                SingleRead::Low => {
                    *bit = if must_carry {
                        SingleRead::High
                    } else {
                        SingleRead::Unknown
                    };
                    break;
                }
                SingleRead::High => {
                    *bit = if must_carry {
                        SingleRead::Low
                    } else {
                        SingleRead::Unknown
                    };
                }
                SingleRead::Unknown => {
                    if must_carry {
                        *bit = SingleRead::High;
                    }
                    must_carry = false;
                }
            }
        }

        res
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
        array::from_fn(|bit| self[bit].combine_with(other[bit]))
    }
}

impl<const SIZE: usize> HasMux for MultiRead<SIZE> {
    fn mux(cond: BaseCondition, low_opt: &impl Fn() -> Self, high_opt: &impl Fn() -> Self) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(&high_opt()),
        }
    }
}

impl<const SIZE: usize> HasCouldBe<usize> for MultiRead<SIZE> {
    fn could_be(&self, other: &usize) -> BaseCondition {
        let check = |check_bit: usize| {
            self.iter()
                .enumerate()
                .all(|(bit, bit_state)| bit_state.could_read((*other >> bit) & 1 == check_bit))
        };

        match (check(1), check(0)) {
            (true, false) => BaseCondition::Yes,
            (false, true) => BaseCondition::No,
            _ => BaseCondition::Unknown,
        }
    }
}
