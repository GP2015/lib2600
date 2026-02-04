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
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(true).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.rw_mut().drive_in(true).unwrap();
    riot.db_mut().drive_in(0x89).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x67);
}

#[rstest]
fn write_ddr_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
) {
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(true).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.cs1_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(true).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0);
}

#[rstest]
fn read_ddr_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
) {
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(true).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.db_mut().drive_in(0x89).unwrap();
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rw_mut().drive_in(true).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x89);
}
