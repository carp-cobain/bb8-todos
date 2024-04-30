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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Complete,
    Incomplete,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        if value.trim().to_lowercase() == "complete" {
            Self::Complete
        } else {
            Self::Incomplete
        }
    }
}

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::Complete => "complete".into(),
            _ => "incomplete".into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Task {
    pub id: i32,
    pub story_id: i32,
    pub name: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: i32, story_id: i32, name: String, status_str: String) -> Self {
        let status = Status::from(status_str);
        Self {
            id,
            story_id,
            name,
            status,
        }
    }
}
