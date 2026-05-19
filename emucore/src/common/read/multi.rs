use crate::common::{
    BaseCondition, HasMux, IsCondition,
    line::{error::LineError, ident::LineIdent},
    read::single::SingleRead,
};
use core::array;
use heapless::Vec;

pub type MultiRead<const SIZE: usize> = [SingleRead; SIZE];

pub trait IsMultiRead {
    fn from_value(value: u16) -> Self;
    fn iter_possible_reads(&self) -> impl Iterator<Item = u16>;
    fn is_value(&self, val: u16) -> BaseCondition;
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
        let mut count = Vec::<_, SIZE>::new();
        let mut mask = 0;

        for (i, &read) in self.iter().enumerate() {
            match read.as_bool() {
                Some(b) => mask |= u16::from(b) << i,
                None => count.push(i).expect("big enough"),
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

    fn is_value(&self, value: u16) -> BaseCondition {
        let check = |check_bit: u16| {
            self.iter()
                .enumerate()
                .all(|(bit, bit_state)| bit_state.could_read((value >> bit) & 1 == check_bit))
        };

        match (check(1), check(0)) {
            (true, false) => BaseCondition::Yes,
            (false, true) => BaseCondition::No,
            _ => BaseCondition::Unknown,
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

pub trait CheckMultiRead<const SIZE: usize> {
    fn ok_or_impossible(self, name: &'static str) -> Result<MultiRead<SIZE>, LineError>;
}

impl<const SIZE: usize> CheckMultiRead<SIZE> for Result<MultiRead<SIZE>, usize> {
    fn ok_or_impossible(self, name: &'static str) -> Result<MultiRead<SIZE>, LineError> {
        match self {
            Ok(read) => Ok(read),
            Err(bit) => Err(LineError::ImpossibleLineSignal {
                ident: LineIdent::BusLine {
                    bus_name: name,
                    bit,
                },
            }),
        }
    }
}
