pub fn get_bit_of_usize(val: usize, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

pub fn get_bit_of_u8(val: u8, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

pub fn usize_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
    val >> bit_count != 0
}

pub fn get_low_bits_of_usize(val: usize, bit_count: usize) -> usize {
    val & ((1 << bit_count) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0b101, 0, true)]
    #[case(0b101, 1, false)]
    #[case(0b101, 7, false)]
    fn get_bit_of_usize_t(#[case] val: usize, #[case] bit: usize, #[case] res: bool) {
        assert_eq!(get_bit_of_usize(val, bit), res);
    }

    #[rstest]
    #[case(0b101, 0, true)]
    #[case(0b101, 1, false)]
    #[case(0b101, 7, false)]
    fn get_bit_of_u8_t(#[case] val: u8, #[case] bit: usize, #[case] res: bool) {
        assert_eq!(get_bit_of_u8(val, bit), res);
    }

    #[rstest]
    #[case(0b1011, 3, true)]
    #[case(0b1011, 4, false)]
    #[case(0b1011, 5, false)]
    fn usize_exceeds_bit_count_t(#[case] val: usize, #[case] bit_count: usize, #[case] res: bool) {
        assert_eq!(usize_exceeds_bit_count(val, bit_count), res);
    }

    #[rstest]
    #[case(0b1011, 0, 0)]
    #[case(0b1011, 1, 1)]
    #[case(0b1011, 2, 0b11)]
    #[case(0b1011, 3, 0b11)]
    #[case(0b1011, 7, 0b1011)]
    fn get_low_bits_of_usize_t(#[case] val: usize, #[case] bit_count: usize, #[case] res: usize) {
        assert_eq!(get_low_bits_of_usize(val, bit_count), res);
    }
}
