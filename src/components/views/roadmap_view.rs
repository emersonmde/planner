use dioxus::prelude::*;

use crate::components::ui::{
    Badge, Button, ButtonVariant, CellStyle, ConfirmationDialog, DataTable, Input, ModalMode,
    ProjectName, RoadmapProjectModal, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::models::{get_capacity_status, ProjectColor, Role};
use crate::state::{use_plan_state, use_preferences};

/// Roadmap view - displays roadmap projects table and quarter summary
/// Reference: docs/ui-design.md section 7.1
#[component]
pub fn RoadmapView() -> Element {
    let mut plan_state = use_plan_state();
    let preferences = use_preferences();

    let plan_data = plan_state();
    let prefs_data = preferences();

    // Search filter state
    let search_query = use_signal(String::new);

    // Modal state
    let mut modal_visible = use_signal(|| false);
    let mut modal_mode = use_signal(|| ModalMode::Add);
    let mut modal_initial_name = use_signal(String::new);
    let mut modal_initial_eng_estimate = use_signal(|| 0.0);
    let mut modal_initial_sci_estimate = use_signal(|| 0.0);
    let mut modal_initial_start_date = use_signal(|| plan_data.quarter_start_date);
    let mut modal_initial_launch_date = use_signal(|| {
        plan_data.quarter_start_date + chrono::Duration::weeks(plan_data.num_weeks as i64)
    });
    let mut modal_initial_color = use_signal(|| ProjectColor::Blue);
    let mut modal_initial_notes = use_signal(String::new);

    // Delete confirmation dialog state
    let mut delete_dialog_visible = use_signal(|| false);
    let mut delete_project_id = use_signal(|| None::<uuid::Uuid>);
    let mut delete_project_name = use_signal(String::new);

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
                        // Reset modal to Add mode with default values
                        modal_mode.set(ModalMode::Add);
                        modal_initial_name.set(String::new());
                        modal_initial_eng_estimate.set(0.0);
                        modal_initial_sci_estimate.set(0.0);
                        modal_initial_start_date.set(plan_data.quarter_start_date);
                        modal_initial_launch_date.set(plan_data.quarter_start_date + chrono::Duration::weeks(plan_data.num_weeks as i64));
                        modal_initial_color.set(ProjectColor::Blue);
                        modal_initial_notes.set(String::new());
                        modal_visible.set(true);
                    },
                    "+ New Roadmap Project"
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
                                // Project name with color dot and hover actions
                                TableCell {
                                    style: CellStyle::Emphasis,
                                    div {
                                        class: "project-name-cell",
                                        ProjectName {
                                            name: project.name.clone(),
                                            color: project.color.to_hex(),
                                        }
                                        // Hover actions
                                        {
                                            let project_id = project.id;
                                            let project_name = project.name.clone();
                                            let project_eng_estimate = project.eng_estimate;
                                            let project_sci_estimate = project.sci_estimate;
                                            let project_start_date = project.start_date;
                                            let project_launch_date = project.launch_date;
                                            let project_color = project.color;
                                            let project_notes = project.notes.clone().unwrap_or_default();

                                            rsx! {
                                                div {
                                                    class: "row-actions",
                                                    // Edit button
                                                    {
                                                        let edit_name = project_name.clone();
                                                        let edit_notes = project_notes.clone();
                                                        rsx! {
                                                            button {
                                                                class: "icon-button",
                                                                title: "Edit project",
                                                                onclick: move |_| {
                                                                    modal_mode.set(ModalMode::Edit(project_id));
                                                                    modal_initial_name.set(edit_name.clone());
                                                                    modal_initial_eng_estimate.set(project_eng_estimate);
                                                                    modal_initial_sci_estimate.set(project_sci_estimate);
                                                                    modal_initial_start_date.set(project_start_date);
                                                                    modal_initial_launch_date.set(project_launch_date);
                                                                    modal_initial_color.set(project_color);
                                                                    modal_initial_notes.set(edit_notes.clone());
                                                                    modal_visible.set(true);
                                                                },
                                                                "⚙"
                                                            }
                                                        }
                                                    }
                                                    // Delete button
                                                    {
                                                        let delete_name = project_name.clone();
                                                        rsx! {
                                                            button {
                                                                class: "icon-button danger",
                                                                title: "Delete project",
                                                                onclick: move |_| {
                                                                    delete_project_id.set(Some(project_id));
                                                                    delete_project_name.set(delete_name.clone());
                                                                    delete_dialog_visible.set(true);
                                                                },
                                                                "🗑"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
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

            // Roadmap Project Modal (conditionally rendered)
            if modal_visible() {
                RoadmapProjectModal {
                    mode: modal_mode(),
                    initial_name: modal_initial_name(),
                    initial_eng_estimate: modal_initial_eng_estimate(),
                    initial_sci_estimate: modal_initial_sci_estimate(),
                    initial_start_date: modal_initial_start_date(),
                    initial_launch_date: modal_initial_launch_date(),
                    initial_color: modal_initial_color(),
                    initial_notes: modal_initial_notes(),
                    on_save: move |project| {
                        // Add or update project in plan_state
                        match modal_mode() {
                            ModalMode::Add => {
                                plan_state.write().roadmap_projects.push(project);
                            }
                            ModalMode::Edit(id) => {
                                if let Some(existing) = plan_state
                                    .write()
                                    .roadmap_projects
                                    .iter_mut()
                                    .find(|p| p.id == id)
                                {
                                    *existing = project;
                                }
                            }
                        }
                        modal_visible.set(false);
                    },
                    on_cancel: move |_| {
                        modal_visible.set(false);
                    },
                }
            }

            // Delete Confirmation Dialog
            {
                let project_id = delete_project_id();
                let linked_tech_projects_count = if let Some(id) = project_id {
                    plan_data
                        .technical_projects
                        .iter()
                        .filter(|tp| tp.roadmap_project_id == Some(id))
                        .count()
                } else {
                    0
                };

                let warning_message = if linked_tech_projects_count > 0 {
                    format!(
                        "This will unlink {} technical project{}. The technical projects will not be deleted.",
                        linked_tech_projects_count,
                        if linked_tech_projects_count == 1 { "" } else { "s" }
                    )
                } else {
                    String::new()
                };

                rsx! {
                    ConfirmationDialog {
                        visible: delete_dialog_visible(),
                        title: "Delete Roadmap Project".to_string(),
                        message: format!("Are you sure you want to delete \"{}\"?", delete_project_name()),
                        warning: warning_message,
                        confirm_label: "Delete".to_string(),
                        cancel_label: "Cancel".to_string(),
                        on_confirm: move |_| {
                            if let Some(id) = delete_project_id() {
                                plan_state.with_mut(|p| {
                                    // Remove the roadmap project
                                    p.roadmap_projects.retain(|proj| proj.id != id);
                                    // Unlink all technical projects
                                    for tech_project in &mut p.technical_projects {
                                        if tech_project.roadmap_project_id == Some(id) {
                                            tech_project.roadmap_project_id = None;
                                        }
                                    }
                                });
                            }
                            delete_dialog_visible.set(false);
                        },
                        on_cancel: move |_| {
                            delete_dialog_visible.set(false);
                        },
                    }
                }
            }
        }
    }
}
