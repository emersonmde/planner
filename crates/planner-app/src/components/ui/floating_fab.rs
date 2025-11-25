/// Floating Action Button for paintbrush mode
use dioxus::prelude::*;

/// Floating Action Button component
#[component]
pub fn FloatingFab(
    active: bool,
    project_color: Option<String>,
    on_click: EventHandler<()>,
) -> Element {
    let fab_class = if active {
        "floating-paintbrush-fab active"
    } else {
        "floating-paintbrush-fab"
    };

    let fab_style = if let Some(color) = project_color {
        format!(
            "background: color-mix(in srgb, var(--bg-overlay) 70%, {}); border-color: {};",
            color, color
        )
    } else {
        String::new()
    };

    rsx! {
        button {
            class: "{fab_class}",
            style: "{fab_style}",
            onclick: move |_| on_click.call(()),
        }
    }
}
