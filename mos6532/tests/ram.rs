use rstest::rstest;

mod common;

#[test]
fn read_write_ram_success() {
    let mut riot = common::riot_post_reset();
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert_eq!(riot.read_ram_pulse(0x45).unwrap(), 0x67);
}

#[test]
fn read_write_ram_success_manual() {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(false);
    riot.write_rw(false);
    riot.write_a(0x45).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();
    riot.write_rw(true);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0x67);
}

#[test]
fn read_ram_out_of_bounds() {
    let mut riot = common::riot_post_reset();
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert!(riot.read_ram_pulse(0x85).is_err());
}

#[test]
fn write_ram_out_of_bounds() {
    let mut riot = common::riot_post_reset();
    assert!(riot.write_ram_pulse(0x85, 0x67).is_err());
}

#[test]
fn read_uninitialised_ram() {
    let mut riot = common::riot_post_reset();
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert!(riot.read_ram_pulse(0x46).is_err());
}

#[test]
fn read_reset_ram() {
    let mut riot = common::riot_post_reset();
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    riot.reset_pulse().unwrap();
    assert!(riot.read_ram_pulse(0x45).is_err());
}

#[rstest]
#[case(false)]
#[case(true)]
fn use_ram_no_rs(#[case] rw: bool) {
    let mut riot = common::riot_post_reset_select();
    riot.write_rw(rw);
    assert!(riot.pulse_phi2().is_err());
}

#[test]
fn use_ram_no_rw() {
    let mut riot = common::riot_post_reset_select();
    riot.write_rs(true);
    assert!(riot.pulse_phi2().is_err());
}
