use chrono::NaiveDate;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::ui::{Button, ButtonVariant, ColorPicker, Input};
use crate::models::{ProjectColor, RoadmapProject};

/// Mode for the roadmap project modal (Add or Edit)
#[derive(Clone, Copy, PartialEq)]
pub enum ModalMode {
    Add,
    Edit(Uuid), // Contains project ID being edited
}

/// Props for RoadmapProjectModal
#[derive(Props, Clone, PartialEq)]
pub struct RoadmapProjectModalProps {
    /// Mode (Add or Edit with project ID)
    pub mode: ModalMode,
    /// Initial values for the form
    pub initial_name: String,
    pub initial_eng_estimate: f32,
    pub initial_sci_estimate: f32,
    pub initial_start_date: NaiveDate,
    pub initial_launch_date: NaiveDate,
    pub initial_color: ProjectColor,
    pub initial_notes: String,
    /// Event handlers
    pub on_save: EventHandler<RoadmapProject>,
    pub on_cancel: EventHandler<()>,
}

/// Roadmap project modal for adding/editing roadmap projects
#[component]
pub fn RoadmapProjectModal(props: RoadmapProjectModalProps) -> Element {
    // Form state - simple signals initialized from props
    let name = use_signal(|| props.initial_name.clone());
    let mut eng_estimate = use_signal(|| props.initial_eng_estimate);
    let mut sci_estimate = use_signal(|| props.initial_sci_estimate);
    let mut start_date = use_signal(|| props.initial_start_date);
    let mut launch_date = use_signal(|| props.initial_launch_date);
    let mut color = use_signal(|| props.initial_color);
    let mut notes = use_signal(|| props.initial_notes.clone());

    // Validation errors
    let mut name_error = use_signal(String::new);
    let mut eng_estimate_error = use_signal(String::new);
    let mut sci_estimate_error = use_signal(String::new);
    let mut date_error = use_signal(String::new);

    // Validation function
    let mut validate_form = move || -> bool {
        let mut is_valid = true;

        // Validate name
        if name().trim().is_empty() {
            name_error.set("Project name is required".to_string());
            is_valid = false;
        } else {
            name_error.set(String::new());
        }

        // Validate eng estimate (allow 0 for placeholder projects)
        if eng_estimate() < 0.0 {
            eng_estimate_error.set("Engineering estimate cannot be negative".to_string());
            is_valid = false;
        } else {
            eng_estimate_error.set(String::new());
        }

        // Validate sci estimate
        if sci_estimate() < 0.0 {
            sci_estimate_error.set("Science estimate cannot be negative".to_string());
            is_valid = false;
        } else {
            sci_estimate_error.set(String::new());
        }

        // Validate dates
        if start_date() >= launch_date() {
            date_error.set("Launch date must be after start date".to_string());
            is_valid = false;
        } else {
            date_error.set(String::new());
        }

        is_valid
    };

    // Handle save
    let handle_save = move |_| {
        if !validate_form() {
            return;
        }

        let project = match props.mode {
            ModalMode::Add => RoadmapProject {
                id: Uuid::new_v4(),
                name: name().trim().to_string(),
                eng_estimate: eng_estimate(),
                sci_estimate: sci_estimate(),
                start_date: start_date(),
                launch_date: launch_date(),
                color: color(),
                notes: if notes().trim().is_empty() {
                    None
                } else {
                    Some(notes().trim().to_string())
                },
            },
            ModalMode::Edit(id) => RoadmapProject {
                id,
                name: name().trim().to_string(),
                eng_estimate: eng_estimate(),
                sci_estimate: sci_estimate(),
                start_date: start_date(),
                launch_date: launch_date(),
                color: color(),
                notes: if notes().trim().is_empty() {
                    None
                } else {
                    Some(notes().trim().to_string())
                },
            },
        };

        props.on_save.call(project);
        // No need to reset form - component will unmount when parent closes modal
    };

    let modal_title = match props.mode {
        ModalMode::Add => "Add Roadmap Project",
        ModalMode::Edit(_) => "Edit Roadmap Project",
    };

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| props.on_cancel.call(()),

            // Modal container
            div {
                class: "modal-container roadmap-project-modal",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "{modal_title}" }
                    button {
                        class: "modal-close",
                        onclick: move |_| props.on_cancel.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    // Project name
                    div { class: "form-field",
                        label { class: "form-label", "Project Name *" }
                        Input {
                            value: name,
                            placeholder: "e.g., Q1 Platform Improvements".to_string(),
                        }
                        div { class: "form-error", "{name_error()}" }
                    }

                    // Estimates row
                    div { class: "form-row",
                        div { class: "form-field",
                            label { class: "form-label", "Engineering Estimate (weeks) *" }
                            input {
                                r#type: "number",
                                class: "input",
                                step: "0.5",
                                min: "0",
                                placeholder: "0.0",
                                value: "{eng_estimate()}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f32>() {
                                        eng_estimate.set(v);
                                    }
                                },
                            }
                            div { class: "form-error", "{eng_estimate_error()}" }
                        }

                        div { class: "form-field",
                            label { class: "form-label", "Science Estimate (weeks) *" }
                            input {
                                r#type: "number",
                                class: "input",
                                step: "0.5",
                                min: "0",
                                placeholder: "0.0",
                                value: "{sci_estimate()}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f32>() {
                                        sci_estimate.set(v);
                                    }
                                },
                            }
                            div { class: "form-error", "{sci_estimate_error()}" }
                        }
                    }

                    // Dates row
                    div { class: "form-row",
                        div { class: "form-field",
                            label { class: "form-label", "Start Date *" }
                            input {
                                r#type: "date",
                                class: "input",
                                value: "{start_date()}",
                                oninput: move |e| {
                                    if let Ok(d) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                        start_date.set(d);
                                    }
                                },
                            }
                        }

                        div { class: "form-field",
                            label { class: "form-label", "Launch Date *" }
                            input {
                                r#type: "date",
                                class: "input",
                                value: "{launch_date()}",
                                oninput: move |e| {
                                    if let Ok(d) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                        launch_date.set(d);
                                    }
                                },
                            }
                        }
                    }
                    div { class: "form-error", "{date_error()}" }

                    // Color picker
                    div { class: "form-field",
                        label { class: "form-label", "Project Color" }
                        ColorPicker {
                            selected: color(),
                            onchange: move |c| color.set(c),
                        }
                    }

                    // Notes
                    div { class: "form-field",
                        label { class: "form-label", "Notes (optional)" }
                        textarea {
                            class: "textarea",
                            rows: "3",
                            placeholder: "Additional notes or context...",
                            value: "{notes()}",
                            oninput: move |e| notes.set(e.value()),
                        }
                    }
                }

                // Footer
                div { class: "modal-footer",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: handle_save,
                        if matches!(props.mode, ModalMode::Add) {
                            "Add Project"
                        } else {
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}
