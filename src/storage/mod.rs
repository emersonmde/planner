/// Storage abstraction for persisting preferences
///
/// For v1.0, we use web-sys localStorage for web builds.
/// Desktop persistence can be added later if needed.
use crate::models::Preferences;

/// Save preferences to localStorage (web + wasm only)
#[cfg(all(feature = "web", target_family = "wasm"))]
pub fn save_preferences(prefs: &Preferences) -> Result<(), String> {
    let json = serde_json::to_string(prefs)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;

    let window = web_sys::window().ok_or("No window object")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;

    storage
        .set_item("planner_preferences", &json)
        .map_err(|e| format!("Failed to save to localStorage: {:?}", e))?;

    Ok(())
}

/// Save preferences (web feature but not wasm - stub)
#[cfg(all(feature = "web", not(target_family = "wasm")))]
pub fn save_preferences(_prefs: &Preferences) -> Result<(), String> {
    Ok(())
}

/// Load preferences from localStorage (web only)
#[cfg(all(feature = "web", target_family = "wasm"))]
pub fn load_preferences() -> Option<Preferences> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item("planner_preferences").ok()??;

    serde_json::from_str(&json).ok()
}

/// Load preferences (web feature but not wasm - stub)
#[cfg(all(feature = "web", not(target_family = "wasm")))]
pub fn load_preferences() -> Option<Preferences> {
    None
}

/// Save preferences (desktop stub - no persistence yet)
#[cfg(not(feature = "web"))]
pub fn save_preferences(_prefs: &Preferences) -> Result<(), String> {
    // Desktop persistence not yet implemented
    Ok(())
}

/// Load preferences (desktop stub - returns None)
#[cfg(not(feature = "web"))]
pub fn load_preferences() -> Option<Preferences> {
    None
}
