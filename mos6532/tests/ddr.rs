mod common;
use mos6532::{Bus, Riot, SinglePin};
use rstest::rstest;

#[rstest]
fn read_write_ddr_success(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ddra_pulse(0x67).unwrap();
    riot.write_ddrb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0x67);
    assert_eq!(riot.read_ddrb_pulse().unwrap(), 0x89);
}

#[rstest]
fn read_write_ddr_success_manual(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
) {
    riot.rs().drive_in(true).unwrap();
    riot.rw().drive_in(false).unwrap();
    riot.a().drive_in_bit(2, false).unwrap();
    riot.a().drive_in_bit(1, a1).unwrap();
    riot.a().drive_in_bit(0, true).unwrap();
    riot.db().drive_value_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.rw().drive_in(true).unwrap();
    riot.db().drive_value_in(0x89).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x67);
}

#[rstest]
fn write_ddr_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
) {
    riot.cs1().drive_in(false).unwrap();
    riot.rs().drive_in(true).unwrap();
    riot.rw().drive_in(false).unwrap();
    riot.a().drive_in_bit(2, false).unwrap();
    riot.a().drive_in_bit(1, a1).unwrap();
    riot.a().drive_in_bit(0, true).unwrap();
    riot.db().drive_value_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.cs1().drive_in(true).unwrap();
    riot.rw().drive_in(true).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0);
}

#[rstest]
fn read_ddr_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
) {
    riot.rs().drive_in(true).unwrap();
    riot.rw().drive_in(false).unwrap();
    riot.a().drive_in_bit(2, false).unwrap();
    riot.a().drive_in_bit(1, a1).unwrap();
    riot.a().drive_in_bit(0, true).unwrap();
    riot.db().drive_value_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.db().drive_value_in(0x89).unwrap();
    riot.cs1().drive_in(false).unwrap();
    riot.rw().drive_in(true).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x89);
}
