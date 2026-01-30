mod common;
use mos6532::{Bus, Riot, SinglePin};
use rstest::{fixture, rstest};

#[fixture]
fn initial_pa7(#[default(false)] state: bool) -> Riot {
    let mut riot = Riot::new();
    riot.pa().drive_in_bit(7, state).unwrap();
    riot.reset_pulse().unwrap();
    riot.res().drive_in(true).unwrap();
    riot
}

#[rstest]
fn edc() {
    let riot = initial_pa7(true);
}
