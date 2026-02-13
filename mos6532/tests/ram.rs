mod common;
use mos6532::{BusInterface, Riot, SinglePinInterface};
use rstest::rstest;

#[rstest]
fn read_write_ram_success(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    assert_eq!(riot.read_ram_pulse(0x45).unwrap(), 0x67);
}

#[rstest]
fn read_write_ram_success_manual(#[from(common::riot_post_reset_select)] mut riot: Riot) {
    riot.rs_mut().drive_in(false).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().drive_in(0x45).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();

    riot.rw_mut().drive_in(true).unwrap();
    riot.db_mut().drive_in(0x89).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x67);
}

#[rstest]
fn write_ram_deselected(#[from(common::riot_post_reset_select)] mut riot: Riot) {
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rs_mut().drive_in(false).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().drive_in(0x45).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();
    assert!(riot.read_ram_pulse(0x45).is_err());
}

#[rstest]
fn read_ram_deselected(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();

    riot.db_mut().drive_in(0x89).unwrap();
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rw_mut().drive_in(true).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x89);
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
