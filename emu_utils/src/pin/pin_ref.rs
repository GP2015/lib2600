pub trait RefType {}
pub struct Immutable;
pub struct Mutable;
impl RefType for Immutable {}
impl RefType for Mutable {}

pub enum ObjRef<'a, O> {
    Immutable(&'a O),
    Mutable(&'a mut O),
}

impl<'a, O> ObjRef<'a, O> {
    pub fn as_ref(&self) -> &O {
        match self {
            ObjRef::Immutable(pin) => pin,
            ObjRef::Mutable(pin) => pin,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut O> {
        if let ObjRef::Mutable(pin) = self {
            Some(pin)
        } else {
            None
        }
    }
}
