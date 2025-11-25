use dioxus::prelude::*;

/// Button component variants
/// Reference: docs/component-reference.md section 3
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    #[allow(dead_code)] // Reserved for secondary actions in editing UI
    Secondary,
    #[allow(dead_code)] // Reserved for destructive actions
    Danger,
}

#[component]
pub fn Button(
    /// Button variant (primary, secondary, or danger)
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
        ButtonVariant::Danger => "btn btn-danger",
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
