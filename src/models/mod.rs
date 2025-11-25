//! Data models for the Quarterly Planner application
//!
//! These models represent the core domain entities:
//! - Engineers/Scientists with capacity
//! - Roadmap projects (high-level initiatives)
//! - Technical projects (implementation work)
//! - Weekly allocations
//! - Project color assignments
//!
//! ## State Architecture (Milestone 9)
//!
//! The application uses a two-signal architecture:
//! - `Preferences`: Team config (persisted to localStorage)
//! - `PlanState`: Planning data (exported/imported per quarter)
//! - `PlanExport`: Self-contained export format (combines both)
//!
//! See ADR-004 for design rationale.

mod plan;
mod plan_export;
mod plan_state;
mod preferences;
mod status;

pub use plan::*;
#[allow(unused_imports)] // Used in M13+ for plan export/import
pub use plan_export::*;
pub use plan_state::*;
pub use preferences::*;
pub use status::*;
