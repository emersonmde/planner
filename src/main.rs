// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;

use components::layout::View;
use components::{AllocationView, RoadmapView, TechnicalView, TopNav};
use models::PlanExport;
use state::AppContext;

/// Define a components module that contains all shared components for our app.
mod components;
mod models;
mod plan_io;
mod state;
mod storage;
mod utils;

/// Check URL for plan data query parameter (?plan=<base64>)
/// Returns (preferences, plan_state, viewing_session) if found and valid
#[cfg(target_family = "wasm")]
fn load_plan_from_url() -> Option<(
    models::Preferences,
    models::PlanState,
    state::ViewingSession,
)> {
    let window = web_sys::window()?;
    let location = window.location();
    let search = location.search().ok()?;

    // Parse query string for "plan" parameter
    if search.is_empty() {
        return None;
    }

    // Simple query string parsing (format: ?plan=<base64>)
    let params: Vec<&str> = search.trim_start_matches('?').split('&').collect();
    let plan_param = params.iter().find(|p| p.starts_with("plan="))?;

    let encoded = plan_param.strip_prefix("plan=")?;
    if encoded.is_empty() {
        return None;
    }

    // URL decode the base64 (in case of + becoming spaces, etc.)
    let decoded_param = js_sys::decode_uri_component(encoded).ok()?;
    let encoded_str = decoded_param.as_string()?;

    // Decode base64
    let json = plan_io::base64_decode(&encoded_str).ok()?;

    // Parse JSON
    let export: PlanExport = serde_json::from_str(&json).ok()?;

    // Validate
    if export.validate().is_err() {
        return None;
    }

    let original_json = serde_json::to_string(&export).unwrap_or_default();
    let (prefs, plan) = export.into_signals();

    Some((
        prefs,
        plan,
        state::ViewingSession {
            filename: "Shared Plan".to_string(),
            original_json,
            modified: false,
        },
    ))
}

#[cfg(not(target_family = "wasm"))]
fn load_plan_from_url() -> Option<(
    models::Preferences,
    models::PlanState,
    state::ViewingSession,
)> {
    None // URL parameters not supported on desktop
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundles smaller
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // Initialize logging - DEBUG in dev, INFO in release
    #[cfg(debug_assertions)]
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG).expect("failed to init logger");
    #[cfg(not(debug_assertions))]
    dioxus::logger::init(dioxus::logger::tracing::Level::INFO).expect("failed to init logger");

    info!("Planner v{} starting", env!("CARGO_PKG_VERSION"));

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
    // Check for plan in URL first (takes priority over localStorage)
    let url_plan = load_plan_from_url();

    // Load preferences and plan state - from URL if present, otherwise from storage
    let (initial_prefs, initial_state, initial_viewing) =
        if let Some((prefs, state, session)) = url_plan {
            info!(
                "Loaded shared plan from URL: {} ({} team members, {} allocations)",
                state.quarter_name,
                prefs.team_members.len(),
                state.allocations.len()
            );
            (prefs, state, Some(session))
        } else {
            let prefs = storage::load_preferences();
            let state = storage::load_plan_state();

            if prefs.is_none() && state.is_none() {
                debug!("No saved data found, using defaults");
            }

            (prefs.unwrap_or_default(), state.unwrap_or_default(), None)
        };

    // Create signals for persistent data
    let preferences = use_signal(|| initial_prefs);
    let plan_state = use_signal(|| initial_state);

    // Viewing session signal for imported plan files (ephemeral, not persisted)
    // When Some, the app is displaying a loaded file instead of the localStorage plan
    let mut viewing_session: Signal<Option<state::ViewingSession>> = use_signal(|| initial_viewing);

    // Auto-save preferences to localStorage when they change
    // IMPORTANT: Skip saving when in viewing mode (viewing an imported file)
    use_effect(move || {
        let prefs = preferences();
        // Only save if NOT in viewing mode
        if viewing_session().is_none() {
            let _ = storage::save_preferences(&prefs);
        }
    });

    // Auto-save plan state to localStorage when it changes
    // IMPORTANT: Skip saving when in viewing mode (viewing an imported file)
    use_effect(move || {
        let state = plan_state();
        // Only save if NOT in viewing mode
        if viewing_session().is_none() {
            let _ = storage::save_plan_state(&state);
        }
    });

    // Detect unsaved changes when in viewing mode
    // Compare current state with original JSON to set the modified flag
    use_effect(move || {
        let prefs = preferences();
        let state = plan_state();

        // Only check if in viewing mode
        if let Some(session) = viewing_session() {
            // Create export from current state and serialize
            let current_export = PlanExport::from_signals(prefs, state);
            let current_json = serde_json::to_string(&current_export).unwrap_or_default();

            // Compare with original - update modified flag if different
            let is_modified = current_json != session.original_json;

            // Only update if the modified state changed
            if is_modified != session.modified {
                viewing_session.set(Some(state::ViewingSession {
                    filename: session.filename.clone(),
                    original_json: session.original_json.clone(),
                    modified: is_modified,
                }));
            }
        }
    });

    // Provide the app context with all signals to child components
    use_context_provider(|| AppContext {
        preferences,
        plan_state,
        viewing_session,
    });

    // Active view state
    let active_view = use_signal(|| View::Allocation);

    rsx! {
        // Critical CSS to prevent white flash on load
        document::Style { "html, body {{ background-color: #0f0f11; }}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "app-container",
            // Top navigation bar
            // Note: + Add Member button moved to grid corner cell in M13
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
