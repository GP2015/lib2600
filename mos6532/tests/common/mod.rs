use mos6532::Riot;
use rstest::fixture;

#[allow(dead_code)]
#[fixture]
pub fn riot_post_reset() -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, false);
    riot.reset_pulse().unwrap();
    riot.write_rs(true);
    riot
}

#[allow(dead_code)]
#[fixture]
pub fn riot_post_reset_select(#[from(riot_post_reset)] mut riot: Riot) -> Riot {
    riot.select();
    riot
}
