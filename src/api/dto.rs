use crate::{Error, Result};
use serde::Deserialize;
use std::fmt::Debug;

/// Limit name size in http request body.
const MAX_STORY_NAME_LEN: usize = 100;

/// The POST body for creating stories
#[derive(Debug, Deserialize)]
pub struct CreateStoryBody {
    name: String,
}

impl CreateStoryBody {
    /// Sanitize and validate story name from request body
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim();
        if name.is_empty() || name.len() > MAX_STORY_NAME_LEN {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name.to_string())
    }
}
