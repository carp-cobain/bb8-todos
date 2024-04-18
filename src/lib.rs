pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod pool;
pub mod repo;

// Expose error at top level
pub use error::Error;

// Top level result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
