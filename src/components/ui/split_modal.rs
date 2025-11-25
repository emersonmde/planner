/// Split allocation modal for dividing a week between two projects
use dioxus::prelude::*;
use uuid::Uuid;

/// Split allocation modal component
#[component]
pub fn SplitAllocationModal(
    visible: bool,
    project1_id: Option<Uuid>,
    project1_percentage: f32,
    project2_id: Option<Uuid>,
    on_project1_select: EventHandler<Uuid>,
    on_project2_select: EventHandler<Uuid>,
    on_percentage_change: EventHandler<f32>,
    on_apply: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    if !visible {
        return rsx! {};
    }

    let project2_percentage = 100.0 - project1_percentage;

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| on_cancel.call(()),

            // Modal container
            div {
                class: "modal-container split-allocation-modal",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "Split Allocation" }
                    button {
                        class: "modal-close",
                        onclick: move |_| on_cancel.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    // Project 1 selector
                    div { class: "split-section",
                        label { class: "split-label", "Project A" }
                        ProjectDropdown {
                            selected_id: project1_id,
                            on_select: move |id| on_project1_select.call(id),
                        }
                        div { class: "percentage-display",
                            "{project1_percentage:.0}%"
                        }
                    }

                    // Percentage slider
                    div { class: "split-slider-section",
                        input {
                            r#type: "range",
                            class: "split-slider",
                            min: "0",
                            max: "100",
                            step: "5",
                            value: "{project1_percentage}",
                            oninput: move |e| {
                                if let Ok(val) = e.value().parse::<f32>() {
                                    on_percentage_change.call(val);
                                }
                            },
                        }
                    }

                    // Visual preview bar (vertical split)
                    div { class: "split-preview",
                        div {
                            class: "split-preview-project1",
                            style: "height: {project1_percentage}%;",
                        }
                        div {
                            class: "split-preview-project2",
                            style: "height: {project2_percentage}%;",
                        }
                    }

                    // Project 2 selector
                    div { class: "split-section",
                        label { class: "split-label", "Project B" }
                        ProjectDropdown {
                            selected_id: project2_id,
                            on_select: move |id| on_project2_select.call(id),
                        }
                        div { class: "percentage-display",
                            "{project2_percentage:.0}%"
                        }
                    }

                    // Validation message
                    if project1_id.is_none() || project2_id.is_none() {
                        div { class: "validation-message",
                            "⚠ Please select both projects"
                        }
                    } else if project1_id == project2_id {
                        div { class: "validation-message error",
                            "⚠ Projects must be different"
                        }
                    }
                }

                // Footer
                div { class: "modal-footer",
                    button {
                        class: "modal-button secondary",
                        onclick: move |_| on_cancel.call(()),
                        "Cancel"
                    }
                    button {
                        class: "modal-button primary",
                        disabled: project1_id.is_none() || project2_id.is_none() || project1_id == project2_id,
                        onclick: move |_| on_apply.call(()),
                        "Apply Split"
                    }
                }
            }
        }
    }
}

/// Project dropdown for split modal
#[component]
fn ProjectDropdown(selected_id: Option<Uuid>, on_select: EventHandler<Uuid>) -> Element {
    let plan = crate::state::use_plan_state();
    let plan_data = plan();

    rsx! {
        select {
            class: "project-dropdown",
            value: selected_id.map(|id| id.to_string()).unwrap_or_default(),
            onchange: move |e| {
                if let Ok(id) = Uuid::parse_str(&e.value()) {
                    on_select.call(id);
                }
            },

            option { value: "", "Select project..." }

            for project in &plan_data.technical_projects {
                {
                    let project_id = project.id;
                    let allocated = plan_data.calculate_project_allocated_weeks(&project_id);

                    rsx! {
                        option {
                            value: "{project_id}",
                            selected: selected_id == Some(project_id),
                            "{project.name} ({allocated:.1}w)"
                        }
                    }
                }
            }
        }
    }
}
