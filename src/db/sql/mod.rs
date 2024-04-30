/// Queries for the "stories" table
pub mod stories;

/// Queries for the "tasks" table
pub mod tasks;

/// Supports tables existing in multiple schemas.
pub const SET_SEARCH_PATH: &str = "set search_path to public,bb8_todos";
