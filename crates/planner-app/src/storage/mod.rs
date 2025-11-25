//! Storage abstraction for persisting application state across platforms.
//!
//! - **Web (WASM)**: Uses browser localStorage via web-sys
//! - **Desktop**: Uses file-based storage in OS config directory via dirs crate
//!
//! Both implementations provide the same API for feature parity.
//!
//! Note: Features are mutually exclusive at runtime based on target platform.
//! - WASM targets use localStorage
//! - Native targets use file-based storage

#[cfg(not(target_family = "wasm"))]
use dioxus::logger::tracing::{debug, error, info, warn};
#[cfg(target_family = "wasm")]
use dioxus::logger::tracing::{debug, info, warn};
use planner_core::models::{PlanState, Preferences};

// ============================================================================
// Web Implementation (localStorage) - only for WASM targets
// ============================================================================

#[cfg(target_family = "wasm")]
const PREFERENCES_KEY: &str = "planner_preferences";
#[cfg(target_family = "wasm")]
const PLAN_STATE_KEY: &str = "planner_plan_state";

/// Save preferences to localStorage
#[cfg(target_family = "wasm")]
pub fn save_preferences(prefs: &Preferences) -> Result<(), String> {
    let json = serde_json::to_string(prefs)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;

    let window = web_sys::window().ok_or("No window object")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;

    storage
        .set_item(PREFERENCES_KEY, &json)
        .map_err(|e| format!("Failed to save to localStorage: {:?}", e))?;

    debug!(
        "Saved preferences to localStorage ({} bytes, {} team members)",
        json.len(),
        prefs.team_members.len()
    );
    Ok(())
}

/// Load preferences from localStorage
#[cfg(target_family = "wasm")]
pub fn load_preferences() -> Option<Preferences> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item(PREFERENCES_KEY).ok()??;

    match serde_json::from_str::<Preferences>(&json) {
        Ok(prefs) => {
            info!(
                "Loaded preferences from localStorage ({} team members)",
                prefs.team_members.len()
            );
            Some(prefs)
        }
        Err(e) => {
            warn!("Failed to parse preferences from localStorage: {}", e);
            None
        }
    }
}

/// Clear preferences from localStorage
#[cfg(target_family = "wasm")]
pub fn clear_preferences() -> Result<(), String> {
    let window = web_sys::window().ok_or("No window object")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;

    storage
        .remove_item(PREFERENCES_KEY)
        .map_err(|e| format!("Failed to clear localStorage: {:?}", e))?;

    info!("Cleared preferences from localStorage");
    Ok(())
}

/// Save plan state to localStorage
#[cfg(target_family = "wasm")]
pub fn save_plan_state(state: &PlanState) -> Result<(), String> {
    let json = serde_json::to_string(state)
        .map_err(|e| format!("Failed to serialize plan state: {}", e))?;

    let window = web_sys::window().ok_or("No window object")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;

    storage
        .set_item(PLAN_STATE_KEY, &json)
        .map_err(|e| format!("Failed to save to localStorage: {:?}", e))?;

    debug!(
        "Saved plan state to localStorage ({} bytes, {} allocations)",
        json.len(),
        state.allocations.len()
    );
    Ok(())
}

/// Load plan state from localStorage
#[cfg(target_family = "wasm")]
pub fn load_plan_state() -> Option<PlanState> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item(PLAN_STATE_KEY).ok()??;

    match serde_json::from_str::<PlanState>(&json) {
        Ok(state) => {
            info!(
                "Loaded plan state from localStorage ({}, {} allocations)",
                state.quarter_name,
                state.allocations.len()
            );
            Some(state)
        }
        Err(e) => {
            warn!("Failed to parse plan state from localStorage: {}", e);
            None
        }
    }
}

/// Clear plan state from localStorage
#[cfg(target_family = "wasm")]
pub fn clear_plan_state() -> Result<(), String> {
    let window = web_sys::window().ok_or("No window object")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;

    storage
        .remove_item(PLAN_STATE_KEY)
        .map_err(|e| format!("Failed to clear localStorage: {:?}", e))?;

    info!("Cleared plan state from localStorage");
    Ok(())
}

// ============================================================================
// Native Implementation (file-based) - for desktop, mobile, and other native targets
// ============================================================================

