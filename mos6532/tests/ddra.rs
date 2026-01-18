mod common;

#[test]
fn read_write_ddra_success() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0x67).unwrap();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0x67);
}

#[test]
fn read_uninitialised_ddra() {
    let mut riot = common::riot_post_reset();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0);
}

#[test]
fn write_ddra_no_rs() {
    let mut riot = common::riot_post_reset_select();
    riot.write_a_bit(2, false).unwrap();
    riot.write_a_bit(0, true).unwrap();
    riot.write_a_bit(1, false).unwrap();
    riot.write_rw(false);
    assert!(riot.pulse_phi2().is_err());
}

// #[test]
// fn read_ram_no_rs() {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_rw(false);
//     assert!(riot.pulse_phi2().is_err());
// }

// #[test]
// fn write_ram_no_rs() {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_rw(true);
//     assert!(riot.pulse_phi2().is_err());
// }

// #[test]
// fn use_ram_no_rw() {
//     let mut riot = common::riot_post_reset_select();
//     riot.write_rs(true);
//     assert!(riot.pulse_phi2().is_err());
// }
