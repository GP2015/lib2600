use crate::error::RIOTError;

pub struct Pin {
    name: String,
    state: Option<bool>,
}

impl Pin {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }

    pub fn reset(&mut self) {
        self.state = None;
    }

    pub fn read(&self) -> Result<bool, RIOTError> {
        match self.state {
            Some(state) => Ok(state),
            None => Err(RIOTError::UninitialisedPin {
                pin_name: self.name.clone(),
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
    fn drive_read_pin() {
        let mut pin = Pin::new(String::new());
        pin.drive(true);
        assert_eq!(pin.read().unwrap(), true);
        pin.drive(false);
        assert_eq!(pin.read().unwrap(), false);
    }

    #[test]
    fn read_uninitialised_pin() {
        let pin = Pin::new(String::new());
        assert!(pin.read().is_err());
    }
}
