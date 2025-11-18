use dioxus::prelude::*;

/// Button component variants
/// Reference: docs/component-reference.md section 3
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    #[allow(dead_code)] // Reserved for secondary actions in editing UI
    Secondary,
}

#[component]
pub fn Button(
    /// Button variant (primary or secondary)
    variant: ButtonVariant,
    /// Button text content
    children: Element,
    /// Optional click handler
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let class_name = match variant {
        ButtonVariant::Primary => "btn btn-primary",
        ButtonVariant::Secondary => "btn btn-secondary",
    };

    rsx! {
        button {
            class: "{class_name}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}
