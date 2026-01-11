use mos6532::Riot;

pub fn riot_post_reset() -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, false);
    riot.reset_pulse().unwrap();
    riot
}

pub fn riot_post_reset_select() -> Riot {
    let mut riot = Riot::new();
    riot.write_pa_bit(7, false);
    riot.reset_pulse().unwrap();
    riot.select();
    riot
}

pub fn riot_post_select() -> Riot {
    let mut riot = Riot::new();
    riot.select();
    riot
}
