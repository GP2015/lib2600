use crate::error::RIOTError;

pub struct Pin {
    state: Option<bool>,
}

impl Pin {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn read(&self) -> Result<bool, RIOTError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RIOTError::UninitialisedPin),
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
    fn drive_read_pin() {
        let mut pin = Pin::new();
        pin.drive(true);
        assert_eq!(pin.read().unwrap(), true);
        pin.drive(false);
        assert_eq!(pin.read().unwrap(), false);
    }

    #[test]
    fn read_uninitialised_pin() {
        let pin = Pin::new();
        assert!(pin.read().is_err());
    }
}
