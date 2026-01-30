mod common;
use mos6532::{Bus, Riot, SinglePin};
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(rw).unwrap(),
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(rw).unwrap(),
                2 => riot.a().drive_in_bit(2, false).unwrap(),
                3 => riot.a().drive_in_bit(1, a1).unwrap(),
                4 => riot.a().drive_in_bit(0, a0).unwrap(),
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(false).unwrap(),
                2 => riot.a().drive_in_bit(4, true).unwrap(),
                3 => riot.a().drive_in_bit(3, a3).unwrap(),
                4 => riot.a().drive_in_bit(2, true).unwrap(),
                5 => riot.a().drive_in_bit(1, a1).unwrap(),
                6 => riot.a().drive_in_bit(0, a0).unwrap(),
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(true).unwrap(),
                2 => riot.a().drive_in_bit(3, a3).unwrap(),
                3 => riot.a().drive_in_bit(2, true).unwrap(),
                4 => riot.a().drive_in_bit(0, false).unwrap(),
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(true).unwrap(),
                4 => riot.a().drive_in_bit(2, true).unwrap(),
                6 => riot.a().drive_in_bit(0, true).unwrap(),
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
                0 => riot.rs().drive_in(true).unwrap(),
                1 => riot.rw().drive_in(false).unwrap(),
                2 => riot.a().drive_in_bit(4, false).unwrap(),
                3 => riot.a().drive_in_bit(2, true).unwrap(),
                4 => riot.a().drive_in_bit(1, a1).unwrap(),
                5 => riot.a().drive_in_bit(0, a0).unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}
