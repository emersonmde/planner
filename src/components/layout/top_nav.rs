use dioxus::prelude::*;

/// Represents the different views in the application
#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Roadmap,
    Technical,
    Allocation,
}

/// Top navigation component with view tabs, quarter selector, and capacity indicator
/// Reference: docs/ui-design.md section 5.1, docs/mockup.html lines 1017-1054
#[component]
pub fn TopNav(active_view: Signal<View>) -> Element {
    rsx! {
        nav { class: "top-nav",
            // App title with icon
            div { class: "app-title",
                svg {
                    width: "20",
                    height: "20",
                    view_box: "0 0 20 20",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    rect { x: "3", y: "3", width: "14", height: "14", rx: "2" }
                    line { x1: "3", y1: "8", x2: "17", y2: "8" }
                    line { x1: "8", y1: "3", x2: "8", y2: "17" }
                }
                "Quarterly Planner"
            }

            // Quarter selector dropdown
            select { class: "quarter-selector",
                option { "Q1 2025" }
                option { "Q2 2025" }
                option { "Q3 2025" }
                option { "Q4 2025" }
            }

            // View tabs
            div { class: "view-tabs",
                button {
                    class: if active_view() == View::Roadmap { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Roadmap),
                    "Roadmap"
                }
                button {
                    class: if active_view() == View::Technical { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Technical),
                    "Technical"
                }
                button {
                    class: if active_view() == View::Allocation { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Allocation),
                    "Allocation"
                }
            }

            // Capacity indicator
            div { class: "capacity-indicator",
                span { class: "capacity-text", "67.5 / 78 weeks" }
                div { class: "capacity-bar",
                    div {
                        class: "capacity-bar-fill success",
                        style: "width: 87%"
                    }
                }
            }

            // File menu button
            button { class: "file-menu-btn",
                svg {
                    width: "20",
                    height: "20",
                    view_box: "0 0 20 20",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    line { x1: "3", y1: "6", x2: "17", y2: "6" }
                    line { x1: "3", y1: "10", x2: "17", y2: "10" }
                    line { x1: "3", y1: "14", x2: "17", y2: "14" }
                }
            }
        }
    }
}
