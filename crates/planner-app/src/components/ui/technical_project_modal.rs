use chrono::NaiveDate;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::ui::{Button, ButtonVariant, Input};
use planner_core::models::{ProjectColor, RoadmapProject, TechnicalProject};

/// Mode for the technical project modal (Add or Edit)
#[derive(Clone, Copy, PartialEq)]
pub enum TechnicalModalMode {
    Add,
    Edit(Uuid), // Contains project ID being edited
}

/// Props for TechnicalProjectModal
#[derive(Props, Clone, PartialEq)]
pub struct TechnicalProjectModalProps {
    /// Mode (Add or Edit with project ID)
    pub mode: TechnicalModalMode,
    /// Initial values for the form
    pub initial_name: String,
    pub initial_roadmap_project_id: Option<Uuid>,
    pub initial_eng_estimate: f32,
    pub initial_sci_estimate: f32,
    pub initial_start_date: NaiveDate,
    pub initial_expected_completion: Option<NaiveDate>,
    pub initial_notes: String,
    /// Available roadmap projects for dropdown
    pub roadmap_projects: Vec<RoadmapProject>,
    /// Event handlers
    pub on_save: EventHandler<TechnicalProject>,
    pub on_cancel: EventHandler<()>,
}

/// Technical project modal for adding/editing technical projects
#[component]
pub fn TechnicalProjectModal(props: TechnicalProjectModalProps) -> Element {
    // Form state - signals initialized from props
    let name = use_signal(|| props.initial_name.clone());
    let mut roadmap_project_id = use_signal(|| props.initial_roadmap_project_id);
    let mut eng_estimate = use_signal(|| props.initial_eng_estimate);
    let mut sci_estimate = use_signal(|| props.initial_sci_estimate);
    let mut start_date = use_signal(|| props.initial_start_date);
    let mut expected_completion = use_signal(|| props.initial_expected_completion);
    let mut notes = use_signal(|| props.initial_notes.clone());

    // Validation errors
    let mut name_error = use_signal(String::new);
    let mut estimate_error = use_signal(String::new);

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

        // Validate at least one estimate is > 0
        if eng_estimate() <= 0.0 && sci_estimate() <= 0.0 {
            estimate_error.set("At least one estimate must be greater than 0".to_string());
            is_valid = false;
        } else {
            estimate_error.set(String::new());
        }

        is_valid
    };

    // Handle save
    let handle_save = move |_| {
        if !validate_form() {
            return;
        }

        let project = match props.mode {
            TechnicalModalMode::Add => TechnicalProject {
                id: Uuid::new_v4(),
                name: name().trim().to_string(),
                roadmap_project_id: roadmap_project_id(),
                eng_estimate: eng_estimate(),
                sci_estimate: sci_estimate(),
                start_date: start_date(),
                expected_completion: expected_completion(),
                notes: if notes().trim().is_empty() {
                    None
                } else {
                    Some(notes().trim().to_string())
                },
            },
            TechnicalModalMode::Edit(id) => TechnicalProject {
                id,
                name: name().trim().to_string(),
                roadmap_project_id: roadmap_project_id(),
                eng_estimate: eng_estimate(),
                sci_estimate: sci_estimate(),
                start_date: start_date(),
                expected_completion: expected_completion(),
                notes: if notes().trim().is_empty() {
                    None
                } else {
                    Some(notes().trim().to_string())
                },
            },
        };

        props.on_save.call(project);
    };

    let modal_title = match props.mode {
        TechnicalModalMode::Add => "Add Technical Project",
        TechnicalModalMode::Edit(_) => "Edit Technical Project",
    };

    // Get the color of the selected roadmap project (for display)
    let selected_color = roadmap_project_id()
        .and_then(|id| props.roadmap_projects.iter().find(|rp| rp.id == id))
        .map(|rp| rp.color)
        .unwrap_or(ProjectColor::Blue);

    // Calculate total estimate for display
    let total_estimate = eng_estimate() + sci_estimate();

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| props.on_cancel.call(()),

            // Modal container
            div {
                class: "modal-container technical-project-modal",
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
                            placeholder: "e.g., Payment API Integration".to_string(),
                        }
                        div { class: "form-error", "{name_error()}" }
                    }

                    // Roadmap project dropdown
                    div { class: "form-field",
                        label { class: "form-label", "Roadmap Project" }
                        div { class: "dropdown-with-color",
                            // Color indicator
                            span {
                                class: "color-dot",
                                style: "background-color: {selected_color.to_hex()};"
                            }
                            select {
                                class: "select",
                                value: roadmap_project_id().map(|id| id.to_string()).unwrap_or_default(),
                                onchange: move |e| {
                                    let value = e.value();
                                    if value.is_empty() {
                                        roadmap_project_id.set(None);
                                    } else if let Ok(id) = Uuid::parse_str(&value) {
                                        roadmap_project_id.set(Some(id));
                                    }
                                },
                                option { value: "", "None" }
                                for rp in &props.roadmap_projects {
                                    option {
                                        value: "{rp.id}",
                                        "{rp.name}"
                                    }
                                }
                            }
                        }
                        div { class: "form-hint", "Color is inherited from the linked roadmap project" }
                    }

                    // Estimates row (Eng, Sci, Total)
                    div { class: "form-row form-row-3",
                        div { class: "form-field",
                            label { class: "form-label", "Eng Estimate *" }
                            input {
                                r#type: "number",
                                class: "input",
                                step: "0.5",
                                min: "0",
                                placeholder: "0.0",
                                value: "{eng_estimate()}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f32>() {
                                        eng_estimate.set(v.max(0.0));
                                    }
                                },
                            }
                        }

                        div { class: "form-field",
                            label { class: "form-label", "Sci Estimate *" }
                            input {
                                r#type: "number",
                                class: "input",
                                step: "0.5",
                                min: "0",
                                placeholder: "0.0",
                                value: "{sci_estimate()}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<f32>() {
                                        sci_estimate.set(v.max(0.0));
                                    }
                                },
                            }
                        }

                        div { class: "form-field",
                            label { class: "form-label", "Total" }
                            input {
                                r#type: "number",
                                class: "input",
                                disabled: true,
                                value: "{total_estimate:.1}",
                            }
                        }
                    }
                    div { class: "form-error", "{estimate_error()}" }

                    // Dates row
                    div { class: "form-row",
                        div { class: "form-field",
                            label { class: "form-label", "Start Date" }
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
                            div { class: "form-hint", "Auto-updates from allocations" }
                        }

                        div { class: "form-field",
                            label { class: "form-label", "Expected Completion" }
                            input {
                                r#type: "date",
                                class: "input",
                                value: expected_completion().map(|d| d.to_string()).unwrap_or_default(),
                                oninput: move |e| {
                                    let value = e.value();
                                    if value.is_empty() {
                                        expected_completion.set(None);
                                    } else if let Ok(d) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                                        expected_completion.set(Some(d));
                                    }
                                },
                            }
                            div { class: "form-hint", "Auto-updates from allocations" }
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
                        if matches!(props.mode, TechnicalModalMode::Add) {
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
