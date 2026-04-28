use crate::pin::LineConnection;

pub struct Line {
    name: String,
    connections: Vec<LineConnection>,
}

impl Line {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            connections: Vec::new(),
        }
    }
}
