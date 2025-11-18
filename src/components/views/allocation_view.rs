use dioxus::prelude::*;
use std::collections::HashMap;

use crate::components::ui::{GridCell, GridCellVariant};
use crate::state::use_plan_state;
use crate::utils::generate_quarter_weeks;

/// Allocation Grid view - displays the weekly allocation grid for engineers
/// Reference: docs/ui-design.md section 7.3
#[component]
pub fn AllocationView() -> Element {
    let plan = use_plan_state();
    let plan_data = plan();

    // Generate weeks for the quarter
    let weeks = generate_quarter_weeks(
        plan_data.quarter_start_date,
        plan_data.weeks_in_quarter,
        plan_data.sprint_length_weeks,
    );

    // Calculate grid columns dynamically (1 for row header + weeks)
    let grid_template_columns = format!("180px repeat({}, 150px)", weeks.len());

    // Pre-compute allocation map for O(1) lookups
    let allocation_map: HashMap<(uuid::Uuid, chrono::NaiveDate), &crate::models::Allocation> =
        plan_data
            .allocations
            .iter()
            .map(|a| ((a.team_member_id, a.week_start_date), a))
            .collect();

    rsx! {
        div { class: "view active allocation-grid-view",
            div { class: "allocation-grid-container",
                div {
                    class: "allocation-grid",
                    style: "grid-template-columns: {grid_template_columns};",

                    // Top-left corner cell (empty)
                    div { class: "grid-header-corner" }

                    // Column headers (weeks)
                    for week in weeks.iter() {
                        {
                            let is_sprint_start = week.is_sprint_start();
                            let sprint_class = if is_sprint_start {
                                "grid-week-header sprint-separator"
                            } else {
                                "grid-week-header"
                            };

                            rsx! {
                                div { class: "{sprint_class}",
                                    // Sprint number (always present for alignment, but empty if not sprint start)
                                    div { class: "sprint-label",
                                        if is_sprint_start {
                                            "Sprint {week.sprint_number}"
                                        }
                                    }
                                    // Week date (always shown)
                                    div { class: "week-date",
                                        "{week.format_date(true)}"
                                    }
                                    // Week progress
                                    div { class: "week-progress",
                                        "{week.format_week_number()}"
                                    }
                                }
                            }
                        }
                    }

                    // Team member rows
                    for engineer in &plan_data.team_members {
                        {
                            let engineer_id = engineer.id;
                            let allocated = plan_data.calculate_allocated_weeks(&engineer_id);
                            let capacity = engineer.capacity;
                            let diff = (allocated - capacity).abs();

                            let capacity_status = if diff <= 0.5 {
                                "success"
                            } else if diff <= 1.0 {
                                "warning"
                            } else {
                                "error"
                            };

                            let utilization_pct = if capacity > 0.0 {
                                (allocated / capacity * 100.0).min(100.0)
                            } else {
                                0.0
                            };

                            rsx! {
                                // Engineer row header
                                div { class: "grid-row-header",
                                    div { class: "engineer-info",
                                        div { class: "engineer-name-row",
                                            span { class: "engineer-name", "{engineer.name}" }
                                            span {
                                                class: "role-badge",
                                                "{engineer.role.short_name()}"
                                            }
                                        }
                                        div { class: "capacity-row",
                                            span {
                                                class: "capacity-text capacity-{capacity_status}",
                                                "{allocated} / {capacity} weeks"
                                            }
                                            div { class: "capacity-bar",
                                                div {
                                                    class: "capacity-bar-fill capacity-{capacity_status}",
                                                    style: "width: {utilization_pct}%",
                                                }
                                            }
                                        }
                                    }
                                }

                                // Grid cells for this engineer
                                for week in &weeks {
                                    {
                                        // Find allocation for this team member and week (O(1) lookup)
                                        let allocation = allocation_map.get(&(engineer_id, week.start_date)).copied();

                                        // Determine cell variant based on allocation
                                        let variant = if let Some(alloc) = allocation {
                                            if alloc.is_oncall() {
                                                GridCellVariant::Oncall
                                            } else if alloc.assignments.len() == 2 {
                                                // Split allocation
                                                let assignment1 = &alloc.assignments[0];
                                                let assignment2 = &alloc.assignments[1];

                                                let project1 = plan_data.get_technical_project(&assignment1.technical_project_id);
                                                let project2 = plan_data.get_technical_project(&assignment2.technical_project_id);

                                                if let (Some(p1), Some(p2)) = (project1, project2) {
                                                    let rp1 = p1.roadmap_project_id.and_then(|id| plan_data.get_roadmap_project(&id));
                                                    let rp2 = p2.roadmap_project_id.and_then(|id| plan_data.get_roadmap_project(&id));

                                                    GridCellVariant::Split {
                                                        project1_name: p1.name.clone(),
                                                        project1_color: rp1.map(|rp| rp.color).unwrap_or(crate::models::ProjectColor::Blue),
                                                        project1_percentage: assignment1.percentage,
                                                        project2_name: p2.name.clone(),
                                                        project2_color: rp2.map(|rp| rp.color).unwrap_or(crate::models::ProjectColor::Green),
                                                        project2_percentage: assignment2.percentage,
                                                    }
                                                } else {
                                                    GridCellVariant::Empty
                                                }
                                            } else if let Some(assignment) = alloc.assignments.first() {
                                                // Single project allocation
                                                let project = plan_data.get_technical_project(&assignment.technical_project_id);

                                                if let Some(proj) = project {
                                                    let roadmap_project = proj.roadmap_project_id.and_then(|id| plan_data.get_roadmap_project(&id));
                                                    let project_color = roadmap_project.map(|rp| rp.color).unwrap_or(crate::models::ProjectColor::Blue);
                                                    let is_before_start = week.start_date < proj.start_date;

                                                    // TODO: Detect multi-week series for connected cells
                                                    // For Phase 4 read-only, we'll use SingleWeek
                                                    // Multi-week detection will be enhanced in future phases
                                                    GridCellVariant::SingleWeek {
                                                        project_name: proj.name.clone(),
                                                        project_color,
                                                        percentage: assignment.percentage,
                                                        is_before_start,
                                                    }
                                                } else {
                                                    GridCellVariant::Empty
                                                }
                                            } else {
                                                GridCellVariant::Empty
                                            }
                                        } else {
                                            GridCellVariant::Empty
                                        };

                                        rsx! {
                                            GridCell { variant }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
