#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegBitState {
    True,
    False,
    Undefined,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBitRegState {
    Val(usize),
    Undefined,
}
