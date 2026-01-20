mod common;

#[test]
fn reset_ram() {
    let mut riot = common::riot_post_reset();
    riot.write_ram_pulse(0x45, 0x67).unwrap();
    riot.reset_pulse().unwrap();
    assert!(riot.read_ram_pulse(0x45).is_err());
}

#[test]
fn reset_ddr() {
    let mut riot = common::riot_post_reset();
    assert_eq!(riot.read_ddra_pulse().unwrap(), 0);
    assert_eq!(riot.read_ddrb_pulse().unwrap(), 0);
}
