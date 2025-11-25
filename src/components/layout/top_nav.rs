use dioxus::logger::tracing::info;
use dioxus::prelude::*;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsCast;

use crate::components::ui::SettingsModal;
use crate::models::{PlanExport, PlanState, Preferences};
use crate::plan_io::trigger_plan_download;
use crate::state::{create_sample_plan, use_plan_state, use_preferences, use_viewing_session};
use crate::storage;

/// Represents the different views in the application
#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Roadmap,
    Technical,
    Allocation,
}

/// Top navigation component with view tabs, plan menu, and capacity indicator
#[component]
pub fn TopNav(active_view: Signal<View>) -> Element {
    let show_plan_menu = use_signal(|| false);
    let mut show_settings = use_signal(|| false);

    let mut plan_state = use_plan_state();
    let mut preferences = use_preferences();
    let viewing_session = use_viewing_session();

    let plan = plan_state();
    let prefs = preferences();

    // Calculate capacity metrics
    let (total_capacity, total_allocated, bar_class) = calculate_capacity_metrics(&plan, &prefs);
    let utilization_pct = calculate_utilization_display(total_allocated, total_capacity);

    // Viewing mode state
    let is_viewing = viewing_session().is_some();
    let viewing_filename = viewing_session().as_ref().map(|s| s.filename.clone());
    let viewing_modified = viewing_session()
        .as_ref()
        .map(|s| s.modified)
        .unwrap_or(false);

    let plan_menu_title = viewing_filename
        .clone()
        .unwrap_or_else(|| plan.quarter_name.clone());

    // Hidden file input for web imports
    let file_input_id = use_signal(|| "plan-file-input".to_string());

    // Register keyboard shortcuts (web only)
    #[cfg(target_family = "wasm")]
    register_keyboard_shortcuts(file_input_id, preferences, plan_state);

    rsx! {
        // Hidden file input for import (web only - desktop uses native dialog)
        input {
            r#type: "file",
            id: "{file_input_id()}",
            accept: ".json,application/json",
            style: "display: none;",
            onchange: move |_| {
                #[cfg(target_family = "wasm")]
                handle_file_import(file_input_id(), preferences, plan_state, viewing_session);
            },
        }

        nav { class: "top-nav",
            // App title
            div { class: "app-title",
                AppIcon {}
                "Planner"
            }

            // Plan Menu
            PlanMenu {
                is_viewing,
                plan_menu_title,
                viewing_modified,
                show_plan_menu,
                file_input_id,
                preferences,
                plan_state,
                viewing_session,
            }

            // View tabs
            ViewTabs { active_view }

            // Capacity indicator
            CapacityIndicator {
                total_allocated,
                total_capacity,
                bar_class,
                utilization_pct,
            }

            // Close button (viewing mode only)
            if is_viewing {
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| restore_from_local_storage(preferences, plan_state, viewing_session),
                    "Close"
                }
            }

            // Settings button
            button {
                class: "settings-btn",
                onclick: move |_| show_settings.set(true),
                title: "Settings",
                "⚙️"
            }
        }

        // Settings Modal
        if show_settings() {
            SettingsModal {
                on_clear_preferences: move |_| {
                    info!("Clearing all preferences and plan state");
                    let _ = storage::clear_preferences();
                    let _ = storage::clear_plan_state();
                    preferences.set(Preferences::default());
                    plan_state.set(PlanState::default());
                },
                on_load_sample_data: move |_| {
                    info!("Loading sample plan data");
                    let (sample_prefs, sample_state) = create_sample_plan();
                    preferences.set(sample_prefs);
                    plan_state.set(sample_state);
                },
                on_close: move |_| show_settings.set(false),
            }
        }
    }
}

// =============================================================================
// Sub-components
// =============================================================================

/// App icon SVG
#[component]
fn AppIcon() -> Element {
    rsx! {
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
    }
}

