use crate::error::RIOTError;

pub struct BitReg {
    name: String,
    state: Option<bool>,
}

impl BitReg {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn reset(&mut self) {
        self.state = None;
    }

    pub fn read(&self) -> Result<bool, RIOTError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RIOTError::UninitialisedBitReg {
                reg_name: self.name.clone(),
            }),
        }
    }

    pub fn drive(&mut self, state: bool) {
        self.state = Some(state);
    }

    pub fn is_driven(&self) -> bool {
        match self.state {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_read() {
        let mut bit_reg = BitReg::new(String::new());
        bit_reg.drive(true);
        assert_eq!(bit_reg.read().unwrap(), true);
        bit_reg.drive(false);
        assert_eq!(bit_reg.read().unwrap(), false);
    }

    #[test]
    fn read_uninitialised() {
        let bit_reg = BitReg::new(String::new());
        assert!(bit_reg.read().is_err());
    }

    #[test]
    fn is_driven() {
        let mut bit_reg = BitReg::new(String::new());
        assert_eq!(bit_reg.is_driven(), false);
        bit_reg.drive(true);
        assert_eq!(bit_reg.is_driven(), true);
    }
}
