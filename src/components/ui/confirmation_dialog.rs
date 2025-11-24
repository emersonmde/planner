use dioxus::prelude::*;

use super::{Button, ButtonVariant};

/// Props for ConfirmationDialog component
#[derive(Props, Clone, PartialEq)]
pub struct ConfirmationDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Dialog title
    pub title: String,
    /// Main message
    pub message: String,
    /// Optional warning message
    #[props(default = String::new())]
    pub warning: String,
    /// Confirm button label
    #[props(default = "Delete".to_string())]
    pub confirm_label: String,
    /// Cancel button label
    #[props(default = "Cancel".to_string())]
    pub cancel_label: String,
    /// Event handlers
    pub on_confirm: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

/// Confirmation dialog for destructive actions
///
/// Displays a modal dialog with confirm/cancel buttons
#[component]
pub fn ConfirmationDialog(props: ConfirmationDialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| props.on_cancel.call(()),

            // Dialog container
            div {
                class: "modal-container confirmation-dialog",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "{props.title}" }
                    button {
                        class: "modal-close",
                        onclick: move |_| props.on_cancel.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    p { class: "dialog-message", "{props.message}" }
                    if !props.warning.is_empty() {
                        p { class: "dialog-warning", "{props.warning}" }
                    }
                }

                // Footer
                div { class: "modal-footer",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| props.on_cancel.call(()),
                        "{props.cancel_label}"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| props.on_confirm.call(()),
                        "{props.confirm_label}"
                    }
                }
            }
        }
    }
}
