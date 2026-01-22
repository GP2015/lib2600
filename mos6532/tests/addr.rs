mod common;
use mos6532::Riot;
use rstest::rstest;

#[rstest]
fn use_ram_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] rw: bool,
    #[values(0, 1)] skip: usize,
) {
    for i in 0..2 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(rw),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
fn use_io_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] rw: bool,
    #[values(false, true)] a1: bool,
    #[values(false, true)] a0: bool,
    #[values(0, 1, 2, 3, 4)] skip: usize,
) {
    for i in 0..5 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(rw),
                2 => riot.write_a_bit(2, false).unwrap(),
                3 => riot.write_a_bit(1, a1).unwrap(),
                4 => riot.write_a_bit(0, a0).unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
fn write_timer_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a3: bool,
    #[values(false, true)] a1: bool,
    #[values(false, true)] a0: bool,
    #[values(0, 1, 2, 3, 4, 5, 6)] skip: usize,
) {
    for i in 0..7 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(false),
                2 => riot.write_a_bit(4, true).unwrap(),
                3 => riot.write_a_bit(3, a3).unwrap(),
                4 => riot.write_a_bit(2, true).unwrap(),
                5 => riot.write_a_bit(1, a1).unwrap(),
                6 => riot.write_a_bit(0, a0).unwrap(),
                _ => (),
            }
        }
        assert!(riot.pulse_phi2().is_err());
    }
}

#[rstest]
fn read_timer_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a3: bool,
    #[values(0, 1, 2, 3, 4)] skip: usize,
) {
    for i in 0..5 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(true),
                2 => riot.write_a_bit(3, a3).unwrap(),
                3 => riot.write_a_bit(2, true).unwrap(),
                4 => riot.write_a_bit(0, false).unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
fn read_interrupt_flags_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(0, 1, 2, 3)] skip: usize,
) {
    for i in 0..4 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(true),
                4 => riot.write_a_bit(2, true).unwrap(),
                6 => riot.write_a_bit(0, true).unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}

#[rstest]
fn write_edc_with_uninitialised_pins(
    #[from(common::riot_post_reset_select)] mut riot: Riot,
    #[values(false, true)] a1: bool,
    #[values(false, true)] a0: bool,
    #[values(0, 1, 2, 3, 4, 5)] skip: usize,
) {
    for i in 0..6 {
        if i != skip {
            match i {
                0 => riot.write_rs(true),
                1 => riot.write_rw(false),
                2 => riot.write_a_bit(4, false).unwrap(),
                3 => riot.write_a_bit(2, true).unwrap(),
                4 => riot.write_a_bit(1, a1).unwrap(),
                5 => riot.write_a_bit(0, a0).unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}
