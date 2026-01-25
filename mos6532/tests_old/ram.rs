mod common;
use mos6532::Riot;
use rstest::rstest;

#[rstest]
fn read_write_ram_success(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert_eq!(riot.read_ram_pulse(0x45).unwrap(), 0x67);
}

#[rstest]
fn read_write_ram_success_manual(#[from(common::riot_post_reset_select)] mut riot: Riot) {
    riot.write_rs(false);
    riot.write_rw(false);
    riot.write_a(0x45).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();

    riot.write_rw(true);
    riot.write_db(0x89);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0x67);
}

#[rstest]
fn write_ram_deselected(#[from(common::riot_post_reset_select)] mut riot: Riot) {
    riot.write_cs1(false);
    riot.write_rs(false);
    riot.write_rw(false);
    riot.write_a(0x45).unwrap();
    riot.write_db(0x67);
    riot.pulse_phi2().unwrap();
    assert!(riot.read_ram_pulse(0x45).is_err());
}

#[rstest]
fn read_ram_deselected(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();

    riot.write_db(0x89);
    riot.write_cs1(false);
    riot.write_rw(true);
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.read_db().unwrap(), 0x89);
}

#[rstest]
fn read_ram_out_of_bounds(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert!(riot.read_ram_pulse(0x85).is_err());
}

#[rstest]
fn write_ram_out_of_bounds(#[from(common::riot_post_reset)] mut riot: Riot) {
    assert!(riot.write_ram_pulse(0x85, 0x67).is_err());
}

#[rstest]
fn read_ram_uninitialised(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert!(riot.read_ram_pulse(0x46).is_err());
}
