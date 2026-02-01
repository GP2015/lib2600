use crate::register::RegisterError;
use num_traits::{NumOps, One};

pub struct ValueRegister<T> {
    name: String,
    value: Option<T>,
}

impl<T> ValueRegister<T>
where
    T: Copy + NumOps + One,
{
    pub fn new(name: String) -> Self {
        Self { name, value: None }
    }

    pub fn read(&self) -> Result<T, RegisterError> {
        let Some(val) = self.value else {
            return Err(RegisterError::ReadUndefined {
                name: self.name.clone(),
            });
        };

        Ok(val)
    }

    pub fn write(&mut self, value: T) -> Result<(), RegisterError> {
        self.value = Some(value);
        Ok(())
    }

    pub fn is_written(&self) -> bool {
        self.value.is_some()
    }

    // pub fn increment(&mut self) -> Result<(), RegisterError> {
    //     let Some(val) = self.value else {
    //         return Err(RegisterError::RegisterUninitialised {
    //             name: self.name.clone(),
    //         });
    //     };

    //     self.value = Some(val + T::one());
    //     Ok(())
    // }

    // pub fn decrement(&mut self) -> Result<(), RegisterError> {
    //     let Some(val) = self.value else {
    //         return Err(RegisterError::RegisterUninitialised {
    //             name: self.name.clone(),
    //         });
    //     };

    //     self.value = Some(val - T::one());
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let mut reg = ValueRegister::new(String::new());
        reg.write(0x67).unwrap();
        assert_eq!(reg.read().unwrap(), 0x67);
    }

    #[test]
    fn read_uninitialised() {
        let reg = ValueRegister::<usize>::new(String::new());
        assert!(reg.read().is_err());
    }

    #[test]
    fn write() {
        let mut reg = ValueRegister::new(String::new());
        assert!(reg.write(0x67).is_ok());
    }

    #[test]
    fn is_written() {
        let mut reg = ValueRegister::new(String::new());
        assert!(!reg.is_written());
        assert!(reg.write(0x67).is_ok());
        assert!(reg.is_written());
    }

    // #[test]
    // fn increment() {
    //     let mut reg = ValueRegister::new(String::new());
    //     reg.write(0x67).unwrap();
    //     reg.increment().unwrap();
    //     assert_eq!(reg.read().unwrap(), 0x68);
    // }

    // #[test]
    // fn decrement() {
    //     let mut reg = ValueRegister::new(String::new());
    //     reg.write(0x67).unwrap();
    //     reg.decrement().unwrap();
    //     assert_eq!(reg.read().unwrap(), 0x66);
    // }
}
