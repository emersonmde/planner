use dioxus::prelude::*;

/// Badge type indicating status
/// Reference: docs/component-reference.md section 2
#[derive(Clone, Copy, PartialEq)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    #[allow(dead_code)] // Reserved for informational badges in future UI
    Info,
}

#[component]
pub fn Badge(
    /// Badge type (success, warning, or error)
    badge_type: BadgeType,
    /// Badge content (typically a number or short text)
    children: Element,
) -> Element {
    let class_name = match badge_type {
        BadgeType::Success => "status-badge success",
        BadgeType::Warning => "status-badge warning",
        BadgeType::Error => "status-badge error",
        BadgeType::Info => "status-badge info",
    };

    let icon = match badge_type {
        BadgeType::Success => rsx! {
            svg {
                class: "status-icon",
                view_box: "0 0 12 12",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                polyline { points: "2,6 5,9 10,3" }
            }
        },
        BadgeType::Warning => rsx! {
            svg {
                class: "status-icon",
                view_box: "0 0 12 12",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                path { d: "M6,2 L10,10 L2,10 Z" }
                line { x1: "6", y1: "5", x2: "6", y2: "7" }
            }
        },
        BadgeType::Error => rsx! {
            svg {
                class: "status-icon",
                view_box: "0 0 12 12",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                line { x1: "6", y1: "3", x2: "6", y2: "7" }
                circle { cx: "6", cy: "9", r: "0.5", fill: "currentColor" }
            }
        },
        BadgeType::Info => rsx! {
            svg {
                class: "status-icon",
                view_box: "0 0 12 12",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                circle { cx: "6", cy: "6", r: "4" }
                line { x1: "6", y1: "5", x2: "6", y2: "8" }
            }
        },
    };

    rsx! {
        span { class: "{class_name}",
            {icon}
            {children}
        }
    }
}
