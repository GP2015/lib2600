#[derive(Debug, Default)]
pub struct PossibleIoInstructions {
    pub write_ora: bool,
    pub read_ora: bool,
    pub write_orb: bool,
    pub read_orb: bool,
    pub write_ddra: bool,
    pub read_ddra: bool,
    pub write_ddrb: bool,
    pub read_ddrb: bool,
}

impl PossibleIoInstructions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn only_possible(&self) -> bool {
        [
            self.write_ora,
            self.read_ora,
            self.write_orb,
            self.read_orb,
            self.write_ddra,
            self.read_ddra,
            self.write_ddrb,
            self.read_ddrb,
        ]
        .into_iter()
        .filter(|&b| b)
        .count()
            < 2
    }
}
