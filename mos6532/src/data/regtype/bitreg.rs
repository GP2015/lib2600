use crate::error::RiotError;

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

    pub fn read(&self) -> Result<bool, RiotError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RiotError::UninitialisedBitReg {
                reg_name: self.name.clone(),
            }),
        }
    }

    pub fn drive(&mut self, state: bool) {
        self.state = Some(state);
    }

    pub fn is_driven(&self) -> bool {
        self.state.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_read() {
        let mut reg = BitReg::new(String::new());
        reg.drive(true);
        assert!(reg.read().unwrap());
        reg.drive(false);
        assert!(!reg.read().unwrap());
    }

    #[test]
    fn read_uninitialised() {
        let reg = BitReg::new(String::new());
        assert!(reg.read().is_err());
    }

    #[test]
    fn is_driven() {
        let mut reg = BitReg::new(String::new());
        assert!(!reg.is_driven());
        reg.drive(true);
        assert!(reg.is_driven());
    }
}
