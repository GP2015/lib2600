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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_read() {
        let mut bit = BitReg::new(String::new());
        bit.drive(true);
        assert_eq!(bit.read().unwrap(), true);
        bit.drive(false);
        assert_eq!(bit.read().unwrap(), false);
    }

    #[test]
    fn read_uninitialised() {
        let bit = BitReg::new(String::new());
        assert!(bit.read().is_err());
    }
}
