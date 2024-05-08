use crate::{
    domain::{Status, Task},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::str::FromStr;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// A page of domain objects
#[derive(Debug, Serialize)]
pub struct Page<T: Serialize> {
    pub prev_page_id: i32,
    pub next_page_id: i32,
    pub data: Vec<T>,
}

impl<T: Serialize> Page<T> {
    // Create a new page of domain objects
    pub fn new(prev_page_id: i32, next_page_id: i32, data: Vec<T>) -> Self {
        Self {
            prev_page_id,
            next_page_id,
            data,
        }
    }
}

/// The request body for creating or updating stories
#[derive(Debug, Deserialize)]
pub struct StoryBody {
    name: String,
}

impl StoryBody {
    /// Sanitize and validate story name from request body
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name.to_string())
    }
}

/// The POST body for creating tasks
#[derive(Debug, Deserialize)]
pub struct CreateTaskBody {
    pub name: String,
    pub story_id: i32,
}

impl CreateTaskBody {
    /// Sanitize and validate task name and story_id from request body
    pub fn validate(&self) -> Result<(i32, String)> {
        // Collects error messages
        let mut messages = Vec::new();

        // Validate body params
        let story_id = self.story_id;
        if story_id <= 0 {
            messages.push("story_id: must be > 0".into());
        }
        let name = self.name.trim();
        if name.is_empty() || name.len() > MAX_NAME_LEN {
            messages.push("name: invalid length".into());
        }

        // Return params or errors
        if messages.is_empty() {
            Ok((story_id, name.to_string()))
        } else {
            Err(Error::InvalidArgs { messages })
        }
    }
}

/// The PATCH body for updating tasks
#[derive(Debug, Deserialize)]
pub struct PatchTaskBody {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl PatchTaskBody {
    /// Helper to validate fields to update for a task.
    pub fn validate(&self, task: Task) -> Result<(String, Status)> {
        // Make sure at least one field is provided
        if self.name.is_none() && self.status.is_none() {
            return Err(Error::invalid_args("name and/or status must be provided"));
        }

        // Defaults
        let mut name = task.name;
        let mut status = task.status;

        // Validate
        let mut messages = Vec::new();
        if let Some(n) = &self.name {
            let n = n.trim();
            if n.is_empty() || n.len() > MAX_NAME_LEN {
                messages.push("name: invalid length".into());
            } else {
                name = n.to_string();
            }
        }
        if let Some(s) = &self.status {
            if let Ok(s) = Status::from_str(s) {
                status = s;
            } else {
                messages.push("status: invalid enum variant".into());
            }
        }

        // Determine result of validation
        if messages.is_empty() {
            Ok((name, status))
        } else {
            Err(Error::InvalidArgs { messages })
        }
    }
}

// The query parameters for getting a page of rows
#[derive(Debug, Deserialize, Default)]
pub struct PagingParams {
    // Page id (last accessed page's max id)
    pub page_id: Option<i32>,
}
