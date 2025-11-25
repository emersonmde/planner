//! Planner library - platform-independent modules
//!
//! This library exposes modules that can be tested without platform-specific
//! dependencies. The binary (main.rs) imports these for the actual application.
//!
//! ## Module Structure
//!
//! - `models`: Data structures (projects, allocations, team members)
//! - `utils`: Helper functions (date calculations, quarter helpers)
//!
//! Platform-specific modules (components, storage, plan_io, state) remain
//! in the binary crate to avoid pulling in GUI dependencies during CI testing.

pub mod models;
pub mod utils;
