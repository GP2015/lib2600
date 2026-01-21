mod common;

#[test]
fn write_output_p() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x67);
    assert_eq!(riot.read_pb().unwrap(), 0x89);
}

#[test]
fn write_input_p() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x45);
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x23);
    assert_eq!(riot.read_pb().unwrap(), 0x45);
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x67);
    assert_eq!(riot.read_pb().unwrap(), 0x89);
}

#[test]
fn write_mixed_p() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x45);
    riot.write_ddra_pulse(0x0F).unwrap();
    riot.write_ddrb_pulse(0xF0).unwrap();
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x27);
    assert_eq!(riot.read_pb().unwrap(), 0x85);
}

#[test]
fn read_input_p() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x45);
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ora_pulse().unwrap(), 0x23);
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x45);
}

#[test]
fn read_output_p() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ora_pulse().unwrap(), 0x67);
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x89);
}

#[test]
fn read_mixed_p() {
    let mut riot = common::riot_post_reset();
    riot.write_pa(0x23);
    riot.write_pb(0x45);
    riot.write_ddra_pulse(0x0F).unwrap();
    riot.write_ddrb_pulse(0xF0).unwrap();
    riot.write_ora_pulse(0x67).unwrap();
    riot.write_orb_pulse(0x89).unwrap();
    assert_eq!(riot.read_ora_pulse().unwrap(), 0x27);
    assert_eq!(riot.read_orb_pulse().unwrap(), 0x85);
}

#[test]
fn output_p_update_deselected() {
    let mut riot = common::riot_post_reset();
    riot.write_ddra_pulse(0xFF).unwrap();
    riot.write_ddrb_pulse(0xFF).unwrap();
    riot.write_ora_pulse(0x23).unwrap();
    riot.write_orb_pulse(0x45).unwrap();
    riot.write_pa(0x67);
    riot.write_pb(0x89);
    assert_eq!(riot.read_pa().unwrap(), 0x67);
    assert_eq!(riot.read_pb().unwrap(), 0x89);
    riot.deselected_pulse().unwrap();
    assert_eq!(riot.read_pa().unwrap(), 0x23);
    assert_eq!(riot.read_pb().unwrap(), 0x45);
}
