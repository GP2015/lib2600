mod common;
use rstest::rstest;

#[test]
fn read_write_ddr_success() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0x67).unwrap();
    riot.write_ddrb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0x67);
    assert_eq!(riot.read_ddrb_pulse().unwrap(), 0x89);
}

#[rstest]
fn read_write_ddr_success_manual(#[values(false, true)] a1: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_rw(false);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, true).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();

    riot.write_rw(true);
    riot.write_db(0x89);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0x67);
}

#[rstest]
fn write_ddr_deselected(#[values(false, true)] a1: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_cs1(false);
    riot.write_rs(true);
    riot.write_rw(false);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, true).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();

    riot.write_cs1(true);
    riot.write_rw(true);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0);
}

#[rstest]
fn read_ddr_deselected(#[values(false, true)] a1: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    riot.write_rw(false);
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, true).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();

    riot.write_db(0x89);
    riot.write_cs1(false);
    riot.write_rw(true);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0x89);
}
