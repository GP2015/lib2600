use crate::error::RIOTError;

pub struct ValueReg<T: Copy> {
    name: String,
    value: Option<T>,
}

impl<T: Copy> ValueReg<T> {
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }

    pub fn read(&self) -> Result<T, RIOTError> {
        let Some(val) = self.value else {
            return Err(RIOTError::UninitialisedValueReg {
                reg_name: self.name.clone(),
            });
        };

        Ok(val)
    }

    pub fn drive(&mut self, value: T) -> Result<(), RIOTError> {
        self.value = Some(value);
        Ok(())
    }

    pub fn is_driven(&self) -> bool {
        match self.value {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let mut reg = ValueReg::new(String::new());
        reg.drive(0x67).unwrap();
        assert_eq!(reg.read().unwrap(), 0x67);
    }

    #[test]
    fn read_uninitialised() {
        let reg = ValueReg::<usize>::new(String::new());
        assert!(reg.read().is_err());
    }

    #[test]
    fn drive() {
        let mut reg = ValueReg::new(String::new());
        assert!(reg.drive(0x67).is_ok());
    }

    #[test]
    fn is_driven() {
        let mut reg = ValueReg::new(String::new());
        assert_eq!(reg.is_driven(), false);
        assert!(reg.drive(0x67).is_ok());
        assert_eq!(reg.is_driven(), true);
    }
}
