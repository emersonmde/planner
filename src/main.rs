// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::layout::View;
use components::ui::{TeamMemberModal, TeamMemberModalMode};
use components::{AllocationView, RoadmapView, TechnicalView, TopNav};
use models::Role;
use state::AppContext;

/// Define a components module that contains all shared components for our app.
mod components;
mod models;
mod state;
mod storage;
mod utils;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundles smaller
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Load preferences and plan state from storage, falling back to smart defaults
    let initial_prefs = storage::load_preferences().unwrap_or_default();
    let initial_state = storage::load_plan_state().unwrap_or_default();

    // Create two independent signals
    let mut preferences = use_signal(|| initial_prefs);
    let plan_state = use_signal(|| initial_state);

    // Auto-save preferences to localStorage when they change
    use_effect(move || {
        let prefs = preferences();
        let _ = storage::save_preferences(&prefs);
    });

    // Auto-save plan state to localStorage when it changes
    use_effect(move || {
        let state = plan_state();
        let _ = storage::save_plan_state(&state);
    });

    // Provide the app context with two signals to all child components
    use_context_provider(|| AppContext {
        preferences,
        plan_state,
    });

    // Active view state
    let active_view = use_signal(|| View::Allocation);

    // Team member modal state
    let mut show_team_member_modal = use_signal(|| false);

    // Handle adding new team member
    let handle_add_team_member = move |_| {
        show_team_member_modal.set(true);
    };

    // Handle save team member
    let handle_save_team_member = move |member: models::TeamMember| {
        preferences.with_mut(|p| {
            p.team_members.push(member);
        });
        show_team_member_modal.set(false);
    };

    // Handle cancel team member modal
    let handle_cancel_team_member = move |_| {
        show_team_member_modal.set(false);
    };

    // Get default capacity for new members
    let default_capacity = preferences().default_capacity;

    rsx! {
        // Critical CSS to prevent white flash on load
        document::Style { "html, body {{ background-color: #0f0f11; }}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            // Top navigation bar
            TopNav {
                active_view,
                on_add_team_member: handle_add_team_member,
            }

            // Main content area
            main { class: "main-content",
                // Render the appropriate view based on active_view
                match active_view() {
                    View::Roadmap => rsx! { RoadmapView {} },
                    View::Technical => rsx! { TechnicalView {} },
                    View::Allocation => rsx! { AllocationView {} },
                }
            }

            // Team member modal (Add mode from TopNav)
            if show_team_member_modal() {
                TeamMemberModal {
                    mode: TeamMemberModalMode::Add,
                    initial_name: String::new(),
                    initial_role: Role::Engineering,
                    initial_capacity: 0.0,
                    default_capacity,
                    allocated_weeks: 0.0,
                    on_save: handle_save_team_member,
                    on_cancel: handle_cancel_team_member,
                }
            }
        }
    }
}
