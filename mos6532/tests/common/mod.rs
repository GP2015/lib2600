use mos6532::Riot;
use rstest::fixture;

#[allow(dead_code)]
#[fixture]
pub fn riot_post_reset() -> Riot {
    let mut riot = Riot::new();
    riot.reset_pulse().unwrap();
    riot.res_mut().add_high_in(true).unwrap();
    riot
}

#[allow(dead_code)]
#[fixture]
pub fn riot_post_reset_select(#[from(riot_post_reset)] mut riot: Riot) -> Riot {
    riot.select().unwrap();
    riot
}
