use strum_macros::Display;

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub enum LineIdent {
    #[strum(to_string = "{name}")]
    UniqueLine { name: &'static str },

    #[strum(to_string = "{bus_name} bit {bit}")]
    BusLine { bus_name: &'static str, bit: usize },
}

impl From<&'static str> for LineIdent {
    fn from(value: &'static str) -> Self {
        Self::UniqueLine { name: value }
    }
}
