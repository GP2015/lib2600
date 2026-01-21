mod common;
use mos6532::Riot;
use mos6532::RiotError;
use rstest::rstest;

const ATYPE: bool = false;
const BTYPE: bool = true;

fn write_ddr_pulse(riot: &mut Riot, reg: bool, data: u8) -> Result<(), RiotError> {
    match reg {
        ATYPE => riot.write_ddra_pulse(data),
        BTYPE => riot.write_ddrb_pulse(data),
    }
}

fn write_or_pulse(riot: &mut Riot, reg: bool, data: u8) -> Result<(), RiotError> {
    match reg {
        ATYPE => riot.write_ora_pulse(data),
        BTYPE => riot.write_orb_pulse(data),
    }
}

fn read_or_pulse(riot: &mut Riot, reg: bool) -> Result<u8, RiotError> {
    match reg {
        ATYPE => riot.read_ora_pulse(),
        BTYPE => riot.read_orb_pulse(),
    }
}

fn write_p(riot: &mut Riot, reg: bool, data: u8) {
    match reg {
        ATYPE => riot.write_pa(data),
        BTYPE => riot.write_pb(data),
    }
}

fn read_p(riot: &mut Riot, reg: bool) -> Result<u8, RiotError> {
    match reg {
        ATYPE => riot.read_pa(),
        BTYPE => riot.read_pb(),
    }
}

#[rstest]
#[case(ATYPE)]
#[case(BTYPE)]
fn write_output_p(#[case] reg: bool) {
    let mut riot = common::riot_post_reset();
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE)]
#[case(BTYPE)]
fn write_input_p(#[case] reg: bool) {
    let mut riot = common::riot_post_reset();
    write_p(&mut riot, reg, 0x67);
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x89);
}

#[rstest]
#[case(ATYPE, 0x0F, 0x69)]
#[case(ATYPE, 0xF0, 0x87)]
#[case(BTYPE, 0x0F, 0x69)]
#[case(BTYPE, 0xF0, 0x87)]
fn write_mixed_p(#[case] reg: bool, #[case] ddr: u8, #[case] out: u8) {
    let mut riot = common::riot_post_reset();
    write_p(&mut riot, reg, 0x67);
    write_ddr_pulse(&mut riot, reg, ddr).unwrap();
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), out);
}

#[rstest]
#[case(ATYPE)]
#[case(BTYPE)]
fn read_input_p(#[case] reg: bool) {
    let mut riot = common::riot_post_reset();
    write_p(&mut riot, reg, 0x67);
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE)]
#[case(BTYPE)]
fn read_output_p(#[case] reg: bool) {
    let mut riot = common::riot_post_reset();
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    write_p(&mut riot, reg, 0x89);
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), 0x67);
}

#[rstest]
#[case(ATYPE, 0x0F, 0x69)]
#[case(ATYPE, 0xF0, 0x87)]
#[case(BTYPE, 0x0F, 0x69)]
#[case(BTYPE, 0xF0, 0x87)]
fn read_mixed_p(#[case] reg: bool, #[case] ddr: u8, #[case] out: u8) {
    let mut riot = common::riot_post_reset();
    write_p(&mut riot, reg, 0x67);
    write_ddr_pulse(&mut riot, reg, ddr).unwrap();
    write_or_pulse(&mut riot, reg, 0x89).unwrap();
    assert_eq!(read_or_pulse(&mut riot, reg).unwrap(), out);
}

#[rstest]
#[case(ATYPE)]
#[case(BTYPE)]
fn output_p_update_deselected(#[case] reg: bool) {
    let mut riot = common::riot_post_reset();
    write_ddr_pulse(&mut riot, reg, 0xFF).unwrap();
    write_or_pulse(&mut riot, reg, 0x67).unwrap();
    write_p(&mut riot, reg, 0x89);
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x89);
    riot.deselected_pulse().unwrap();
    assert_eq!(read_p(&mut riot, reg).unwrap(), 0x67);
}
