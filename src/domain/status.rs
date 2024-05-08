use crate::Error;
use serde::Serialize;
use std::str::FromStr;

// Status strings
const COMPLETE: &str = "complete";
const INCOMPLETE: &str = "incomplete";

/// Indicates whether a task has been completed.
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
            COMPLETE => Ok(Self::Complete),
            INCOMPLETE => Ok(Self::Incomplete),
            _ => Err(Error::invalid_args("invalid enum variant")),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Complete => f.write_str(COMPLETE),
            Self::Incomplete => f.write_str(INCOMPLETE),
        }
    }
}
