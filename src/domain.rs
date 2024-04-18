use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Story {
    pub id: i32,
    pub name: String,
}

impl Story {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
