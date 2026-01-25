mod common;
use mos6532::Riot;
use rstest::{fixture, rstest};

#[fixture]
fn initial_pa7(#[default(false)] state: bool) -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, state);
    riot.reset_pulse().unwrap();
    riot.write_res(true);
    riot
}

#[rstest]
fn edc() {
    let riot = initial_pa7(true);
}
