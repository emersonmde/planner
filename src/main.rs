// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::layout::View;
use components::{AllocationView, RoadmapView, TechnicalView, TopNav};
use state::{create_sample_plan, AppContext};

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
    // Load preferences from localStorage or use sample data
    let (initial_prefs, initial_state) = {
        let (sample_prefs, sample_state) = create_sample_plan();

        // Try to load preferences from localStorage (web only)
        let prefs = storage::load_preferences().unwrap_or(sample_prefs);

        (prefs, sample_state)
    };

    // Create two independent signals
    let preferences = use_signal(|| initial_prefs);
    let plan_state = use_signal(|| initial_state);

    // Auto-save preferences to localStorage when they change
    use_effect(move || {
        let prefs = preferences();
        // Save in background (ignore errors for now - could log them in production)
        let _ = storage::save_preferences(&prefs);
    });

    // Provide the app context with two signals to all child components
    use_context_provider(|| AppContext {
        preferences,
        plan_state,
    });

    // Active view state
    let active_view = use_signal(|| View::Allocation);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            // Top navigation bar
            TopNav { active_view }

            // Main content area
            main { class: "main-content",
                // Render the appropriate view based on active_view
                match active_view() {
                    View::Roadmap => rsx! { RoadmapView {} },
                    View::Technical => rsx! { TechnicalView {} },
                    View::Allocation => rsx! { AllocationView {} },
                }
            }
        }
    }
}
