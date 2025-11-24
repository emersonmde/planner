use dioxus::prelude::*;
use std::collections::HashSet;
use uuid::Uuid;

use crate::components::ui::{
    Badge, Button, ButtonVariant, CellStyle, ConfirmationDialog, DataTable, Input, ProjectName,
    TableCell, TableHeader, TableHeaderCell, TableRow, TechnicalModalMode, TechnicalProjectModal,
};
use crate::models::{get_capacity_status, ProjectColor};
use crate::state::{use_plan_state, use_preferences};

/// Filter options for technical projects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FilterOption {
    All,
    FullyAllocated,
    NeedsAllocation,
    NoLink,
}

/// Sort options for technical projects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortOption {
    Roadmap,
    Status,
    Allocation,
}

/// Technical Projects view - displays technical projects with side panel filters
/// Reference: docs/ui-design.md section 7.2
#[component]
pub fn TechnicalView() -> Element {
    let mut plan = use_plan_state();
    let preferences = use_preferences();
    let plan_data = plan();
    let prefs_data = preferences();

    // Local state for filters and sorting
    let search_query = use_signal(String::new);
    let mut active_filters = use_signal(|| {
        let mut filters = HashSet::new();
        filters.insert(FilterOption::All);
        filters
    });
    let mut sort_by = use_signal(|| SortOption::Roadmap);

    // Modal state
    let mut modal_visible = use_signal(|| false);
    let mut modal_mode = use_signal(|| TechnicalModalMode::Add);
    let mut modal_initial_name = use_signal(String::new);
    let mut modal_initial_roadmap_id = use_signal(|| None::<Uuid>);
    let mut modal_initial_eng_estimate = use_signal(|| 0.0_f32);
    let mut modal_initial_sci_estimate = use_signal(|| 0.0_f32);
    let mut modal_initial_start_date = use_signal(|| plan_data.quarter_start_date);
    let mut modal_initial_completion = use_signal(|| None::<chrono::NaiveDate>);
    let mut modal_initial_notes = use_signal(String::new);

    // Delete confirmation dialog state
    let mut delete_dialog_visible = use_signal(|| false);
    let mut delete_project_id = use_signal(|| None::<Uuid>);
    let mut delete_project_name = use_signal(String::new);
    let mut delete_allocated_weeks = use_signal(|| 0.0_f32);

    // Filter and sort technical projects
    let mut filtered_projects: Vec<_> = plan_data
        .technical_projects
        .iter()
        .filter(|project| {
            let filters = active_filters();

            // Search filter
            if !search_query().is_empty() {
                let query = search_query().to_lowercase();
                if !project.name.to_lowercase().contains(&query) {
                    return false;
                }
            }

            // Allocation filters
            if filters.contains(&FilterOption::All) {
                return true;
            }

            let allocated = plan_data.calculate_project_allocated_weeks(&project.id);
            let estimated = project.total_estimate();

            // Check allocation ratio
            let is_fully_allocated = if estimated > 0.0 {
                let ratio = allocated / estimated;
                (0.9..=1.1).contains(&ratio)
            } else {
                allocated == 0.0 // No estimate = "fully allocated" if no allocation
            };

            if filters.contains(&FilterOption::FullyAllocated) && is_fully_allocated {
                return true;
            }

            if filters.contains(&FilterOption::NeedsAllocation) && !is_fully_allocated {
                return true;
            }

            if filters.contains(&FilterOption::NoLink) && project.roadmap_project_id.is_none() {
                return true;
            }

            false
        })
        .collect();

    // Sort projects
    match sort_by() {
        SortOption::Roadmap => {
            filtered_projects.sort_by_key(|p| {
                p.roadmap_project_id
                    .and_then(|id| plan_data.get_roadmap_project(&id).map(|rp| rp.name.clone()))
            });
        }
        SortOption::Status => {
            // Sort by allocation status: needs attention first (under/over allocated)
            filtered_projects.sort_by_key(|p| {
                let allocated = plan_data.calculate_project_allocated_weeks(&p.id);
                let estimated = p.total_estimate();
                if estimated == 0.0 {
                    if allocated == 0.0 {
                        2
                    } else {
                        1
                    } // No estimate: neutral or warning
                } else {
                    let ratio = allocated / estimated;
                    if (0.9..=1.1).contains(&ratio) {
                        2 // Fully allocated - lowest priority
                    } else {
                        0 // Needs attention - highest priority
                    }
                }
            });
        }
        SortOption::Allocation => {
            filtered_projects.sort_by(|a, b| {
                let a_allocated = plan_data.calculate_project_allocated_weeks(&a.id);
                let b_allocated = plan_data.calculate_project_allocated_weeks(&b.id);
                b_allocated.partial_cmp(&a_allocated).unwrap()
            });
        }
    }

    // Get roadmap projects for the dropdown
    let roadmap_projects = plan_data.roadmap_projects.clone();

    rsx! {
        div { class: "view technical-view",
            // Side panel with filters
            div { class: "side-panel",
                // Filter section
                div { class: "side-panel-section",
                    h3 { class: "section-title", "FILTER" }

                    // All Projects filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                if filters.contains(&FilterOption::All) {
                                    filters.remove(&FilterOption::All);
                                } else {
                                    filters.clear();
                                    filters.insert(FilterOption::All);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::All) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "All Projects" }
                    }

                    // Fully Allocated filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                filters.remove(&FilterOption::All);
                                if filters.contains(&FilterOption::FullyAllocated) {
                                    filters.remove(&FilterOption::FullyAllocated);
                                } else {
                                    filters.insert(FilterOption::FullyAllocated);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::FullyAllocated) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "Fully Allocated" }
                    }

                    // Needs Allocation filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                filters.remove(&FilterOption::All);
                                if filters.contains(&FilterOption::NeedsAllocation) {
                                    filters.remove(&FilterOption::NeedsAllocation);
                                } else {
                                    filters.insert(FilterOption::NeedsAllocation);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::NeedsAllocation) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "Needs Allocation" }
                    }

                    // No Roadmap Link filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                filters.remove(&FilterOption::All);
                                if filters.contains(&FilterOption::NoLink) {
                                    filters.remove(&FilterOption::NoLink);
                                } else {
                                    filters.insert(FilterOption::NoLink);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::NoLink) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "No Roadmap Link" }
                    }
                }

                // Sort section
                div { class: "side-panel-section",
                    h3 { class: "section-title", "SORT BY" }

                    // Sort by Roadmap
                    div {
                        class: "filter-option",
                        onclick: move |_| sort_by.set(SortOption::Roadmap),
                        div {
                            class: if sort_by() == SortOption::Roadmap { "radio checked" } else { "radio" }
                        }
                        span { "Roadmap Project" }
                    }

                    // Sort by Needs Attention
                    div {
                        class: "filter-option",
                        onclick: move |_| sort_by.set(SortOption::Status),
                        div {
                            class: if sort_by() == SortOption::Status { "radio checked" } else { "radio" }
                        }
                        span { "Needs Attention" }
                    }

                    // Sort by Allocated Amount
                    div {
                        class: "filter-option",
                        onclick: move |_| sort_by.set(SortOption::Allocation),
                        div {
                            class: if sort_by() == SortOption::Allocation { "radio checked" } else { "radio" }
                        }
                        span { "Allocated Amount" }
                    }
                }
            }

            // Main content area
            div { class: "view-content",
                // Search and actions bar (matching Roadmap style)
                div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: var(--space-lg);",
                    Input {
                        value: search_query,
                        placeholder: "Search technical projects...".to_string(),
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| {
                            // Reset modal to Add mode with default values
                            modal_mode.set(TechnicalModalMode::Add);
                            modal_initial_name.set(String::new());
                            modal_initial_roadmap_id.set(None);
                            modal_initial_eng_estimate.set(0.0);
                            modal_initial_sci_estimate.set(0.0);
                            modal_initial_start_date.set(plan_data.quarter_start_date);
                            modal_initial_completion.set(None);
                            modal_initial_notes.set(String::new());
                            modal_visible.set(true);
                        },
                        "+ New Technical Project"
                    }
                }

                // Technical Projects table
                DataTable {
                    TableHeader {
                        TableHeaderCell { "Project Name" }
                        TableHeaderCell { "Roadmap" }
                        TableHeaderCell { "Eng Est." }
                        TableHeaderCell { "Sci Est." }
                        TableHeaderCell { "Total Est." }
                        TableHeaderCell { "Eng Alloc." }
                        TableHeaderCell { "Sci Alloc." }
                        TableHeaderCell { "Total Alloc." }
                        TableHeaderCell { "Team" }
                        TableHeaderCell { "Start" }
                        TableHeaderCell { "End" }
                    }

                    for project in filtered_projects {
                        {
                            // Get role lookup function
                            let get_role = |member_id: &Uuid| {
                                prefs_data.team_members.iter()
                                    .find(|m| &m.id == member_id)
                                    .map(|m| m.role)
                            };

                            // Calculate allocations by role
                            let (eng_alloc, sci_alloc, total_alloc) = plan_data
                                .calculate_technical_project_allocated_by_role(&project.id, get_role);

                            // Normalize to avoid -0.0 display
                            let eng_alloc = if eng_alloc == 0.0 { 0.0 } else { eng_alloc };
                            let sci_alloc = if sci_alloc == 0.0 { 0.0 } else { sci_alloc };
                            let total_alloc = if total_alloc == 0.0 { 0.0 } else { total_alloc };

                            // Get estimates
                            let eng_estimate = project.eng_estimate;
                            let sci_estimate = project.sci_estimate;
                            let total_estimate = project.total_estimate();

                            // Get allocation status for each category
                            let eng_status = get_capacity_status(eng_alloc, eng_estimate);
                            let sci_status = get_capacity_status(sci_alloc, sci_estimate);
                            let total_status = get_capacity_status(total_alloc, total_estimate);

                            // Get assigned team members
                            let assigned_member_ids = plan_data.get_assigned_team_members(&project.id);
                            let assigned_names: Vec<String> = assigned_member_ids
                                .iter()
                                .filter_map(|id| {
                                    prefs_data.team_members.iter()
                                        .find(|m| &m.id == id)
                                        .map(|m| m.name.clone())
                                })
                                .collect();

                            // Fix team display: get first name only for truncated display
                            let team_display = if assigned_names.is_empty() {
                                "—".to_string()
                            } else if assigned_names.len() == 1 {
                                assigned_names[0].clone()
                            } else if assigned_names.len() == 2 {
                                // For 2 members, show first names only
                                let first_names: Vec<&str> = assigned_names
                                    .iter()
                                    .map(|n| n.split_whitespace().next().unwrap_or(n.as_str()))
                                    .collect();
                                first_names.join(", ")
                            } else {
                                // For 3+ members, show first person's first name + count
                                let first_name = assigned_names[0]
                                    .split_whitespace()
                                    .next()
                                    .unwrap_or(&assigned_names[0]);
                                format!("{} +{}", first_name, assigned_names.len() - 1)
                            };

                            // Get date range from allocations
                            let date_range = plan_data.get_project_allocation_date_range(&project.id);
                            let (start_display, end_display) = match date_range {
                                Some((start, end)) => {
                                    // End date is end of the week (start + 6 days)
                                    let end_date = end + chrono::Duration::days(6);
                                    (
                                        start.format("%b %-d").to_string(),
                                        end_date.format("%b %-d").to_string(),
                                    )
                                }
                                None => ("—".to_string(), "—".to_string()),
                            };

                            let roadmap_project = project.roadmap_project_id
                                .and_then(|id| plan_data.get_roadmap_project(&id));

                            let project_color = roadmap_project
                                .map(|rp| rp.color)
                                .unwrap_or(ProjectColor::Blue);

                            // Clone values needed for event handlers
                            let project_id = project.id;
                            let project_name = project.name.clone();
                            let project_roadmap_id = project.roadmap_project_id;
                            let project_eng_estimate = project.eng_estimate;
                            let project_sci_estimate = project.sci_estimate;
                            let project_start = project.start_date;
                            let project_completion = project.expected_completion;
                            let project_notes = project.notes.clone().unwrap_or_default();

                            rsx! {
                                TableRow {
                                    // Project name with hover actions (matching Roadmap style)
                                    TableCell {
                                        style: CellStyle::Emphasis,
                                        div {
                                            class: "project-name-cell",
                                            ProjectName {
                                                name: project.name.clone(),
                                                color: project_color.to_hex().to_string(),
                                            }
                                            // Hover actions
                                            {
                                                let edit_name = project_name.clone();
                                                let edit_notes = project_notes.clone();
                                                let delete_name = project_name.clone();

                                                rsx! {
                                                    div {
                                                        class: "row-actions",
                                                        // Edit button
                                                        button {
                                                            class: "icon-button",
                                                            title: "Edit project",
                                                            onclick: move |_| {
                                                                modal_mode.set(TechnicalModalMode::Edit(project_id));
                                                                modal_initial_name.set(edit_name.clone());
                                                                modal_initial_roadmap_id.set(project_roadmap_id);
                                                                modal_initial_eng_estimate.set(project_eng_estimate);
                                                                modal_initial_sci_estimate.set(project_sci_estimate);
                                                                modal_initial_start_date.set(project_start);
                                                                modal_initial_completion.set(project_completion);
                                                                modal_initial_notes.set(edit_notes.clone());
                                                                modal_visible.set(true);
                                                            },
                                                            "⚙"
                                                        }
                                                        // Delete button
                                                        button {
                                                            class: "icon-button danger",
                                                            title: "Delete project",
                                                            onclick: move |_| {
                                                                delete_project_id.set(Some(project_id));
                                                                delete_project_name.set(delete_name.clone());
                                                                delete_allocated_weeks.set(total_alloc);
                                                                delete_dialog_visible.set(true);
                                                            },
                                                            "🗑"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // Roadmap project
                                    TableCell {
                                        style: CellStyle::Secondary,
                                        if let Some(rp) = roadmap_project {
                                            "{rp.name}"
                                        } else {
                                            "—"
                                        }
                                    }
                                    // Engineering estimate
                                    TableCell {
                                        style: CellStyle::MonospaceSecondary,
                                        "{eng_estimate}"
                                    }
                                    // Science estimate
                                    TableCell {
                                        style: CellStyle::MonospaceSecondary,
                                        "{sci_estimate}"
                                    }
                                    // Total estimate
                                    TableCell {
                                        style: CellStyle::MonospaceEmphasis,
                                        "{total_estimate}"
                                    }
                                    // Engineering allocation (with status badge)
                                    TableCell {
                                        style: CellStyle::Default,
                                        Badge {
                                            badge_type: eng_status,
                                            "{eng_alloc}"
                                        }
                                    }
                                    // Science allocation (with status badge)
                                    TableCell {
                                        style: CellStyle::Default,
                                        Badge {
                                            badge_type: sci_status,
                                            "{sci_alloc}"
                                        }
                                    }
                                    // Total allocation (with status badge)
                                    TableCell {
                                        style: CellStyle::Default,
                                        Badge {
                                            badge_type: total_status,
                                            "{total_alloc}"
                                        }
                                    }
                                    // Team members
                                    TableCell {
                                        style: CellStyle::Secondary,
                                        "{team_display}"
                                    }
                                    // Start date
                                    TableCell {
                                        style: CellStyle::Secondary,
                                        "{start_display}"
                                    }
                                    // End date
                                    TableCell {
                                        style: CellStyle::Secondary,
                                        "{end_display}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Technical Project Modal
            if modal_visible() {
                TechnicalProjectModal {
                    mode: modal_mode(),
                    initial_name: modal_initial_name(),
                    initial_roadmap_project_id: modal_initial_roadmap_id(),
                    initial_eng_estimate: modal_initial_eng_estimate(),
                    initial_sci_estimate: modal_initial_sci_estimate(),
                    initial_start_date: modal_initial_start_date(),
                    initial_expected_completion: modal_initial_completion(),
                    initial_notes: modal_initial_notes(),
                    roadmap_projects: roadmap_projects.clone(),
                    on_save: move |project| {
                        match modal_mode() {
                            TechnicalModalMode::Add => {
                                plan.write().technical_projects.push(project);
                            }
                            TechnicalModalMode::Edit(id) => {
                                if let Some(existing) = plan
                                    .write()
                                    .technical_projects
                                    .iter_mut()
                                    .find(|p| p.id == id)
                                {
                                    *existing = project;
                                }
                            }
                        }
                        plan.write().mark_modified();
                        modal_visible.set(false);
                    },
                    on_cancel: move |_| {
                        modal_visible.set(false);
                    },
                }
            }

            // Delete Confirmation Dialog
            {
                let allocated_weeks = delete_allocated_weeks();
                let warning_message = if allocated_weeks > 0.0 {
                    format!(
                        "This will remove {:.1} weeks of allocations.",
                        allocated_weeks
                    )
                } else {
                    String::new()
                };

                rsx! {
                    ConfirmationDialog {
                        visible: delete_dialog_visible(),
                        title: "Delete Technical Project".to_string(),
                        message: format!("Are you sure you want to delete \"{}\"?", delete_project_name()),
                        warning: warning_message,
                        confirm_label: "Delete".to_string(),
                        cancel_label: "Cancel".to_string(),
                        on_confirm: move |_| {
                            if let Some(id) = delete_project_id() {
                                // Remove all allocations referencing this project
                                plan.write().allocations.iter_mut().for_each(|alloc| {
                                    alloc.assignments.retain(|a| a.technical_project_id != id);
                                });
                                // Remove empty allocations
                                plan.write().allocations.retain(|alloc| !alloc.assignments.is_empty());
                                // Remove the technical project
                                plan.write().technical_projects.retain(|p| p.id != id);
                                plan.write().mark_modified();
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