/// Plan menu dropdown with file operations
#[component]
fn PlanMenu(
    is_viewing: bool,
    plan_menu_title: String,
    viewing_modified: bool,
    mut show_plan_menu: Signal<bool>,
    file_input_id: Signal<String>,
    mut preferences: Signal<Preferences>,
    mut plan_state: Signal<PlanState>,
    mut viewing_session: Signal<Option<crate::state::ViewingSession>>,
) -> Element {
    rsx! {
        div { class: "plan-menu-wrapper",
            // Menu button
            button {
                class: if is_viewing { "plan-menu-btn viewing" } else { "plan-menu-btn" },
                onclick: move |_| show_plan_menu.set(!show_plan_menu()),
                if is_viewing {
                    span { class: "plan-menu-icon", "📄" }
                }
                span { class: "plan-menu-title", "{plan_menu_title}" }
                if viewing_modified {
                    span { class: "plan-menu-modified", "•" }
                }
                ChevronIcon {}
            }

            // Dropdown
            if show_plan_menu() {
                div {
                    class: "plan-menu-backdrop",
                    onclick: move |_| show_plan_menu.set(false),
                }
                div { class: "plan-menu-dropdown",
                    if is_viewing {
                        ViewingModeMenu {
                            viewing_modified,
                            show_plan_menu,
                            preferences,
                            plan_state,
                            viewing_session,
                        }
                    } else {
                        NormalModeMenu {
                            show_plan_menu,
                            file_input_id,
                            preferences,
                            plan_state,
                            viewing_session,
                        }
                    }
                }
            }
        }
    }
}

/// Menu items when viewing an imported plan
#[component]
fn ViewingModeMenu(
    viewing_modified: bool,
    mut show_plan_menu: Signal<bool>,
    mut preferences: Signal<Preferences>,
    mut plan_state: Signal<PlanState>,
    mut viewing_session: Signal<Option<crate::state::ViewingSession>>,
) -> Element {
    rsx! {
        // Save to File
        MenuItem {
            icon: "💾",
            label: "Save to File...",
            shortcut: Some("⌘S"),
            onclick: move |_| {
                show_plan_menu.set(false);
                let export = PlanExport::from_signals(preferences(), plan_state());
                let _ = trigger_plan_download(&export);
            },
        }

        // Adopt This Plan
        MenuItem {
            icon: "📌",
            label: "Adopt This Plan",
            onclick: move |_| {
                info!("Adopting viewed plan as local plan");
                show_plan_menu.set(false);
                let _ = storage::save_preferences(&preferences());
                let _ = storage::save_plan_state(&plan_state());
                viewing_session.set(None);
                // Clear URL since user now owns this plan
                crate::plan_io::clear_url_plan_param();
            },
        }

        div { class: "plan-menu-separator" }

        // Discard Changes
        button {
            class: "plan-menu-item",
            disabled: !viewing_modified,
            onclick: move |_| {
                show_plan_menu.set(false);
                if let Some(session) = viewing_session() {
                    if let Ok(export) = serde_json::from_str::<PlanExport>(&session.original_json) {
                        let (original_prefs, original_state) = export.into_signals();
                        preferences.set(original_prefs);
                        plan_state.set(original_state);
                        viewing_session.set(Some(crate::state::ViewingSession {
                            filename: session.filename,
                            original_json: session.original_json,
                            modified: false,
                        }));
                    }
                }
            },
            span { class: "menu-icon", "↩" }
            span { class: "menu-label", "Discard Changes" }
        }

        // Close
        MenuItem {
            icon: "✕",
            label: "Close",
            onclick: move |_| {
                show_plan_menu.set(false);
                restore_from_local_storage(preferences, plan_state, viewing_session);
            },
        }
    }
}

