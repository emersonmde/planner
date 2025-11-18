//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to define common UI elements like buttons, forms, and modals, as well as layout and view components.

pub mod layout;
pub mod ui;
pub mod views;

// Re-export commonly used components for convenience
pub use layout::TopNav;
#[allow(unused_imports)] // Reserved for future use in editing UI
pub use ui::{Badge, BadgeType, Button, ButtonVariant};
pub use views::{AllocationView, RoadmapView, TechnicalView};
