pub use crate::riot::core::Riot;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emulator {
    pub riot: Riot,
}
