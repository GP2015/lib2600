mod common;
use rstest::rstest;

#[rstest]
#[case(false, false, false)]
#[case(false, false, true)]
#[case(false, true, false)]
#[case(false, true, true)]
#[case(true, false, false)]
#[case(true, false, true)]
#[case(true, true, false)]
#[case(true, true, true)]
fn use_io_no_rs(#[case] rw: bool, #[case] a1: bool, #[case] a0: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rw(rw);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, a0).unwrap();
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
#[case(false, false)]
#[case(false, true)]
#[case(true, false)]
#[case(true, true)]
fn use_io_no_rw(#[case] a1: bool, #[case] a0: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, a0).unwrap();
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
#[case(false, false, false)]
#[case(false, false, true)]
#[case(false, true, false)]
#[case(false, true, true)]
#[case(true, false, false)]
#[case(true, false, true)]
#[case(true, true, false)]
#[case(true, true, true)]
fn use_io_no_a2(#[case] rw: bool, #[case] a1: bool, #[case] a0: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_rw(rw);
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, a0).unwrap();
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
#[case(false, false)]
#[case(false, true)]
#[case(true, false)]
#[case(true, true)]
fn use_io_no_a1(#[case] rw: bool, #[case] a0: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_rw(rw);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(0, a0).unwrap();
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
#[case(false, false)]
#[case(false, true)]
#[case(true, false)]
#[case(true, true)]
fn use_io_no_a0(#[case] rw: bool, #[case] a1: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_rw(rw);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    assert!(riot.pulse_phi2().is_err());
}
