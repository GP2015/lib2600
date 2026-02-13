mod common;
use mos6532::{BusInterface, Riot, RiotError, SinglePinInterface};
use rstest::rstest;

const ATYPE: bool = false;
const BTYPE: bool = true;

fn write_ddr_pulse(riot: &mut Riot, reg: bool, data: usize) -> Result<(), RiotError> {
    match reg {
        ATYPE => riot.write_ddra_pulse(data),
        BTYPE => riot.write_ddrb_pulse(data),
    }
}

fn write_or_pulse(riot: &mut Riot, reg: bool, data: usize) -> Result<(), RiotError> {
    match reg {
        ATYPE => riot.write_ora_pulse(data),
        BTYPE => riot.write_orb_pulse(data),
    }
}

fn read_or_pulse(riot: &mut Riot, reg: bool) -> Result<usize, RiotError> {
    match reg {
        ATYPE => riot.read_ora_pulse(),
        BTYPE => riot.read_orb_pulse(),
    }
}

fn write_p(riot: &mut Riot, reg: bool, data: usize) {
    match reg {
        ATYPE => riot.pa_mut().drive_in(data).unwrap(),
        BTYPE => riot.pb_mut().drive_in(data).unwrap(),
    }
}

fn read_p(riot: &mut Riot, reg: bool) -> Result<usize, RiotError> {
    match reg {
        ATYPE => riot.pa().read(),
        BTYPE => riot.pb().read(),
    }
}

#[rstest]
fn write_output_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
) {
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
fn write_input_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
) {
    write_p(&mut riot, reg, 0x67);
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x89);
}

#[rstest]
#[case(0x0F, 0x69)]
#[case(0xF0, 0x87)]
fn write_mixed_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
    #[case] ddr: usize,
    #[case] out: usize,
) {
    write_p(&mut riot, reg, 0x67);
    write_ddr_pulse(&mut riot, reg, ddr).unwrap();
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), out);
}

#[rstest]
fn read_input_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
) {
    write_p(&mut riot, reg, 0x67);
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
fn read_output_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
) {
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    write_p(&mut riot, reg, 0x89);
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(0x0F, 0x69)]
#[case(0xF0, 0x87)]
fn read_mixed_p(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
    #[case] ddr: usize,
    #[case] out: usize,
) {
    write_p(&mut riot, reg, 0x67);
    write_ddr_pulse(&mut riot, reg, ddr).unwrap();
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), out);
}

#[rstest]
fn output_p_update_on_deselected_pulse(
    #[from(common::riot_post_reset)] mut riot: Riot,
    #[values(ATYPE, BTYPE)] reg: bool,
) {
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    write_p(&mut riot, reg, 0x89);
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x89);
    riot.cs1_mut().drive_in(false).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE, false)]
#[case(BTYPE, true)]
fn write_output_p_manual(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[case] reg: bool,
    #[case] a1: bool,
) {
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(false).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE, false)]
#[case(BTYPE, true)]
fn read_input_p_manual(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[case] reg: bool,
    #[case] a1: bool,
) {
    write_p(&mut riot, reg, 0x67);
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(true).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(false).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(riot.db().read().unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE, false)]
#[case(BTYPE, true)]
fn write_output_p_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[case] reg: bool,
    #[case] a1: bool,
) {
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(false).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(false).unwrap();
    riot.db_mut().drive_in(0x67).unwrap();
    riot.pulse_phi2().unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0);
}

#[rstest]
#[case(ATYPE, false)]
#[case(BTYPE, true)]
fn read_input_p_deselected(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[case] reg: bool,
    #[case] a1: bool,
) {
    write_p(&mut riot, reg, 0x67);
    riot.cs1_mut().drive_in(false).unwrap();
    riot.rs_mut().drive_in(true).unwrap();
    riot.rw_mut().drive_in(true).unwrap();
    riot.a_mut().pin_mut(2).unwrap().drive_in(false).unwrap();
    riot.a_mut().pin_mut(1).unwrap().drive_in(a1).unwrap();
    riot.a_mut().pin_mut(0).unwrap().drive_in(false).unwrap();
    riot.pulse_phi2().unwrap();
    assert!(riot.db().read().is_err());
}