/// Menu items in normal (non-viewing) mode
#[component]
fn NormalModeMenu(
    mut show_plan_menu: Signal<bool>,
    file_input_id: Signal<String>,
    mut preferences: Signal<Preferences>,
    mut plan_state: Signal<PlanState>,
    mut viewing_session: Signal<Option<crate::state::ViewingSession>>,
) -> Element {
    rsx! {
        // Open Plan
        button {
            class: "plan-menu-item",
            onclick: move |_| async move {
                // NOTE: Don't close menu before async operation - component unmount drops the future
                #[cfg(target_family = "wasm")]
                {
                    show_plan_menu.set(false);
                    trigger_file_open(file_input_id());
                }
                #[cfg(not(target_family = "wasm"))]
                {
                    use dioxus::logger::tracing::{debug, error, info};
                    debug!("Opening file dialog (async handler)");
                    let file = rfd::AsyncFileDialog::new()
                        .add_filter("Plan Files", &["json"])
                        .add_filter("All Files", &["*"])
                        .set_title("Open Plan")
                        .pick_file()
                        .await;
                    // Close menu after dialog returns (not before, or future gets dropped)
                    show_plan_menu.set(false);
                    info!("Dialog returned: {:?}", file.is_some());
                    if let Some(file) = file {
                        let filename = file.file_name();
                        let path = file.path().to_path_buf();
                        info!("Reading file: {} from {:?}", filename, path);
                        // Use sync read - async read() doesn't complete properly in Dioxus
                        match std::fs::read_to_string(&path) {
                            Ok(json_str) => {
                                info!("File read, {} bytes", json_str.len());
                                info!("Parsing JSON...");
                                match load_plan_from_json(
                                    &json_str,
                                    &filename,
                                    &mut preferences,
                                    &mut plan_state,
                                    &mut viewing_session,
                                ) {
                                    Ok(()) => info!("Plan loaded successfully!"),
                                    Err(e) => error!("Failed to load plan: {}", e),
                                }
                            }
                            Err(e) => error!("Failed to read file: {}", e),
                        }
                    } else {
                        info!("Dialog was cancelled");
                    }
                }
            },
            span { class: "menu-icon", "📂" }
            span { class: "menu-label", "Open Plan..." }
            span { class: "menu-shortcut", "⌘O" }
        }

        // Save Plan
        button {
            class: "plan-menu-item",
            onclick: move |_| async move {
                #[cfg(target_family = "wasm")]
                {
                    show_plan_menu.set(false);
                    let export = PlanExport::from_signals(preferences(), plan_state());
                    let _ = trigger_plan_download(&export);
                }
                #[cfg(not(target_family = "wasm"))]
                {
                    use dioxus::logger::tracing::{debug, error, info};
                    let export = PlanExport::from_signals(preferences(), plan_state());
                    let json = match serde_json::to_string_pretty(&export) {
                        Ok(j) => j,
                        Err(e) => {
                            error!("Failed to serialize: {}", e);
                            return;
                        }
                    };
                    let filename = format!(
                        "plan-{}-{}.json",
                        export.team_name.to_lowercase().replace(' ', "-"),
                        export.quarter_name.to_lowercase().replace(' ', "-")
                    );
                    debug!("Opening save dialog (async handler)");
                    let file = rfd::AsyncFileDialog::new()
                        .add_filter("Plan Files", &["json"])
                        .set_file_name(&filename)
                        .set_title("Save Plan")
                        .save_file()
                        .await;
                    // Close menu after dialog returns (not before, or future gets dropped)
                    show_plan_menu.set(false);
                    debug!("Save dialog returned: {:?}", file.is_some());
                    if let Some(file) = file {
                        let path = file.path().to_path_buf();
                        // Use sync write - async write() doesn't complete properly in Dioxus
                        if let Err(e) = std::fs::write(&path, &json) {
                            error!("Failed to write file: {}", e);
                        } else {
                            info!("Plan saved to {:?}", path);
                        }
                    }
                }
            },
            span { class: "menu-icon", "💾" }
            span { class: "menu-label", "Save Plan..." }
            span { class: "menu-shortcut", "⌘S" }
        }

        div { class: "plan-menu-separator" }

        // Copy to Clipboard
        MenuItem {
            icon: "📋",
            label: "Copy to Clipboard",
            onclick: move |_| {
                show_plan_menu.set(false);
                let export = PlanExport::from_signals(preferences(), plan_state());
                let _ = crate::plan_io::copy_plan_to_clipboard(&export);
            },
        }

        // Copy Link (shareable URL)
        MenuItem {
            icon: "🔗",
            label: "Copy Link",
            onclick: move |_| {
                show_plan_menu.set(false);
                let export = PlanExport::from_signals(preferences(), plan_state());
                let _ = crate::plan_io::copy_shareable_url(&export);
            },
        }

        // Paste from Clipboard
        MenuItem {
            icon: "📥",
            label: "Paste from Clipboard",
            onclick: move |_| {
                show_plan_menu.set(false);
                #[cfg(target_family = "wasm")]
                handle_paste_from_clipboard(preferences, plan_state, viewing_session);
                #[cfg(not(target_family = "wasm"))]
                handle_paste_from_clipboard_desktop(preferences, plan_state, viewing_session);
            },
        }
    }
}

/// Reusable menu item component
#[component]
fn MenuItem(
    icon: &'static str,
    label: &'static str,
    #[props(default)] shortcut: Option<&'static str>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "plan-menu-item",
            onclick: move |e| onclick.call(e),
            span { class: "menu-icon", "{icon}" }
            span { class: "menu-label", "{label}" }
            if let Some(shortcut) = shortcut {
                span { class: "menu-shortcut", "{shortcut}" }
            }
        }
    }
}

