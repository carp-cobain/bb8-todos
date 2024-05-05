use crate::Error;
use serde::Serialize;
use std::str::FromStr;

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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Complete,
    #[default]
    Incomplete,
}

impl FromStr for Status {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s.trim().to_lowercase().as_str() {
            "complete" => Ok(Self::Complete),
            "incomplete" => Ok(Self::Incomplete),
            _ => Err(Error::invalid_args("invalid enum variant".into())),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Complete => f.write_str("complete"),
            Self::Incomplete => f.write_str("incomplete"),
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
    pub fn new(id: i32, story_id: i32, name: String, status: Status) -> Self {
        Self {
            id,
            story_id,
            name,
            status,
        }
    }
}
