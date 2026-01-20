use mos6532::Riot;

#[allow(dead_code)]
pub fn riot_post_reset() -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, false);
    riot.reset_pulse().unwrap();
    riot
}

#[allow(dead_code)]
pub fn riot_post_reset_select() -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, false);
    riot.reset_pulse().unwrap();
    riot.select();
    riot
}

// #[allow(dead_code)]
// pub fn riot_post_select() -> Riot {
//     let mut riot = Riot::new();
//     riot.select();
//     riot
// }