/// View tabs component
#[component]
fn ViewTabs(mut active_view: Signal<View>) -> Element {
    rsx! {
        div { class: "view-tabs",
            ViewTab { view: View::Allocation, label: "Allocation", active_view }
            ViewTab { view: View::Technical, label: "Technical", active_view }
            ViewTab { view: View::Roadmap, label: "Roadmap", active_view }
        }
    }
}

/// Individual view tab
#[component]
fn ViewTab(view: View, label: &'static str, mut active_view: Signal<View>) -> Element {
    let is_active = active_view() == view;
    rsx! {
        button {
            class: if is_active { "view-tab active" } else { "view-tab" },
            onclick: move |_| active_view.set(view),
            "{label}"
        }
    }
}

/// Capacity indicator bar
#[component]
fn CapacityIndicator(
    total_allocated: f32,
    total_capacity: f32,
    bar_class: &'static str,
    utilization_pct: f32,
) -> Element {
    rsx! {
        div { class: "capacity-indicator",
            span { class: "capacity-text", "{total_allocated:.1} / {total_capacity:.0} weeks" }
            div { class: "capacity-bar",
                div {
                    class: "{bar_class}",
                    style: "width: {utilization_pct:.0}%"
                }
            }
        }
    }
}

/// Chevron icon for dropdown
#[component]
fn ChevronIcon() -> Element {
    rsx! {
        svg {
            class: "plan-menu-chevron",
            width: "12",
            height: "12",
            view_box: "0 0 12 12",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            polyline { points: "3,4 6,7 9,4" }
        }
    }
}

// =============================================================================
// Helper functions
// =============================================================================

/// Calculate capacity metrics for the team
fn calculate_capacity_metrics(plan: &PlanState, prefs: &Preferences) -> (f32, f32, &'static str) {
    let total_capacity: f32 = prefs.team_members.iter().map(|m| m.capacity).sum();
    let total_allocated: f32 = plan
        .allocations
        .iter()
        .flat_map(|a| &a.assignments)
        .map(|a| a.percentage / 100.0)
        .sum();

    let utilization_ratio = if total_capacity > 0.0 {
        total_allocated / total_capacity
    } else {
        0.0
    };

    let bar_class = match () {
        _ if total_capacity == 0.0 || total_allocated == 0.0 => "capacity-bar-fill",
        _ if utilization_ratio < 0.85 => "capacity-bar-fill warning",
        _ if utilization_ratio <= 1.0 => "capacity-bar-fill success",
        _ if utilization_ratio <= 1.10 => "capacity-bar-fill warning",
        _ => "capacity-bar-fill danger",
    };

    (total_capacity, total_allocated, bar_class)
}

/// Calculate utilization percentage for display (capped at 100%)
fn calculate_utilization_display(allocated: f32, capacity: f32) -> f32 {
    if capacity > 0.0 {
        (allocated / capacity * 100.0).min(100.0)
    } else {
        0.0
    }
}

/// Restore preferences and plan state from localStorage
fn restore_from_local_storage(
    mut preferences: Signal<Preferences>,
    mut plan_state: Signal<PlanState>,
    mut viewing_session: Signal<Option<crate::state::ViewingSession>>,
) {
    info!("Closing viewed plan, restoring from local storage");
    let restored_prefs = storage::load_preferences().unwrap_or_default();
    let restored_state = storage::load_plan_state().unwrap_or_default();
    preferences.set(restored_prefs);
    plan_state.set(restored_state);
    viewing_session.set(None);
    // Clear ?plan= from URL so refresh doesn't reload the shared plan
    crate::plan_io::clear_url_plan_param();
}

/// Trigger file open dialog (web only - clicks hidden file input)
#[cfg(target_family = "wasm")]
fn trigger_file_open(file_input_id: String) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(input) = document.get_element_by_id(&file_input_id) {
                if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                    html_input.click();
                }
            }
        }
    }
}

