/// Assign project modal for single-cell allocation
use dioxus::prelude::*;
use uuid::Uuid;

use planner_core::models::ProjectColor;

/// Assign project modal component
#[component]
pub fn AssignProjectModal(
    visible: bool,
    selected_project_id: Option<Uuid>,
    on_project_select: EventHandler<Uuid>,
    on_apply: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    if !visible {
        return rsx! {};
    }

    let plan = crate::state::use_plan_state();
    let plan_data = plan();

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| on_cancel.call(()),

            // Modal container
            div {
                class: "modal-container assign-project-modal",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "Assign Project" }
                    button {
                        class: "modal-close",
                        onclick: move |_| on_cancel.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    div { class: "assign-section",
                        label { class: "assign-label", "Select Project" }

                        // Project list
                        div { class: "assign-project-list",
                            for project in &plan_data.technical_projects {
                                {
                                    let project_id = project.id;
                                    let allocated = plan_data.calculate_project_allocated_weeks(&project_id);
                                    let color = project.roadmap_project_id
                                        .and_then(|id| plan_data.get_roadmap_project(&id))
                                        .map(|rp| rp.color)
                                        .unwrap_or(ProjectColor::Blue);
                                    let is_selected = selected_project_id == Some(project_id);

                                    rsx! {
                                        button {
                                            class: if is_selected { "assign-project-option selected" } else { "assign-project-option" },
                                            onclick: move |_| on_project_select.call(project_id),
                                            div {
                                                class: "project-color-dot",
                                                style: "background: {color.to_hex()};",
                                            }
                                            span { class: "project-name", "{project.name}" }
                                            span { class: "project-allocated", "{allocated:.1}w" }
                                        }
                                    }
                                }
                            }

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
                        disabled: selected_project_id.is_none(),
                        onclick: move |_| on_apply.call(()),
                        "Assign"
                    }
                }
            }
        }
    }
}
