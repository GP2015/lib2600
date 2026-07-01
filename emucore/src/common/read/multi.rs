use crate::common::{
    CheckIs, HasMux, IsCondition, condition::BaseCondition, read::single::SingleRead,
};
use arrayvec::ArrayVec;
use core::array;

pub type MultiRead<const SIZE: usize> = [SingleRead; SIZE];

pub trait IsMultiRead {
    fn from_value(value: u16) -> Self;
    fn from_pattern(pattern: &str) -> Self;

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

    fn from_pattern(pattern: &str) -> Self {
        let bits: ArrayVec<SingleRead, SIZE> = pattern
            .chars()
            .rev()
            .map(|c| match c {
                '0' => SingleRead::Low,
                '1' => SingleRead::High,
                '?' => SingleRead::Unknown,
                _ => unreachable!(),
            })
            .collect();

        bits.into_inner()
            .expect("the pattern should have exactly SIZE characters")
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

impl<const SIZE: usize> CheckIs<usize> for MultiRead<SIZE> {
    fn is(&self, val: usize) -> BaseCondition {
        let mut must_be = true;

        for (i, &read_bit) in self.iter().enumerate() {
            let val_bit = val >> i & 1 == 1;

            match (read_bit, val_bit) {
                (SingleRead::Low, false) | (SingleRead::High, true) => (),
                (SingleRead::Low, true) | (SingleRead::High, false) => return BaseCondition::No,
                (SingleRead::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCondition::Yes
        } else {
            BaseCondition::Unknown
        }
    }
}

impl<const SIZE: usize> CheckIs<&Self> for MultiRead<SIZE> {
    fn is(&self, pattern: &Self) -> BaseCondition {
        let mut must_be = true;

        for (&read_bit, &pattern_bit) in self.iter().zip(pattern.iter()) {
            match (read_bit, pattern_bit) {
                (SingleRead::Low, SingleRead::Low)
                | (SingleRead::High, SingleRead::High)
                | (_, SingleRead::Unknown) => (),

                (SingleRead::Low, SingleRead::High) | (SingleRead::High, SingleRead::Low) => {
                    return BaseCondition::No;
                }

                (SingleRead::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCondition::Yes
        } else {
            BaseCondition::Unknown
        }
    }
}
