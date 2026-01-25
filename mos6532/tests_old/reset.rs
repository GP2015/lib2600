mod common;
use mos6532::Riot;
use rstest::rstest;

#[rstest]
fn reset_ram(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    riot.reset_pulse().unwrap();
    assert!(riot.read_ram_pulse(0x45).is_err());
}

#[rstest]
fn reset_ddr(#[from(common::riot_post_reset)] mut riot: Riot) {
    riot.write_ddra_pulse(0x67).unwrap();
    riot.write_ddrb_pulse(0x89).unwrap();
    riot.reset_pulse().unwrap();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0);
    assert_eq!(riot.read_ddrb_pulse().unwrap(), 0);
}
