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

pub fn write_a2_to_a0(riot: &mut Riot, a2: bool, a1: bool, a0: bool) {
    riot.write_a_bit(2, a2).unwrap();
    riot.write_a_bit(1, a1).unwrap();
    riot.write_a_bit(0, a0).unwrap();
}
