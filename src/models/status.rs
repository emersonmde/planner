//! Status types shared between models and UI components

/// Badge/status type indicating capacity or progress state
/// Used by both model calculations and UI Badge component
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    #[allow(dead_code)] // Reserved for informational badges in future UI
    Info,
    /// Neutral state - no status indication (e.g., 0/0 allocation)
    Neutral,
}
