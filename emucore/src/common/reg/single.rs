use crate::common::read::single::SingleRead;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BitReg {
    inner: SingleRead,
}

impl BitReg {
    pub const fn read(&self) -> SingleRead {
        self.inner
    }

    pub const fn set_to_read(&mut self, inner: SingleRead) {
        self.inner = inner;
    }
}

impl From<SingleRead> for BitReg {
    fn from(value: SingleRead) -> Self {
        Self { inner: value }
    }
}

impl From<bool> for BitReg {
    fn from(value: bool) -> Self {
        Self {
            inner: SingleRead::from(value),
        }
    }
}
