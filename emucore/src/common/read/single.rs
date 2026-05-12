use crate::common::mux::{BaseCondition, HasMux, IsCondition};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SingleRead {
    Low,
    High,
    Unknown,
}

impl SingleRead {
    pub const fn new_drive(drive: bool) -> Self {
        if drive { Self::High } else { Self::Low }
    }

    pub const fn could_read_low(self) -> bool {
        matches!(self, Self::Low | Self::Unknown)
    }

    pub const fn could_read_high(self) -> bool {
        matches!(self, Self::High | Self::Unknown)
    }

    pub const fn could_read(self, state: bool) -> bool {
        if state {
            self.could_read_high()
        } else {
            self.could_read_low()
        }
    }

    pub const fn possible_reads(self) -> &'static [bool] {
        match self {
            Self::Low => &[false],
            Self::High => &[true],
            Self::Unknown => &[false, true],
        }
    }

    pub const fn combine_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Low, Self::Low) => Self::Low,
            (Self::High, Self::High) => Self::High,
            _ => Self::Unknown,
        }
    }
}

impl IsCondition for SingleRead {
    fn as_cond(&self) -> BaseCondition {
        match self {
            Self::High => BaseCondition::Yes,
            Self::Low => BaseCondition::No,
            Self::Unknown => BaseCondition::Unknown,
        }
    }
}

impl HasMux for SingleRead {
    fn mux(
        cond: &impl IsCondition,
        low_opt: &impl Fn() -> Self,
        high_opt: &impl Fn() -> Self,
    ) -> Self {
        match cond.as_cond() {
            BaseCondition::No => low_opt(),
            BaseCondition::Yes => high_opt(),
            BaseCondition::Unknown => low_opt().combine_with(high_opt()),
        }
    }
}
