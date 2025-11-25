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

use crate::models::{PlanState, Preferences};

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

    Ok(())
}

/// Load preferences from localStorage
#[cfg(target_family = "wasm")]
pub fn load_preferences() -> Option<Preferences> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item(PREFERENCES_KEY).ok()??;

    serde_json::from_str(&json).ok()
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

    Ok(())
}

/// Load plan state from localStorage
#[cfg(target_family = "wasm")]
pub fn load_plan_state() -> Option<PlanState> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item(PLAN_STATE_KEY).ok()??;

    serde_json::from_str(&json).ok()
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

    std::fs::write(&path, json).map_err(|e| format!("Failed to write preferences file: {}", e))?;

    Ok(())
}

/// Load preferences from config file
#[cfg(not(target_family = "wasm"))]
pub fn load_preferences() -> Option<Preferences> {
    let path = get_preferences_path()?;

    if !path.exists() {
        return None;
    }

    let json = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Clear preferences by removing config file
#[cfg(not(target_family = "wasm"))]
pub fn clear_preferences() -> Result<(), String> {
    let path = get_preferences_path().ok_or("Could not determine config directory")?;

    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove preferences file: {}", e))?;
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

    std::fs::write(&path, json).map_err(|e| format!("Failed to write plan state file: {}", e))?;

    Ok(())
}

/// Load plan state from config file
#[cfg(not(target_family = "wasm"))]
pub fn load_plan_state() -> Option<PlanState> {
    let path = get_plan_state_path()?;

    if !path.exists() {
        return None;
    }

    let json = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Clear plan state by removing config file
#[cfg(not(target_family = "wasm"))]
pub fn clear_plan_state() -> Result<(), String> {
    let path = get_plan_state_path().ok_or("Could not determine config directory")?;

    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove plan state file: {}", e))?;
    }

    Ok(())
}
