#[derive(Debug, Default)]
pub struct PossibleInstructions {
    pub nop: bool,
    pub reset: bool,
    pub ram: bool,
    pub io: bool,
    pub write_timer: bool,
    pub read_timer: bool,
    pub read_interrupt_flag: bool,
    pub write_edc: bool,
}

impl PossibleInstructions {
    pub fn only_possible(&self) -> bool {
        [
            self.nop,
            self.reset,
            self.ram,
            self.io,
            self.write_timer,
            self.read_timer,
            self.read_interrupt_flag,
            self.write_edc,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            < 2
    }
}
