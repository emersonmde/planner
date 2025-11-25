use dioxus::prelude::*;

use crate::components::ui::{Button, ButtonVariant, SettingsModal};
use crate::models::{PlanState, Preferences};
use crate::state::{create_sample_plan, use_plan_state, use_preferences};
use crate::storage;

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
pub fn TopNav(
    active_view: Signal<View>,
    #[props(default)] on_add_team_member: EventHandler<()>,
) -> Element {
    // State for hamburger menu dropdown
    let mut show_menu = use_signal(|| false);
    // State for settings modal
    let mut show_settings = use_signal(|| false);

    // Access state signals
    let mut plan_state = use_plan_state();
    let mut preferences = use_preferences();

    // Get quarter name from plan state
    let quarter_name = plan_state().quarter_name.clone();

    // Calculate capacity metrics
    let plan = plan_state();
    let prefs = preferences();

    // Total team capacity = sum of all team members' capacity
    let total_capacity: f32 = prefs.team_members.iter().map(|m| m.capacity).sum();

    // Total allocated = sum of all allocation percentages converted to weeks
    let total_allocated: f32 = plan
        .allocations
        .iter()
        .flat_map(|a| &a.assignments)
        .map(|a| a.percentage / 100.0)
        .sum();

    // Calculate utilization percentage (capped at 100% for bar width display)
    let utilization_ratio = if total_capacity > 0.0 {
        total_allocated / total_capacity
    } else {
        0.0
    };
    let utilization_pct_display = (utilization_ratio * 100.0).min(100.0);

    // Determine bar color based on utilization thresholds:
    // - Neutral: no allocations (0%)
    // - Warning: under-utilized (<85%) or slightly over (100-110%)
    // - Success: healthy range (85-100%)
    // - Error: over-committed (>110%)
    let bar_class = if total_capacity == 0.0 || total_allocated == 0.0 {
        "capacity-bar-fill" // Neutral - no data
    } else if utilization_ratio < 0.85 {
        "capacity-bar-fill warning" // Under-utilized
    } else if utilization_ratio <= 1.0 {
        "capacity-bar-fill success" // Healthy range
    } else if utilization_ratio <= 1.10 {
        "capacity-bar-fill warning" // Slightly over
    } else {
        "capacity-bar-fill danger" // Over-committed
    };

    // Handle clear preferences (clears both preferences and plan state)
    let handle_clear_preferences = move |_| {
        // Clear from localStorage
        let _ = storage::clear_preferences();
        let _ = storage::clear_plan_state();
        // Reset to defaults
        preferences.set(Preferences::default());
        plan_state.set(PlanState::default());
    };

    // Handle load sample data
    let handle_load_sample_data = move |_| {
        let (sample_prefs, sample_state) = create_sample_plan();
        preferences.set(sample_prefs);
        plan_state.set(sample_state);
    };

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
                "Planner"
            }

            // Quarter display (read-only, configured via plan import)
            div { class: "quarter-display",
                "{quarter_name}"
            }

            // View tabs
            div { class: "view-tabs",
                button {
                    class: if active_view() == View::Allocation { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Allocation),
                    "Allocation"
                }
                button {
                    class: if active_view() == View::Technical { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Technical),
                    "Technical"
                }
                button {
                    class: if active_view() == View::Roadmap { "view-tab active" } else { "view-tab" },
                    onclick: move |_| active_view.set(View::Roadmap),
                    "Roadmap"
                }
            }

            // Capacity indicator
            div { class: "capacity-indicator",
                span { class: "capacity-text", "{total_allocated:.1} / {total_capacity:.0} weeks" }
                div { class: "capacity-bar",
                    div {
                        class: "{bar_class}",
                        style: "width: {utilization_pct_display:.0}%"
                    }
                }
            }

            // Add Team Member button
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| on_add_team_member.call(()),
                "+ Add Member"
            }

            // File menu button with dropdown
            div { class: "file-menu-wrapper",
                button {
                    class: "file-menu-btn",
                    onclick: move |_| show_menu.set(!show_menu()),
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

                // Dropdown menu
                if show_menu() {
                    div {
                        class: "file-menu-backdrop",
                        onclick: move |_| show_menu.set(false),
                    }
                    div { class: "file-menu-dropdown",
                        button {
                            class: "file-menu-item",
                            onclick: move |_| {
                                show_menu.set(false);
                                show_settings.set(true);
                            },
                            span { class: "menu-icon", "⚙" }
                            span { class: "menu-label", "Settings" }
                        }
                    }
                }
            }
        }

        // Settings Modal
        if show_settings() {
            SettingsModal {
                on_clear_preferences: handle_clear_preferences,
                on_load_sample_data: handle_load_sample_data,
                on_close: move |_| show_settings.set(false),
            }
        }
    }
}
