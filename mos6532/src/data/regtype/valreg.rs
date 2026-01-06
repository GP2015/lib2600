use crate::error::RIOTError;
use num_traits::{NumOps, One};

pub struct ValueReg<T> {
    name: String,
    value: Option<T>,
}

impl<T> ValueReg<T>
where
    T: Copy + NumOps + One,
{
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

    pub fn increment(&mut self) -> Result<(), RIOTError> {
        let Some(val) = self.value else {
            return Err(RIOTError::UninitialisedValueReg {
                reg_name: self.name.clone(),
            });
        };

        self.value = Some(val + T::one());
        Ok(())
    }

    pub fn decrement(&mut self) -> Result<(), RIOTError> {
        let Some(val) = self.value else {
            return Err(RIOTError::UninitialisedValueReg {
                reg_name: self.name.clone(),
            });
        };

        self.value = Some(val - T::one());
        Ok(())
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

    #[test]
    fn increment() {
        let mut reg = ValueReg::new(String::new());
        reg.drive(0x67).unwrap();
        reg.increment().unwrap();
        assert_eq!(reg.read().unwrap(), 0x68);
    }

    #[test]
    fn decrement() {
        let mut reg = ValueReg::new(String::new());
        reg.drive(0x67).unwrap();
        reg.decrement().unwrap();
        assert_eq!(reg.read().unwrap(), 0x66);
    }
}
