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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_drive_in(rw, true).unwrap(),
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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_drive_in(rw, true).unwrap(),
                2 => riot.a_mut().pin_mut(2).unwrap().add_low_in(true).unwrap(),
                3 => riot
                    .a_mut()
                    .pin_mut(1)
                    .unwrap()
                    .add_drive_in(a1, true)
                    .unwrap(),
                4 => riot
                    .a_mut()
                    .pin_mut(0)
                    .unwrap()
                    .add_drive_in(a0, true)
                    .unwrap(),
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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_low_in(true).unwrap(),
                2 => riot.a_mut().pin_mut(4).unwrap().add_high_in(true).unwrap(),
                3 => riot
                    .a_mut()
                    .pin_mut(3)
                    .unwrap()
                    .add_drive_in(a3, true)
                    .unwrap(),
                4 => riot.a_mut().pin_mut(2).unwrap().add_high_in(true).unwrap(),
                5 => riot
                    .a_mut()
                    .pin_mut(1)
                    .unwrap()
                    .add_drive_in(a1, true)
                    .unwrap(),
                6 => riot
                    .a_mut()
                    .pin_mut(0)
                    .unwrap()
                    .add_drive_in(a0, true)
                    .unwrap(),
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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_high_in(true).unwrap(),
                2 => riot
                    .a_mut()
                    .pin_mut(3)
                    .unwrap()
                    .add_drive_in(a3, true)
                    .unwrap(),
                3 => riot.a_mut().pin_mut(2).unwrap().add_high_in(true).unwrap(),
                4 => riot.a_mut().pin_mut(0).unwrap().add_low_in(true).unwrap(),
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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_high_in(true).unwrap(),
                4 => riot.a_mut().pin_mut(2).unwrap().add_high_in(true).unwrap(),
                6 => riot.a_mut().pin_mut(0).unwrap().add_high_in(true).unwrap(),
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
                0 => riot.rs_mut().add_high_in(true).unwrap(),
                1 => riot.rw_mut().add_low_in(true).unwrap(),
                2 => riot.a_mut().pin_mut(4).unwrap().add_low_in(true).unwrap(),
                3 => riot.a_mut().pin_mut(2).unwrap().add_high_in(true).unwrap(),
                4 => riot
                    .a_mut()
                    .pin_mut(1)
                    .unwrap()
                    .add_drive_in(a1, true)
                    .unwrap(),
                5 => riot
                    .a_mut()
                    .pin_mut(0)
                    .unwrap()
                    .add_drive_in(a0, true)
                    .unwrap(),
                _ => (),
            }
        }
    }
    assert!(riot.pulse_phi2().is_err());
}
