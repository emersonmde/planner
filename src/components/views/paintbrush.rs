/// Paintbrush mode components and logic for interactive allocation editing
use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{Allocation, Assignment, Plan, ProjectColor};
use crate::state::use_plan_state;

/// Tracks which project is selected for paintbrush mode
#[derive(Clone, PartialEq, Debug)]
pub enum SelectedProject {
    None,
    Oncall,
    Technical(Uuid),
}

impl SelectedProject {
    /// Get the display name of the selected project
    pub fn get_name(&self, plan: &Plan) -> Option<String> {
        match self {
            SelectedProject::None => None,
            SelectedProject::Oncall => Some("Oncall".to_string()),
            SelectedProject::Technical(id) => {
                plan.get_technical_project(id).map(|p| p.name.clone())
            }
        }
    }
}

/// Paintbrush controls component (toggle + selected indicator)
#[component]
pub fn PaintbrushControls(
    active: bool,
    selected_project: SelectedProject,
    on_toggle: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let plan = use_plan_state();
    let selected_name = selected_project.get_name(&plan());

    rsx! {
        div { class: "paintbrush-controls",
            // Toggle button
            button {
                class: if active { "paintbrush-toggle active" } else { "paintbrush-toggle" },
                onclick: move |evt| on_toggle.call(evt),
                span { class: "toggle-label",
                    "Paintbrush Mode: "
                    span { class: "toggle-state",
                        if active { "ON" } else { "OFF" }
                    }
                }
            }

            // Selected project indicator (shown when paintbrush is ON)
            if active {
                if let Some(project_name) = selected_name {
                    div { class: "selected-project-indicator",
                        span { class: "indicator-label", "Selected:" }
                        span { class: "indicator-value", "{project_name}" }
                    }
                } else {
                    div { class: "selected-project-indicator warning",
                        span { class: "indicator-label", "⚠ No project selected" }
                    }
                }
            }

            // Project selector (passed as children)
            if active {
                {children}
            }
        }
    }
}

/// Project selector dropdown component
#[component]
pub fn ProjectSelector(
    search_query: String,
    selected_project: SelectedProject,
    on_search_change: EventHandler<String>,
    on_project_select: EventHandler<SelectedProject>,
) -> Element {
    let plan = use_plan_state();
    let plan_data = plan();

    // Filter projects based on search
    let filtered_projects: Vec<_> = plan_data
        .technical_projects
        .iter()
        .filter(|p| {
            search_query.is_empty() || p.name.to_lowercase().contains(&search_query.to_lowercase())
        })
        .collect();

    rsx! {
        div { class: "project-selector-container",
            // Search input
            input {
                class: "project-search",
                r#type: "text",
                placeholder: "Search projects...",
                value: "{search_query}",
                oninput: move |e| on_search_change.call(e.value()),
            }

            // Project list
            div { class: "project-list",
                for project in filtered_projects {
                    {
                        let project_id = project.id;
                        let is_selected = matches!(selected_project, SelectedProject::Technical(id) if id == project_id);
                        let allocated = plan_data.calculate_project_allocated_weeks(&project_id);
                        let color = project.roadmap_project_id
                            .and_then(|id| plan_data.get_roadmap_project(&id))
                            .map(|rp| rp.color)
                            .unwrap_or(ProjectColor::Blue);
                        let color_hex = color.to_hex();

                        rsx! {
                            button {
                                class: if is_selected { "project-option selected" } else { "project-option" },
                                onclick: move |_| on_project_select.call(SelectedProject::Technical(project_id)),
                                span {
                                    class: "project-color-dot",
                                    style: "background-color: {color_hex};",
                                }
                                span { class: "project-name", "{project.name}" }
                                span { class: "project-allocated", "{allocated:.1}w" }
                            }
                        }
                    }
                }

                // Oncall option at bottom
                button {
                    class: if matches!(selected_project, SelectedProject::Oncall) { "project-option oncall-option selected" } else { "project-option oncall-option" },
                    onclick: move |_| on_project_select.call(SelectedProject::Oncall),
                    span { class: "oncall-indicator", "📞" }
                    span { class: "project-name", "Oncall" }
                }
            }
        }
    }
}

/// Allocates a project to a specific cell
/// Returns true if successful, false if validation failed
pub fn allocate_project_to_cell(
    plan: &mut Signal<Plan>,
    selected_project: &SelectedProject,
    team_member_id: Uuid,
    week_start: chrono::NaiveDate,
) -> bool {
    match selected_project {
        SelectedProject::None => false, // Validation failed
        SelectedProject::Oncall => {
            plan.with_mut(|p| {
                // Remove existing allocation if any
                p.allocations.retain(|a| {
                    !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                });

                let mut alloc = Allocation::new(team_member_id, week_start);
                alloc.assignments.push(Assignment::oncall());
                p.allocations.push(alloc);
            });
            true
        }
        SelectedProject::Technical(project_id) => {
            plan.with_mut(|p| {
                // Remove existing allocation if any
                p.allocations.retain(|a| {
                    !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                });

                let mut alloc = Allocation::new(team_member_id, week_start);
                alloc.assignments.push(Assignment::new(*project_id, 100.0));
                p.allocations.push(alloc);
            });
            true
        }
    }
}
