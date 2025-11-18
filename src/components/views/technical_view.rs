use dioxus::prelude::*;
use std::collections::HashSet;

use crate::components::ui::{
    Badge, BadgeType, DataTable, ProjectName, TableCell, TableHeader, TableHeaderCell, TableRow,
};
use crate::models::get_capacity_status;
use crate::state::use_plan_state;

/// Filter options for technical projects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FilterOption {
    All,
    OnTrack,
    AtRisk,
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
    let plan = use_plan_state();
    let plan_data = plan();

    // Local state for filters and sorting
    let mut search_query = use_signal(String::new);
    let mut active_filters = use_signal(|| {
        let mut filters = HashSet::new();
        filters.insert(FilterOption::All);
        filters
    });
    let mut sort_by = use_signal(|| SortOption::Roadmap);

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

            // Status filters
            if filters.contains(&FilterOption::All) {
                return true;
            }

            let allocated = plan_data.calculate_project_allocated_weeks(&project.id);
            let estimated = project.estimated_weeks;
            let status = get_capacity_status(allocated, estimated);

            if filters.contains(&FilterOption::OnTrack) && status == BadgeType::Success {
                return true;
            }

            if filters.contains(&FilterOption::AtRisk)
                && (status == BadgeType::Warning || status == BadgeType::Error)
            {
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
            filtered_projects.sort_by_key(|p| {
                let allocated = plan_data.calculate_project_allocated_weeks(&p.id);
                let estimated = p.estimated_weeks;
                let status = get_capacity_status(allocated, estimated);
                match status {
                    BadgeType::Error => 0,
                    BadgeType::Warning => 1,
                    BadgeType::Success => 2,
                    BadgeType::Info => 3,
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

                    // On Track filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                filters.remove(&FilterOption::All);
                                if filters.contains(&FilterOption::OnTrack) {
                                    filters.remove(&FilterOption::OnTrack);
                                } else {
                                    filters.insert(FilterOption::OnTrack);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::OnTrack) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "On Track" }
                    }

                    // At Risk filter
                    div {
                        class: "filter-option",
                        onclick: move |_| {
                            active_filters.with_mut(|filters| {
                                filters.remove(&FilterOption::All);
                                if filters.contains(&FilterOption::AtRisk) {
                                    filters.remove(&FilterOption::AtRisk);
                                } else {
                                    filters.insert(FilterOption::AtRisk);
                                }
                            });
                        },
                        div {
                            class: if active_filters().contains(&FilterOption::AtRisk) { "checkbox checked" } else { "checkbox" }
                        }
                        span { "At Risk" }
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

                    // Sort by Status
                    div {
                        class: "filter-option",
                        onclick: move |_| sort_by.set(SortOption::Status),
                        div {
                            class: if sort_by() == SortOption::Status { "radio checked" } else { "radio" }
                        }
                        span { "Status" }
                    }

                    // Sort by Allocation
                    div {
                        class: "filter-option",
                        onclick: move |_| sort_by.set(SortOption::Allocation),
                        div {
                            class: if sort_by() == SortOption::Allocation { "radio checked" } else { "radio" }
                        }
                        span { "Allocation" }
                    }
                }
            }

            // Main content area
            div { class: "view-content",
                // Search and actions bar
                div { class: "view-header",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search technical projects...",
                        value: "{search_query()}",
                        oninput: move |evt| search_query.set(evt.value()),
                    }
                    button { class: "button button-primary", "+ Add Technical Project" }
                }

                // Technical Projects table
                DataTable {
                    TableHeader {
                        TableHeaderCell { "Project Name" }
                        TableHeaderCell { "Roadmap Project" }
                        TableHeaderCell { "Estimated" }
                        TableHeaderCell { "Allocated" }
                        TableHeaderCell { "Status" }
                    }

                    for project in filtered_projects {
                        {
                            let allocated = plan_data.calculate_project_allocated_weeks(&project.id);
                            let estimated = project.estimated_weeks;
                            let status = get_capacity_status(allocated, estimated);

                            let roadmap_project = project.roadmap_project_id
                                .and_then(|id| plan_data.get_roadmap_project(&id));

                            let status_text = match status {
                                BadgeType::Success => "On Track",
                                BadgeType::Warning => "At Risk",
                                BadgeType::Error => "Critical",
                                BadgeType::Info => "Not Started",
                            };

                            rsx! {
                                TableRow {
                                    TableCell {
                                        if let Some(rp) = roadmap_project {
                                            ProjectName {
                                                name: project.name.clone(),
                                                color: rp.color.to_hex().to_string(),
                                            }
                                        } else {
                                            ProjectName {
                                                name: project.name.clone(),
                                                color: crate::models::ProjectColor::Blue.to_hex().to_string(),
                                            }
                                        }
                                    }
                                    TableCell {
                                        if let Some(rp) = roadmap_project {
                                            span { class: "text-secondary", "{rp.name}" }
                                        } else {
                                            span { class: "text-tertiary", "—" }
                                        }
                                    }
                                    TableCell {
                                        span { class: "monospace", "{estimated:.1} weeks" }
                                    }
                                    TableCell {
                                        span { class: "monospace", "{allocated:.1} weeks" }
                                    }
                                    TableCell {
                                        Badge {
                                            badge_type: status,
                                            "{status_text}"
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
