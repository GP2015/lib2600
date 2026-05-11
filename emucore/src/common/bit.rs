#[must_use]
pub const fn bit_of_usize(val: usize, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

#[must_use]
pub const fn usize_exceeds_bit_count(val: usize, bit_count: usize) -> bool {
    val >> bit_count != 0
}

#[must_use]
pub const fn low_bits_of_usize(val: usize, bit_count: usize) -> usize {
    val & ((1 << bit_count) - 1)
}

#[must_use]
pub fn some_bits_to_usize<I>(bits: I) -> Option<usize>
where
    I: Iterator<Item = Option<bool>> + DoubleEndedIterator,
{
    bits.rev()
        .try_fold(0, |acc, bit| bit.map(|b| (acc << 1) | usize::from(b)))
}

#[must_use]
pub fn bits_to_usize<I>(bits: I) -> usize
where
    I: Iterator<Item = bool> + DoubleEndedIterator,
{
    bits.rev().fold(0, |acc, b| (acc << 1) | usize::from(b))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(0b101, 0, true)]
    #[case(0b101, 1, false)]
    #[case(0b101, 7, false)]
    fn bit_of_usize(#[case] val: usize, #[case] bit: usize, #[case] res: bool) {
        assert_eq!(super::bit_of_usize(val, bit), res);
    }

    #[rstest]
    #[case(0b1011, 3, true)]
    #[case(0b1011, 4, false)]
    #[case(0b1011, 5, false)]
    fn usize_exceeds_bit_count(#[case] val: usize, #[case] bit_count: usize, #[case] res: bool) {
        assert_eq!(super::usize_exceeds_bit_count(val, bit_count), res);
    }

    #[rstest]
    #[case(0b1011, 0, 0)]
    #[case(0b1011, 1, 1)]
    #[case(0b1011, 2, 0b11)]
    #[case(0b1011, 3, 0b11)]
    #[case(0b1011, 7, 0b1011)]
    fn low_bits_of_usize(#[case] val: usize, #[case] bit_count: usize, #[case] res: usize) {
        assert_eq!(super::low_bits_of_usize(val, bit_count), res);
    }

    #[rstest]
    #[case(&[], 0)]
    #[case(&[false, false, false, false], 0)]
    #[case(&[true], 1)]
    #[case(&[false, true], 0b10)]
    #[case(&[false, true, false, true], 0b1010)]
    #[case(&[true, true, false, true], 0b1011)]
    #[case(&[true, true, false, true, false, false], 0b1011)]
    fn bits_to_usize(#[case] bits: &[bool], #[case] res: usize) {
        assert_eq!(super::bits_to_usize(bits.iter().copied()), res);
    }

    #[rstest]
    #[case(&[], 0)]
    #[case(&[false, false, false, false], 0)]
    #[case(&[true], 1)]
    #[case(&[false, true], 0b10)]
    #[case(&[false, true, false, true], 0b1010)]
    #[case(&[true, true, false, true], 0b1011)]
    #[case(&[true, true, false, true, false, false], 0b1011)]
    fn some_bits_to_usize_success(#[case] bits: &[bool], #[case] res: usize) {
        let val = super::some_bits_to_usize(bits.iter().map(|b| Some(*b)));
        assert_eq!(val, Some(res));
    }

    #[rstest]
    #[case(&[None])]
    #[case(&[Some(true), None])]
    #[case(&[None, Some(false)])]
    #[case(&[Some(true), None, Some(false)])]
    #[case(&[Some(true), Some(false), Some(true), None])]
    #[case(&[None, Some(true), Some(false), Some(true)])]
    fn some_bits_to_usize_failure(#[case] bits: &[Option<bool>]) {
        assert_eq!(super::some_bits_to_usize(bits.iter().copied()), None);
    }
}
