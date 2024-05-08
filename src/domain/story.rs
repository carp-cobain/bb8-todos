use serde::Serialize;

/// A story is something that needs to be done; comprised of a set of tasks.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Story {
    pub id: i32,
    pub name: String,
}

impl Story {
    /// Create a new story
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
