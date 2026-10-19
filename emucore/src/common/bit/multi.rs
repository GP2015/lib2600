use crate::common::{bit::single::BitState, combine::Combine, cond::BaseCondition};
use arrayvec::ArrayVec;
use core::array;
use derive_more::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, Hash, Index, IndexMut, PartialEq)]
pub struct BitStringState<const SIZE: usize>([BitState; SIZE]);

impl<const SIZE: usize> Default for BitStringState<SIZE> {
    fn default() -> Self {
        [BitState::default(); _].into()
    }
}

impl<const SIZE: usize> BitStringState<SIZE> {
    #[must_use]
    pub fn value(&self) -> Option<u16> {
        let mut val = 0;

        for bit in self.iter() {
            val += match bit {
                BitState::Low => 0,
                BitState::High => 1,
                BitState::Unknown => return None,
            };

            val <<= 1;
        }

        Some(val)
    }

    #[must_use]
    pub fn is_defined(&self) -> bool {
        !self.iter().any(|bit| matches!(bit, BitState::Unknown))
    }

    #[must_use]
    pub fn is_uint(&self, val: usize) -> BaseCondition {
        let mut must_be = true;

        for (i, &read_bit) in self.iter().enumerate() {
            let val_bit = val >> i & 1 == 1;

            match (read_bit, val_bit) {
                (BitState::Low, false) | (BitState::High, true) => (),
                (BitState::Low, true) | (BitState::High, false) => return BaseCondition::No,
                (BitState::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCondition::Yes
        } else {
            BaseCondition::Unknown
        }
    }

    #[must_use]
    pub fn is_pattern(&self, pattern: &Self) -> BaseCondition {
        let mut must_be = true;

        for (&read_bit, &pattern_bit) in self.iter().zip(pattern.iter()) {
            match (read_bit, pattern_bit) {
                (BitState::Low, BitState::Low)
                | (BitState::High, BitState::High)
                | (_, BitState::Unknown) => (),

                (BitState::Low, BitState::High) | (BitState::High, BitState::Low) => {
                    return BaseCondition::No;
                }

                (BitState::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCondition::Yes
        } else {
            BaseCondition::Unknown
        }
    }

    pub fn iter_possible_uints(self) -> impl Iterator<Item = u16> {
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

    #[must_use]
    pub fn incremented(self) -> Self {
        let mut res = self;
        let mut must_carry = true;

        for bit in &mut *res {
            match bit {
                BitState::Low => {
                    *bit = if must_carry {
                        BitState::High
                    } else {
                        BitState::Unknown
                    };
                    break;
                }
                BitState::High => {
                    *bit = if must_carry {
                        BitState::Low
                    } else {
                        BitState::Unknown
                    };
                }
                BitState::Unknown => {
                    if must_carry {
                        *bit = BitState::High;
                    }
                    must_carry = false;
                }
            }
        }

        res
    }

    #[must_use]
    pub fn decremented(self) -> Self {
        let mut res = self;
        let mut must_carry = true;

        for bit in &mut *res {
            match bit {
                BitState::High => {
                    *bit = if must_carry {
                        BitState::Low
                    } else {
                        BitState::Unknown
                    };
                    break;
                }
                BitState::Low => {
                    *bit = if must_carry {
                        BitState::High
                    } else {
                        BitState::Unknown
                    };
                }
                BitState::Unknown => {
                    if must_carry {
                        *bit = BitState::Low;
                    }
                    must_carry = false;
                }
            }
        }

        res
    }
}

impl<const SIZE: usize> From<u16> for BitStringState<SIZE> {
    fn from(value: u16) -> Self {
        array::from_fn(|bit| BitState::from(value >> bit & 1 == 1)).into()
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for BitStringState<SIZE>
where
    T: Into<BitState>,
{
    fn from(value: [T; SIZE]) -> Self {
        value.map(Into::into).into()
    }
}

impl<const SIZE: usize> Combine for BitStringState<SIZE> {
    fn combine_with(&self, other: &Self) -> Self {
        array::from_fn(|bit| self[bit].combine_with(&other[bit])).into()
    }
}
