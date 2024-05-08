use super::Status;
use serde::Serialize;

/// A single action item for a story that must be completed.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Task {
    pub id: i32,
    pub story_id: i32,
    pub name: String,
    pub status: Status,
}

impl Task {
    /// Create a new task
    pub fn new(id: i32, story_id: i32, name: String, status: Status) -> Self {
        Self {
            id,
            story_id,
            name,
            status,
        }
    }
}
