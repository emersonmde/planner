use dioxus::prelude::*;

use crate::components::ui::{
    Badge, Button, ButtonVariant, CellStyle, DataTable, Input, ProjectName, TableCell, TableHeader,
    TableHeaderCell, TableRow,
};
use crate::models::{get_capacity_status, Role};
use crate::state::{use_plan_state, use_preferences};

/// Roadmap view - displays roadmap projects table and quarter summary
/// Reference: docs/ui-design.md section 7.1
#[component]
pub fn RoadmapView() -> Element {
    let plan_state = use_plan_state();
    let preferences = use_preferences();

    let plan_data = plan_state();
    let prefs_data = preferences();

    // Search filter state
    let search_query = use_signal(String::new);

    // Filter projects based on search query
    let filtered_projects: Vec<_> = plan_data
        .roadmap_projects
        .iter()
        .filter(|project| {
            let query = search_query().to_lowercase();
            query.is_empty() || project.name.to_lowercase().contains(&query)
        })
        .collect();

    // Calculate quarter summary stats
    let (eng_capacity, sci_capacity, total_capacity) = {
        let mut eng = 0.0;
        let mut sci = 0.0;
        for member in &prefs_data.team_members {
            match member.role {
                Role::Engineering => eng += member.capacity,
                Role::Science => sci += member.capacity,
            }
        }
        (eng, sci, eng + sci)
    };

    let (eng_allocated, sci_allocated, total_allocated) = {
        let mut eng = 0.0;
        let mut sci = 0.0;
        for alloc in &plan_data.allocations {
            if let Some(member) = prefs_data
                .team_members
                .iter()
                .find(|m| m.id == alloc.team_member_id)
            {
                let weeks = alloc.total_percentage() / 100.0;
                match member.role {
                    Role::Engineering => eng += weeks,
                    Role::Science => sci += weeks,
                }
            }
        }
        (eng, sci, eng + sci)
    };

    let utilization = if total_capacity > 0.0 {
        (total_allocated / total_capacity * 100.0).round() as i32
    } else {
        0
    };

    // Calculate team composition
    let eng_count = prefs_data
        .team_members
        .iter()
        .filter(|e| e.role == Role::Engineering)
        .count();
    let sci_count = prefs_data
        .team_members
        .iter()
        .filter(|e| e.role == Role::Science)
        .count();

    rsx! {
        div { class: "view active",
            // Search and action bar
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-lg);",
                Input {
                    value: search_query,
                    placeholder: "Search roadmap projects...".to_string(),
                }
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: move |_| {
                        // Placeholder for add project functionality
                    },
                    "Add Roadmap Project"
                }
            }

            // Roadmap projects table
            DataTable {
                TableHeader {
                    TableHeaderCell { "Project Name" }
                    TableHeaderCell { "Eng Est." }
                    TableHeaderCell { "Sci Est." }
                    TableHeaderCell { "Total Est." }
                    TableHeaderCell { "Eng Alloc." }
                    TableHeaderCell { "Sci Alloc." }
                    TableHeaderCell { "Total Alloc." }
                    TableHeaderCell { "Start Date" }
                    TableHeaderCell { "Launch Date" }
                    TableHeaderCell { "Notes" }
                }

                // Table rows
                for project in filtered_projects {
                    {
                        let get_role = |member_id: &uuid::Uuid| {
                            prefs_data.team_members.iter()
                                .find(|m| &m.id == member_id)
                                .map(|m| m.role)
                        };
                        let (eng_alloc, sci_alloc, total_alloc) = plan_data.calculate_roadmap_allocated_weeks(&project.id, get_role);

                        let eng_status = get_capacity_status(eng_alloc, project.eng_estimate);
                        let sci_status = get_capacity_status(sci_alloc, project.sci_estimate);
                        let total_estimate = project.eng_estimate + project.sci_estimate;
                        let total_status = get_capacity_status(total_alloc, total_estimate);

                        rsx! {
                            TableRow {
                                // Project name with color dot
                                TableCell {
                                    style: CellStyle::Emphasis,
                                    ProjectName {
                                        name: project.name.clone(),
                                        color: project.color.to_hex(),
                                    }
                                }

                                // Engineering estimate
                                TableCell {
                                    style: CellStyle::MonospaceSecondary,
                                    "{project.eng_estimate}"
                                }

                                // Science estimate
                                TableCell {
                                    style: CellStyle::MonospaceSecondary,
                                    "{project.sci_estimate}"
                                }

                                // Total estimate
                                TableCell {
                                    style: CellStyle::MonospaceEmphasis,
                                    "{total_estimate}"
                                }

                                // Engineering allocation
                                TableCell {
                                    style: CellStyle::Default,
                                    Badge {
                                        badge_type: eng_status,
                                        "{eng_alloc}"
                                    }
                                }

                                // Science allocation
                                TableCell {
                                    style: CellStyle::Default,
                                    Badge {
                                        badge_type: sci_status,
                                        "{sci_alloc}"
                                    }
                                }

                                // Total allocation
                                TableCell {
                                    style: CellStyle::Default,
                                    Badge {
                                        badge_type: total_status,
                                        "{total_alloc}"
                                    }
                                }

                                // Start date
                                TableCell {
                                    style: CellStyle::Secondary,
                                    "{project.start_date.format(\"%b %-d\")}"
                                }

                                // Launch date
                                TableCell {
                                    style: CellStyle::Secondary,
                                    "{project.launch_date.format(\"%b %-d\")}"
                                }

                                // Notes (truncated)
                                TableCell {
                                    style: CellStyle::Secondary,
                                    if let Some(ref notes) = project.notes {
                                        "{notes}"
                                    } else {
                                        ""
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Quarter summary section
            div { class: "quarter-summary",
                h2 { class: "summary-title", "{plan_data.quarter_name Summary" }
                div { class: "summary-metrics",
                    // Total capacity
                    div { class: "metric",
                        div { class: "metric-label", "Total Capacity" }
                        div { class: "metric-value", "{total_capacity} weeks" }
                        div { class: "metric-label",
                            {
                                let eng_part = if eng_count > 0 {
                                    format!("{} Engineers × {:.1}", eng_count, eng_capacity / eng_count as f32)
                                } else {
                                    "No engineers".to_string()
                                };
                                let sci_part = if sci_count > 0 {
                                    format!("{} Scientists × {:.1}", sci_count, sci_capacity / sci_count as f32)
                                } else {
                                    "No scientists".to_string()
                                };
                                format!("{} + {}", eng_part, sci_part)
                            }
                        }
                    }

                    // Total allocated
                    div { class: "metric",
                        div { class: "metric-label", "Total Allocated" }
                        div { class: "metric-value", "{total_allocated} weeks" }
                        div { class: "metric-label", "Eng: {eng_allocated} / Sci: {sci_allocated}" }
                    }

                    // Utilization
                    div { class: "metric",
                        div { class: "metric-label", "Utilization" }
                        div {
                            class: "metric-value",
                            style: if utilization >= 80 { "color: var(--success-50)" } else { "color: var(--warning-50)" },
                            "{utilization}%"
                        }
                        div { class: "metric-label",
                            if utilization >= 80 {
                                "On track for quarter"
                            } else {
                                "Below target"
                            }
                        }
                    }
                }
            }
        }
    }
}