/// Load plan from JSON string (shared logic)
fn load_plan_from_json(
    json: &str,
    filename: &str,
    prefs_signal: &mut Signal<Preferences>,
    state_signal: &mut Signal<PlanState>,
    viewing_signal: &mut Signal<Option<crate::state::ViewingSession>>,
) -> Result<(), String> {
    let export: PlanExport =
        serde_json::from_str(json).map_err(|e| format!("Failed to parse: {}", e))?;

    export.validate().map_err(|e| format!("{:?}", e))?;

    let original_json = serde_json::to_string(&export).unwrap_or_default();
    let (loaded_prefs, loaded_state) = export.into_signals();

    viewing_signal.set(Some(crate::state::ViewingSession {
        filename: filename.to_string(),
        original_json,
        modified: false,
    }));

    prefs_signal.set(loaded_prefs);
    state_signal.set(loaded_state);

    Ok(())
}

/// Handle file import from hidden input (web)
#[cfg(target_family = "wasm")]
fn handle_file_import(
    file_input_id: String,
    preferences: Signal<Preferences>,
    plan_state: Signal<PlanState>,
    viewing_session: Signal<Option<crate::state::ViewingSession>>,
) {
    let mut prefs_signal = preferences;
    let mut state_signal = plan_state;
    let mut viewing_signal = viewing_session;

    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(result) = crate::plan_io::read_file_from_input(&file_input_id).await {
            if let Err(e) = load_plan_from_json(
                &result.content,
                &result.filename,
                &mut prefs_signal,
                &mut state_signal,
                &mut viewing_signal,
            ) {
                web_sys::console::error_1(&format!("Failed to load plan: {}", e).into());
            }
        }
    });
}

/// Handle paste from clipboard (web)
#[cfg(target_family = "wasm")]
fn handle_paste_from_clipboard(
    preferences: Signal<Preferences>,
    plan_state: Signal<PlanState>,
    viewing_session: Signal<Option<crate::state::ViewingSession>>,
) {
    let mut prefs_signal = preferences;
    let mut state_signal = plan_state;
    let mut viewing_signal = viewing_session;

    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(content) = crate::plan_io::read_from_clipboard().await {
            // Try to decode as base64, fall back to raw JSON
            let json = if content.starts_with('{') {
                content
            } else {
                match crate::plan_io::base64_decode(&content) {
                    Ok(decoded) => decoded,
                    Err(_) => {
                        web_sys::console::error_1(
                            &"Clipboard does not contain a valid plan".into(),
                        );
                        return;
                    }
                }
            };

            if let Err(e) = load_plan_from_json(
                &json,
                "Pasted Plan",
                &mut prefs_signal,
                &mut state_signal,
                &mut viewing_signal,
            ) {
                web_sys::console::error_1(&format!("Failed to load plan: {}", e).into());
            }
        }
    });
}

/// Register global keyboard shortcuts (web only)
#[cfg(target_family = "wasm")]
fn register_keyboard_shortcuts(
    file_input_id: Signal<String>,
    preferences: Signal<Preferences>,
    plan_state: Signal<PlanState>,
) {
    use wasm_bindgen::prelude::*;

    use_effect(move || {
        let closure = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            let meta = e.meta_key() || e.ctrl_key();
            let key = e.key();

            if meta && (key == "s" || key == "S") {
                e.prevent_default();
                let export = PlanExport::from_signals(preferences(), plan_state());
                let _ = trigger_plan_download(&export);
            } else if meta && (key == "o" || key == "O") {
                e.prevent_default();
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(input) = document.get_element_by_id(&file_input_id()) {
                            if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                                html_input.click();
                            }
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let _ = document
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
        }

        closure.forget();
    });
}

/// Handle paste from clipboard (desktop)
#[cfg(not(target_family = "wasm"))]
fn handle_paste_from_clipboard_desktop(
    mut preferences: Signal<Preferences>,
    mut plan_state: Signal<PlanState>,
    mut viewing_session: Signal<Option<crate::state::ViewingSession>>,
) {
    use dioxus::logger::tracing::{error, warn};

    let content = match crate::plan_io::read_from_clipboard_sync() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read clipboard: {}", e);
            return;
        }
    };

    // Try to decode as base64, fall back to raw JSON
    let json = if content.starts_with('{') {
        content
    } else {
        match crate::plan_io::base64_decode_desktop(&content) {
            Ok(decoded) => decoded,
            Err(_) => {
                warn!("Clipboard does not contain a valid plan");
                return;
            }
        }
    };

    if let Err(e) = load_plan_from_json(
        &json,
        "Pasted Plan",
        &mut preferences,
        &mut plan_state,
        &mut viewing_session,
    ) {
        error!("Failed to load plan: {}", e);
    }
}
