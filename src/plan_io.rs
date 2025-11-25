//! Plan import/export functionality
//!
//! This module handles platform-specific file operations for saving and loading plans.

use crate::models::PlanExport;

/// Trigger a file download with the plan export as JSON
#[cfg(target_family = "wasm")]
pub fn trigger_plan_download(export: &PlanExport) -> Result<(), String> {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        export function download_json(json, filename) {
            const blob = new Blob([json], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = filename;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        }
    "#)]
    extern "C" {
        fn download_json(json: &str, filename: &str);
    }

    let json = serde_json::to_string_pretty(export)
        .map_err(|e| format!("Failed to serialize plan: {}", e))?;

    let filename = generate_plan_filename(export);
    download_json(&json, &filename);

    Ok(())
}

/// Desktop/native implementation of plan download
#[cfg(not(target_family = "wasm"))]
pub fn trigger_plan_download(export: &PlanExport) -> Result<(), String> {
    let json = serde_json::to_string_pretty(export)
        .map_err(|e| format!("Failed to serialize plan: {}", e))?;

    let filename = generate_plan_filename(export);

    if let Some(download_dir) = dirs::download_dir() {
        let path = download_dir.join(&filename);
        std::fs::write(&path, &json).map_err(|e| format!("Failed to write file: {}", e))?;
        eprintln!("Saved plan to: {:?}", path);
    } else {
        return Err("Could not find Downloads folder".to_string());
    }

    Ok(())
}

/// Generate a filename for the plan export
fn generate_plan_filename(export: &PlanExport) -> String {
    let team_name = export.team_name.to_lowercase().replace(' ', "-");
    let quarter = export.quarter_name.to_lowercase().replace(' ', "-");
    format!("plan-{}-{}.json", team_name, quarter)
}

/// Result of reading a file from input
#[cfg(target_family = "wasm")]
pub struct FileReadResult {
    pub filename: String,
    pub content: String,
}

/// Read file content from a file input element using JavaScript
#[cfg(target_family = "wasm")]
pub async fn read_file_from_input(input_id: &str) -> Result<FileReadResult, String> {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let input = document
        .get_element_by_id(input_id)
        .ok_or("Input not found")?
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| "Not an input element")?;

    let files = input.files().ok_or("No files")?;
    let file = files.get(0).ok_or("No file selected")?;
    let filename = file.name();

    let file_reader = web_sys::FileReader::new().map_err(|_| "Failed to create FileReader")?;
    file_reader
        .read_as_text(&file)
        .map_err(|_| "Failed to start reading")?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let fr = file_reader.clone();
        let onload = Closure::once(Box::new(move || {
            if let Ok(result) = fr.result() {
                resolve.call1(&JsValue::NULL, &result).unwrap();
            }
        }) as Box<dyn FnOnce()>);

        let onerror = Closure::once(Box::new(move || {
            reject
                .call1(&JsValue::NULL, &JsValue::from_str("Read error"))
                .unwrap();
        }) as Box<dyn FnOnce()>);

        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onload.forget();
        onerror.forget();
    });

    let result = JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to read file")?;

    let content = result.as_string().ok_or("Result is not a string")?;

    // Clear the input so the same file can be selected again
    input.set_value("");

    Ok(FileReadResult { filename, content })
}

/// Copy plan to clipboard as base64-encoded JSON
#[cfg(target_family = "wasm")]
pub fn copy_plan_to_clipboard(export: &PlanExport) -> Result<(), String> {
    use wasm_bindgen::prelude::*;

    let json = serde_json::to_string(export).map_err(|e| format!("Failed to serialize: {}", e))?;
    let encoded = base64_encode(&json);

    #[wasm_bindgen(inline_js = r#"
        export function copy_to_clipboard(text) {
            navigator.clipboard.writeText(text).catch(err => {
                console.error('Failed to copy:', err);
            });
        }
    "#)]
    extern "C" {
        fn copy_to_clipboard(text: &str);
    }

    copy_to_clipboard(&encoded);
    Ok(())
}

/// Copy plan to clipboard (desktop stub)
#[cfg(not(target_family = "wasm"))]
pub fn copy_plan_to_clipboard(_export: &PlanExport) -> Result<(), String> {
    eprintln!("Clipboard not supported on desktop yet");
    Ok(())
}

/// Read text from clipboard
#[cfg(target_family = "wasm")]
pub async fn read_from_clipboard() -> Result<String, String> {
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("No window")?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();

    let promise = clipboard.read_text();
    let result = JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to read clipboard")?;

    result
        .as_string()
        .ok_or_else(|| "Clipboard is empty".to_string())
}

/// Base64 encode a string (for clipboard sharing)
#[cfg(target_family = "wasm")]
pub fn base64_encode(input: &str) -> String {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        export function btoa_safe(str) {
            return btoa(unescape(encodeURIComponent(str)));
        }
    "#)]
    extern "C" {
        fn btoa_safe(s: &str) -> String;
    }

    btoa_safe(input)
}

/// Base64 decode a string
#[cfg(target_family = "wasm")]
pub fn base64_decode(input: &str) -> Result<String, String> {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
        export function atob_safe(str) {
            try {
                return decodeURIComponent(escape(atob(str)));
            } catch (e) {
                return null;
            }
        }
    "#)]
    extern "C" {
        fn atob_safe(s: &str) -> Option<String>;
    }

    atob_safe(input).ok_or_else(|| "Invalid base64".to_string())
}
