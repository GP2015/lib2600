use crate::common::{bit::single::BitState, combine::Combine, cond::BaseCond};
use arrayvec::ArrayVec;
use core::array;
use derive_more::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Deref, DerefMut, Eq, Hash, PartialEq)]
pub struct BitStringState<const SIZE: usize>([BitState; SIZE]);

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
    pub fn is_uint(&self, val: usize) -> BaseCond {
        let mut must_be = true;

        for (i, &read_bit) in self.iter().enumerate() {
            let val_bit = val >> i & 1 == 1;

            match (read_bit, val_bit) {
                (BitState::Low, false) | (BitState::High, true) => (),
                (BitState::Low, true) | (BitState::High, false) => return BaseCond::No,
                (BitState::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCond::Yes
        } else {
            BaseCond::Unknown
        }
    }

    #[must_use]
    pub fn is_pattern(&self, pattern: &Self) -> BaseCond {
        let mut must_be = true;

        for (&read_bit, &pattern_bit) in self.iter().zip(pattern.iter()) {
            match (read_bit, pattern_bit) {
                (BitState::Low, BitState::Low)
                | (BitState::High, BitState::High)
                | (_, BitState::Unknown) => (),

                (BitState::Low, BitState::High) | (BitState::High, BitState::Low) => {
                    return BaseCond::No;
                }

                (BitState::Unknown, _) => must_be = false,
            }
        }

        if must_be {
            BaseCond::Yes
        } else {
            BaseCond::Unknown
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
    pub fn section<const LEN: usize>(self, start: usize) -> BitStringState<LEN> {
        let arr = self[start..start + LEN]
            .try_into()
            .expect("arrays will always be the same size");
        BitStringState(arr)
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

impl<const SIZE: usize> Default for BitStringState<SIZE> {
    fn default() -> Self {
        Self([BitState::default(); _])
    }
}

impl<T, const SIZE: usize> From<[T; SIZE]> for BitStringState<SIZE>
where
    T: Into<BitState>,
{
    fn from(value: [T; SIZE]) -> Self {
        Self(value.map(Into::into))
    }
}

// num-traits cannot be used here as it causes
// conflicting implementations with From<[T; SIZE]>
macro_rules! impl_from_unsigned {
    ($u_type:ident) => {
        impl<const SIZE: usize> From<$u_type> for BitStringState<SIZE> {
            fn from(value: $u_type) -> Self {
                Self(array::from_fn(|bit| BitState::from(value >> bit & 1 == 1)))
            }
        }
    };
}
impl_from_unsigned!(u8);
impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(usize);

impl<const SIZE: usize> Combine for BitStringState<SIZE> {
    fn combine_with(&self, other: &Self) -> Self {
        Self(array::from_fn(|bit| self[bit].combine_with(&other[bit])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn from_bit_state_arr() {
        let arr = [BitState::Low, BitState::High, BitState::Unknown];
        assert_eq!(BitStringState::from(arr), BitStringState(arr));
    }

    #[test]
    fn default() {
        assert_eq!(BitStringState::default(), [BitState::Unknown; 8].into());
    }

    #[rstest]
    #[case(0, [false, false, false])]
    #[case(1, [true, false, false])]
    #[case(2, [false, true, false])]
    #[case(3, [true, true, false])]
    #[case(4, [false, false, true])]
    #[case(5, [true, false, true])]
    #[case(6, [false, true, true])]
    #[case(7, [true, true, true])]
    fn from_unsigned_normal(#[case] num: usize, #[case] bits: [bool; 3]) {
        assert_eq!(BitStringState::from(num), BitStringState::from(bits));
    }

    #[rstest]
    fn from_unsigned_wrapping(#[values(0, 1, 2, 3, 5, 8, 13, 21)] num: usize) {
        const BITS: usize = 3;
        assert_eq!(
            BitStringState::<BITS>::from(num),
            BitStringState::<BITS>::from(num + (1 << BITS))
        );
    }
}
