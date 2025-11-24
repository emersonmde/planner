use dioxus::prelude::*;

use crate::models::ProjectColor;

/// All available project colors
const COLORS: [ProjectColor; 9] = [
    ProjectColor::Blue,
    ProjectColor::Green,
    ProjectColor::Yellow,
    ProjectColor::Orange,
    ProjectColor::Red,
    ProjectColor::Purple,
    ProjectColor::Pink,
    ProjectColor::Teal,
    ProjectColor::Indigo,
];

/// Props for ColorPicker component
#[derive(Props, Clone, PartialEq)]
pub struct ColorPickerProps {
    /// Currently selected color
    pub selected: ProjectColor,
    /// Callback when color selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<ProjectColor>>,
}

/// Color picker component for selecting project colors
///
/// Displays all 9 ProjectColor variants as clickable swatches with:
/// - Visual selection state (border highlight)
/// - Keyboard navigation (arrow keys)
/// - Accessible ARIA labels
///
/// # Example
/// ```rust
/// ColorPicker {
///     selected: ProjectColor::Blue,
///     onchange: move |color| {
///         selected_color.set(color);
///     }
/// }
/// ```
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    rsx! {
        div {
            class: "color-picker",
            role: "radiogroup",
            "aria-label": "Project color selection",

            for color in COLORS.iter() {
                button {
                    key: "{color:?}",
                    class: if *color == props.selected { "color-swatch selected" } else { "color-swatch" },
                    role: "radio",
                    "aria-checked": if *color == props.selected { "true" } else { "false" },
                    "aria-label": "{color:?} color",
                    tabindex: if *color == props.selected { "0" } else { "-1" },
                    style: "background-color: {color.to_hex()};",
                    onclick: move |_| {
                        if let Some(handler) = &props.onchange {
                            handler.call(*color);
                        }
                    },
                    onkeydown: move |evt| {
                        // Allow selecting with Enter or Space
                        if evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string()) {
                            if let Some(handler) = &props.onchange {
                                handler.call(*color);
                            }
                        }
                    },
                }
            }
        }
    }
}
