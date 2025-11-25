//! planner-core: Platform-independent models and utilities for the Quarterly Planner
//!
//! This crate contains all data models and utility functions that don't depend on
//! platform-specific code (web, desktop). This allows testing on any platform.

pub mod models;
pub mod utils;

// Re-export commonly used types for convenience
pub use models::*;
pub use utils::*;
