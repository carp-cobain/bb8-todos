use crate::{
    domain::{Status, Task},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Limit name size in http request body.
const MAX_NAME_LEN: usize = 100;

/// Combine a story with its tasks to send to the client.
#[derive(Debug, Serialize)]
pub struct StoryDto {
    pub id: i32,
    pub name: String,
    pub tasks: Vec<Task>,
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

        // Validate all body params
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
    pub fn validate(&self) -> Result<()> {
        // Make sure at least one field is provided
        if self.name.is_none() && self.status.is_none() {
            return Err(Error::invalid_args("name and/or status must be provided"));
        }

        // Validate
        let mut messages = Vec::new();
        if let Some(n) = &self.name {
            let n = n.trim();
            if n.is_empty() || n.len() > MAX_NAME_LEN {
                messages.push("name: invalid length".into());
            }
        }
        if let Some(s) = &self.status {
            let s = s.trim().to_lowercase();
            if s != "complete" && s != "incomplete" {
                messages.push("status: invalid enum variant".into());
            }
        }
        if !messages.is_empty() {
            return Err(Error::InvalidArgs { messages });
        }

        Ok(())
    }

    /// Helper to unwrap fields to update for a task, falling back to existing values.
    pub fn unwrap(self, task: Task) -> (String, Status) {
        let name = self.name.unwrap_or(task.name);
        let status = match self.status {
            Some(s) => Status::from(s),
            None => task.status,
        };
        (name, status)
    }
}
