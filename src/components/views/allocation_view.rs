use dioxus::prelude::*;
use std::collections::HashMap;

use crate::components::ui::GridCell;
use crate::state::use_plan_state;
use crate::utils::generate_quarter_weeks;

use super::grid_helpers::{calculate_cell_class, calculate_cell_variant};
use super::paintbrush::{
    allocate_project_to_cell, PaintbrushControls, ProjectSelector, SelectedProject,
};

/// Allocation Grid view - displays the weekly allocation grid for engineers
/// Reference: docs/ui-design.md section 7.3
#[component]
pub fn AllocationView() -> Element {
    let mut plan = use_plan_state();
    let plan_data = plan();

    // Paintbrush mode state
    let mut paintbrush_active = use_signal(|| false);
    let mut selected_project = use_signal(|| SelectedProject::None);
    let mut search_query = use_signal(String::new);
    let mut is_dragging = use_signal(|| false);
    let mut drag_cells = use_signal(Vec::<(uuid::Uuid, chrono::NaiveDate)>::new);
    let mut error_cell = use_signal(|| None::<(uuid::Uuid, chrono::NaiveDate)>);
    let mut success_cells = use_signal(Vec::<(uuid::Uuid, chrono::NaiveDate)>::new);

    // Generate weeks for the quarter using memo
    let weeks = use_memo(move || {
        let plan_data = plan();
        generate_quarter_weeks(
            plan_data.quarter_start_date,
            plan_data.weeks_in_quarter,
            plan_data.sprint_length_weeks,
        )
    });

    // Calculate grid columns dynamically
    let grid_template_columns = use_memo(move || format!("180px repeat({}, 150px)", weeks().len()));

    // Event handlers
    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Escape && paintbrush_active() {
            paintbrush_active.set(false);
            selected_project.set(SelectedProject::None);
            is_dragging.set(false);
            drag_cells.set(Vec::new());
        }
    };

    let toggle_paintbrush = move |_| {
        let new_state = !paintbrush_active();
        paintbrush_active.set(new_state);
        if !new_state {
            selected_project.set(SelectedProject::None);
            is_dragging.set(false);
            drag_cells.set(Vec::new());
        }
    };

    let handle_mouseup = move |_: MouseEvent| {
        if is_dragging() {
            is_dragging.set(false);
            drag_cells.set(Vec::new());
        }
    };

    // Allocation handler - uses extracted logic
    let mut allocate_to_cell = move |team_member_id: uuid::Uuid, week_start: chrono::NaiveDate| {
        if !paintbrush_active() {
            return;
        }

        let success =
            allocate_project_to_cell(&mut plan, &selected_project(), team_member_id, week_start);

        if success {
            error_cell.set(None);
            success_cells.with_mut(|cells| {
                cells.clear();
                cells.push((team_member_id, week_start));
            });
        } else {
            error_cell.set(Some((team_member_id, week_start)));
        }
    };

    // Cell event handlers
    let mut handle_cell_click = move |team_member_id: uuid::Uuid, week_start: chrono::NaiveDate| {
        if paintbrush_active() {
            allocate_to_cell(team_member_id, week_start);
        }
    };

    let mut handle_cell_mousedown =
        move |team_member_id: uuid::Uuid, week_start: chrono::NaiveDate| {
            if paintbrush_active() {
                is_dragging.set(true);
                drag_cells.set(vec![(team_member_id, week_start)]);
                allocate_to_cell(team_member_id, week_start);
            }
        };

    let mut handle_cell_mouseenter =
        move |team_member_id: uuid::Uuid, week_start: chrono::NaiveDate| {
            if paintbrush_active() && is_dragging() {
                drag_cells.with_mut(|cells| {
                    if !cells.contains(&(team_member_id, week_start)) {
                        cells.push((team_member_id, week_start));
                    }
                });
                allocate_to_cell(team_member_id, week_start);
            }
        };

    // Pre-compute allocation map for O(1) lookups
    let allocation_map: HashMap<(uuid::Uuid, chrono::NaiveDate), &crate::models::Allocation> =
        plan_data
            .allocations
            .iter()
            .map(|a| ((a.team_member_id, a.week_start_date), a))
            .collect();

    rsx! {
        div {
            class: if paintbrush_active() { "view active allocation-grid-view paintbrush-active" } else { "view active allocation-grid-view" },
            tabindex: 0,
            onkeydown: handle_keydown,
            onmouseup: handle_mouseup,

            // Paintbrush mode controls (extracted component)
            PaintbrushControls {
                active: paintbrush_active(),
                selected_project: selected_project(),
                on_toggle: toggle_paintbrush,
                ProjectSelector {
                    search_query: search_query(),
                    selected_project: selected_project(),
                    on_search_change: move |q| search_query.set(q),
                    on_project_select: move |p| selected_project.set(p),
                }
            }

            // Grid container
            div { class: "allocation-grid-container",
                div {
                    class: "allocation-grid",
                    style: "grid-template-columns: {grid_template_columns()};",

                    // Top-left corner cell
                    div { class: "grid-header-corner" }

                    // Column headers (weeks)
                    for i in 0..weeks().len() {
                        {
                            let weeks_list = weeks();
                            let week = &weeks_list[i];
                            let is_sprint_start = week.is_sprint_start();
                            let sprint_class = if is_sprint_start {
                                "grid-week-header sprint-separator"
                            } else {
                                "grid-week-header"
                            };

                            rsx! {
                                div { class: "{sprint_class}",
                                    div { class: "sprint-label",
                                        if is_sprint_start {
                                            "Sprint {week.sprint_number}"
                                        }
                                    }
                                    div { class: "week-date", "{week.format_date(true)}" }
                                    div { class: "week-progress", "{week.format_week_number()}" }
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
                                // Row header
                                div { class: "grid-row-header",
                                    div { class: "engineer-info",
                                        div { class: "engineer-name-row",
                                            span { class: "engineer-name", "{engineer.name}" }
                                            span { class: "role-badge", "{engineer.role.short_name()}" }
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

                                // Grid cells
                                for i in 0..weeks().len() {
                                    {
                                        let weeks_list = weeks();
                                        let week = &weeks_list[i];
                                        let week_start_date = week.start_date;

                                        // Look up allocation (O(1))
                                        let allocation = allocation_map
                                            .get(&(engineer_id, week_start_date))
                                            .copied();

                                        // Calculate cell variant using helper
                                        let variant = calculate_cell_variant(
                                            allocation,
                                            &plan_data,
                                            week_start_date
                                        );

                                        // Calculate cell state
                                        let is_error = error_cell() == Some((engineer_id, week_start_date));
                                        let is_drag_target = is_dragging() &&
                                            drag_cells().contains(&(engineer_id, week_start_date));
                                        let is_success = success_cells()
                                            .contains(&(engineer_id, week_start_date));
                                        let cell_class = calculate_cell_class(
                                            is_error,
                                            is_success,
                                            is_drag_target
                                        );

                                        rsx! {
                                            div {
                                                class: "{cell_class}",
                                                onmousedown: move |_| {
                                                    handle_cell_mousedown(engineer_id, week_start_date)
                                                },
                                                onmouseenter: move |_| {
                                                    handle_cell_mouseenter(engineer_id, week_start_date)
                                                },
                                                GridCell {
                                                    variant,
                                                    onclick: move |_| {
                                                        handle_cell_click(engineer_id, week_start_date)
                                                    },
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
    }
}
