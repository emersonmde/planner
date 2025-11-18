use dioxus::prelude::*;

/// Input component for search and text entry
/// Reference: docs/component-reference.md section 10
#[component]
pub fn Input(
    /// Placeholder text
    #[props(default = "".to_string())]
    placeholder: String,
    /// Input value
    value: Signal<String>,
) -> Element {
    rsx! {
        input {
            class: "search-input",
            r#type: "text",
            placeholder: "{placeholder}",
            value: "{value}",
            oninput: move |evt| value.set(evt.value()),
        }
    }
}