/// Get the app config directory for native platforms
#[cfg(not(target_family = "wasm"))]
fn get_app_config_dir() -> Option<std::path::PathBuf> {
    let config_dir = dirs::config_dir()?;
    Some(config_dir.join("quarterly-planner"))
}

/// Get the preferences file path for native platforms
#[cfg(not(target_family = "wasm"))]
fn get_preferences_path() -> Option<std::path::PathBuf> {
    Some(get_app_config_dir()?.join("preferences.json"))
}

/// Get the plan state file path for native platforms
#[cfg(not(target_family = "wasm"))]
fn get_plan_state_path() -> Option<std::path::PathBuf> {
    Some(get_app_config_dir()?.join("plan_state.json"))
}

/// Save preferences to config file
#[cfg(not(target_family = "wasm"))]
pub fn save_preferences(prefs: &Preferences) -> Result<(), String> {
    let path = get_preferences_path().ok_or("Could not determine config directory")?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let json = serde_json::to_string_pretty(prefs)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;

    std::fs::write(&path, &json).map_err(|e| format!("Failed to write preferences file: {}", e))?;

    debug!(
        "Saved preferences to {:?} ({} bytes, {} team members)",
        path,
        json.len(),
        prefs.team_members.len()
    );
    Ok(())
}

/// Load preferences from config file
#[cfg(not(target_family = "wasm"))]
pub fn load_preferences() -> Option<Preferences> {
    let path = get_preferences_path()?;

    if !path.exists() {
        debug!("No preferences file found at {:?}", path);
        return None;
    }

    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to read preferences file {:?}: {}", path, e);
            return None;
        }
    };

    match serde_json::from_str::<Preferences>(&json) {
        Ok(prefs) => {
            info!(
                "Loaded preferences from {:?} ({} team members)",
                path,
                prefs.team_members.len()
            );
            Some(prefs)
        }
        Err(e) => {
            warn!("Failed to parse preferences from {:?}: {}", path, e);
            None
        }
    }
}

/// Clear preferences by removing config file
#[cfg(not(target_family = "wasm"))]
pub fn clear_preferences() -> Result<(), String> {
    let path = get_preferences_path().ok_or("Could not determine config directory")?;

    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove preferences file: {}", e))?;
        info!("Cleared preferences file at {:?}", path);
    } else {
        debug!("No preferences file to clear at {:?}", path);
    }

    Ok(())
}

/// Save plan state to config file
#[cfg(not(target_family = "wasm"))]
pub fn save_plan_state(state: &PlanState) -> Result<(), String> {
    let path = get_plan_state_path().ok_or("Could not determine config directory")?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let json = serde_json::to_string_pretty(state)
        .map_err(|e| format!("Failed to serialize plan state: {}", e))?;

    std::fs::write(&path, &json).map_err(|e| format!("Failed to write plan state file: {}", e))?;

    debug!(
        "Saved plan state to {:?} ({} bytes, {} allocations)",
        path,
        json.len(),
        state.allocations.len()
    );
    Ok(())
}

/// Load plan state from config file
#[cfg(not(target_family = "wasm"))]
pub fn load_plan_state() -> Option<PlanState> {
    let path = get_plan_state_path()?;

    if !path.exists() {
        debug!("No plan state file found at {:?}", path);
        return None;
    }

    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to read plan state file {:?}: {}", path, e);
            return None;
        }
    };

    match serde_json::from_str::<PlanState>(&json) {
        Ok(state) => {
            info!(
                "Loaded plan state from {:?} ({}, {} allocations)",
                path,
                state.quarter_name,
                state.allocations.len()
            );
            Some(state)
        }
        Err(e) => {
            warn!("Failed to parse plan state from {:?}: {}", path, e);
            None
        }
    }
}

/// Clear plan state by removing config file
#[cfg(not(target_family = "wasm"))]
pub fn clear_plan_state() -> Result<(), String> {
    let path = get_plan_state_path().ok_or("Could not determine config directory")?;

    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove plan state file: {}", e))?;
        info!("Cleared plan state file at {:?}", path);
    } else {
        debug!("No plan state file to clear at {:?}", path);
    }

    Ok(())
}
