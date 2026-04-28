use getset::CopyGetters;

#[derive(CopyGetters, Debug)]
pub struct LineConnection {
    #[get_copy = "pub(crate)"]
    id: usize,
}

impl LineConnection {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, CopyGetters)]
pub struct BusConnection {
    #[get_copy = "pub(crate)"]
    id: usize,
}

impl BusConnection {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}
