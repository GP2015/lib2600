use crate::common::{HasCouldBe, condition::BaseCondition, read::multi::MultiRead};

include!(concat!(env!("OUT_DIR"), "/cpu_instr.rs"));

impl<const SIZE: usize> HasCouldBe<Instruction> for MultiRead<SIZE> {
    fn could_be(&self, other: &Instruction) -> BaseCondition {
        BaseCondition::No
    }
}
