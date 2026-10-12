#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum WriteLoc {
    Phi2,
    Addr,
    Db,
    Rw,
    InstrCycles,
    Acc,
    X,
    Y,
    Pc,
    Sp,
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum WriteKind {}

struct CycleWriteEffect {
    loc: WriteLoc,
    kind: WriteKind,
}

enum InstrKind {}
