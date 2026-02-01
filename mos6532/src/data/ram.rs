use emu_utils::register::MBitRegister;

const RAM_SIZE: usize = 128;

pub struct Ram {
    bytes: Vec<MBitRegister>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: (0..RAM_SIZE)
                .map(|byte| MBitRegister::new(8, format!("RAM byte {:x}", byte)))
                .collect(),
        }
    }

    pub fn byte(&mut self, address: usize) -> &mut MBitRegister {
        &mut self.bytes[address]
    }

    pub fn reset(&mut self) {
        self.bytes.iter_mut().for_each(|reg| reg.undefine());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn ram() -> Ram {
        Ram::new()
    }

    #[rstest]
    fn read_uninitialised_byte(mut ram: Ram) {
        assert_eq!(ram.byte(67).state(), vec![None; 8]);
    }

    #[rstest]
    fn reset(mut ram: Ram) {
        ram.byte(23).write(0x45).unwrap();
        ram.byte(67).write(0x89).unwrap();
        ram.reset();
        assert_eq!(ram.byte(23).state(), vec![None; 8]);
        assert_eq!(ram.byte(67).state(), vec![None; 8]);
    }
}
