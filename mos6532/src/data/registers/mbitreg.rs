use crate::data::registers::state::{MBitRegState, RegBitState};

pub struct MBitReg {
    bits: Vec<RegBitState>,
    size: usize,
}

impl MBitReg {
    pub fn new(size: usize) -> Self {
        Self {
            bits: vec![RegBitState::Undefined; size],
            size,
        }
    }

    pub fn get(&self) -> MBitRegState {
        let mut combined = 0;
        for bit in (0..self.size).rev() {
            combined = match self.bits[bit] {
                RegBitState::Undefined => return MBitRegState::Undefined,
                RegBitState::False => combined << 1,
                RegBitState::True => (combined << 1) + 1,
            }
        }
        MBitRegState::Val(combined)
    }

    pub fn get_bit(&self, bit: usize) -> RegBitState {
        self.bits[bit % self.size]
    }

    pub fn set(&mut self, state: MBitRegState) {
        match state {
            MBitRegState::Undefined => self.bits = vec![RegBitState::Undefined; self.size],
            MBitRegState::Val(val) => {
                for bit in 0..self.size {
                    self.bits[bit] = match (val >> bit) & 1 == 1 {
                        true => RegBitState::True,
                        false => RegBitState::False,
                    }
                }
            }
        }
    }

    pub fn set_bit(&mut self, bit: usize, state: RegBitState) {
        self.bits[bit % self.size] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn reg() -> MBitReg {
        MBitReg::new(8)
    }

    #[rstest]
    fn get_initial(reg: MBitReg) {
        assert_eq!(reg.get(), MBitRegState::Undefined);
    }

    #[rstest]
    #[case(MBitRegState::Val(0x67), MBitRegState::Val(0x67))]
    #[case(MBitRegState::Val(0x167), MBitRegState::Val(0x67))]
    #[case(MBitRegState::Undefined, MBitRegState::Undefined)]
    fn set_get(mut reg: MBitReg, #[case] set: MBitRegState, #[case] get: MBitRegState) {
        reg.set(set);
        assert_eq!(reg.get(), get);
    }

    #[rstest]
    fn get_undefined(mut reg: MBitReg) {
        reg.set(MBitRegState::Val(0x67));
        reg.set_bit(6, RegBitState::Undefined);
        assert_eq!(reg.get(), MBitRegState::Undefined);
    }

    #[rstest]
    fn set_undefined(mut reg: MBitReg, #[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: usize) {
        reg.set(MBitRegState::Val(0x67));
        reg.set(MBitRegState::Undefined);
        assert_eq!(reg.get_bit(bit), RegBitState::Undefined);
    }

    #[rstest]
    fn get_initial_bit(reg: MBitReg, #[values(0, 1, 2, 3, 4, 5, 6, 7)] bit: usize) {
        assert_eq!(reg.get_bit(bit), RegBitState::Undefined);
    }

    #[rstest]
    #[case(1, RegBitState::True)]
    #[case(3, RegBitState::False)]
    #[case(8, RegBitState::False)]
    #[case(10, RegBitState::True)]
    fn get_bits(mut reg: MBitReg, #[case] bit: usize, #[case] state: RegBitState) {
        reg.set(MBitRegState::Val(0b11010110));
        assert_eq!(reg.get_bit(bit), state);
    }

    #[rstest]
    fn set_bits(mut reg: MBitReg) {
        for i in 0..8 {
            assert_eq!(reg.get(), MBitRegState::Undefined);
            reg.set_bit(i, RegBitState::False);
        }
        assert_eq!(reg.get(), MBitRegState::Val(0));
    }

    #[rstest]
    fn get_set_bits(mut reg: MBitReg) {
        reg.set_bit(6, RegBitState::True);
        reg.set_bit(7, RegBitState::False);
        assert_eq!(reg.get_bit(6), RegBitState::True);
        assert_eq!(reg.get_bit(7), RegBitState::False);
    }
}
