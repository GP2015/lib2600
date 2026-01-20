mod common;
// use rstest::rstest;

#[test]
fn output_to_p() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x67);
    assert_eq!(riot.read_pb().unwrap(), 0x89);
}

#[test]
fn output_to_p_input_ddr() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x45);
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x23);
    assert_eq!(riot.read_pb().unwrap(), 0x45);
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x67);
    assert_eq!(riot.read_pb().unwrap(), 0x89);
}

#[test]
fn input_to_p() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x67);
    riot.write_ora_pulse(0x45).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ora_pulse().unwrap(), 0x23);
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x67);
}

#[test]
fn input_to_pa_output_ddra() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_orb_pulse(0x67).unwrap();
    riot.write_pa(0x89);
    assert_eq!(riot.read_ora_pulse().unwrap(), 0x89);
}

#[test]
fn input_to_pb_output_ddrb() {
    let mut riot = common::riot_post_reset();
    riot.write_ddrb_pulse(0xFF).unwrap();
    riot.write_orb_pulse(0x67).unwrap();
    riot.write_pb(0x89);
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x67);
    riot.write_ddrb_pulse(0).unwrap();
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x89);
}

// #[rstest]
// #[case(false)]
// #[case(true)]
// fn read_write_or_success_manual(#[case] a1: bool) {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_rs(true);
//     riot.write_rw(false);
//     riot.write_a_bit(2, false).unwrap();
//     riot.write_a_bit(1, a1).unwrap();
//     riot.write_a_bit(0, false).unwrap();
//     riot.write_db(0x67);
//     riot.pulse_phi2().unwrap();

//     riot.write_rw(true);
//     riot.write_db(0x89);
//     riot.pulse_phi2().unwrap();
//     assert_eq!(riot.read_db().unwrap(), 0x67);
// }

// #[rstest]
// #[case(false)]
// #[case(true)]
// fn write_or_deselected(#[case] a1: bool) {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_cs1(false);
//     riot.write_rs(true);
//     riot.write_rw(false);
//     riot.write_a_bit(2, false).unwrap();
//     riot.write_a_bit(1, a1).unwrap();
//     riot.write_a_bit(0, false).unwrap();
//     riot.write_db(0x67);
//     riot.pulse_phi2().unwrap();

//     riot.write_cs1(true);
//     riot.write_rw(true);
//     riot.pulse_phi2().unwrap();
//     assert_eq!(riot.read_db().unwrap(), 0);
// }

// #[rstest]
// #[case(false)]
// #[case(true)]
// fn read_or_deselected(#[case] a1: bool) {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_rs(true);
//     riot.write_rw(false);
//     riot.write_a_bit(2, false).unwrap();
//     riot.write_a_bit(1, a1).unwrap();
//     riot.write_a_bit(0, false).unwrap();
//     riot.write_db(0x67);
//     riot.pulse_phi2().unwrap();

//     riot.write_db(0x89);
//     riot.write_cs1(false);
//     riot.write_rw(true);
//     riot.pulse_phi2().unwrap();
//     assert_eq!(riot.read_db().unwrap(), 0x89);
// }

// #[test]
// fn read_reset_or() {
//     let mut riot = common::riot_post_reset();
//     assert_eq!(riot.read_ora_pulse().unwrap(), 0);
//     assert_eq!(riot.read_orb_pulse().unwrap(), 0);
// }
