mod common;
use mos6532::Riot;
use rstest::{fixture, rstest};

#[fixture]
fn initial_pa7(#[default(false)] state: bool) -> Riot {
    let mut riot = Riot::new();
    riot.pa_mut()
        .pin_mut(7)
        .unwrap()
        .add_drive_in(state, true)
        .unwrap();
    riot.reset_pulse().unwrap();
    riot.res_mut().add_high_in(true).unwrap();
    riot
}

#[rstest]
fn edc(
    #[from(initial_pa7)]
    #[with(false)]
    riot: Riot,
) {
    //
}
