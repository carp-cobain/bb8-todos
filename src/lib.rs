// REST routes
pub mod api;

// env var based config
pub mod config;

// high-level database logic (execute queries, map to domain objects)
pub mod repo;

// low-level database logic (connections, pooling)
pub mod db;

// domain objects
pub mod domain;

// project errors
pub mod error;

// Signing and verification
pub mod signer;

// Expose error at top level
pub use error::Error;

// Top level result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
