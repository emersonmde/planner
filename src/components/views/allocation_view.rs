use dioxus::prelude::*;
use std::collections::HashMap;

use crate::components::ui::{
    AssignProjectModal, ContextMenu, FloatingFab, FloatingProjectPanel, GridCell,
    KeybindingsOverlay, SplitAllocationModal,
};
use crate::models::{Allocation, Assignment};
use crate::state::use_plan_state;
use crate::utils::generate_quarter_weeks;

use super::grid_helpers::{calculate_cell_class, calculate_cell_variant};
use super::paintbrush::{allocate_project_to_cell, SelectedProject};

/// Allocation Grid view - displays the weekly allocation grid for engineers
/// Reference: docs/ui-design.md section 7.3
#[component]
pub fn AllocationView() -> Element {
    let mut plan = use_plan_state();
    let plan_data = plan();

    // Paintbrush mode state
    let mut paintbrush_active = use_signal(|| false);
    let mut selected_project = use_signal(|| SelectedProject::None);
    let mut panel_visible = use_signal(|| false);
    let mut panel_search_query = use_signal(String::new);
    let mut assign_mode = use_signal(|| false); // Differentiates paintbrush vs direct assign
    let mut is_dragging = use_signal(|| false);
    let mut drag_cells = use_signal(Vec::<(uuid::Uuid, chrono::NaiveDate)>::new);
    let mut error_cell = use_signal(|| None::<(uuid::Uuid, chrono::NaiveDate)>);
    let mut success_cells = use_signal(Vec::<(uuid::Uuid, chrono::NaiveDate)>::new);

    // Context menu state
    let mut context_menu_visible = use_signal(|| false);
    let mut context_menu_x = use_signal(|| 0);
    let mut context_menu_y = use_signal(|| 0);
    let mut context_menu_cell = use_signal(|| None::<(uuid::Uuid, chrono::NaiveDate)>);

    // Assign project modal state
    let mut assign_modal_visible = use_signal(|| false);
    let mut assign_project_id = use_signal(|| None::<uuid::Uuid>);

    // Split modal state
    let mut split_modal_visible = use_signal(|| false);
    let mut split_project1_id = use_signal(|| None::<uuid::Uuid>);
    let mut split_project2_id = use_signal(|| None::<uuid::Uuid>);
    let mut split_percentage = use_signal(|| 50.0);

    // Keybindings overlay state
    let mut keybindings_visible = use_signal(|| false);

    // Clipboard state and focused cell for hover-based copy/paste
    let mut clipboard = use_signal(|| None::<Vec<Assignment>>);
    let mut focused_cell = use_signal(|| None::<(uuid::Uuid, chrono::NaiveDate)>);

    // Generate weeks for the quarter using memo
    let weeks = use_memo(move || {
        let plan_data = plan();
        generate_quarter_weeks(
            plan_data.quarter_start_date,
            plan_data.weeks_in_quarter,
            plan_data.sprint_length_weeks,
        )
    });

    // Calculate grid columns dynamically (engineers as columns now)
    let grid_template_columns = use_memo(move || {
        let plan_data = plan();
        format!("120px repeat({}, 180px)", plan_data.team_members.len())
    });

    // Event handlers
    let handle_keydown = move |evt: KeyboardEvent| {
        // Escape - exit paintbrush mode, close panel, or close modals
        if evt.key() == Key::Escape {
            if panel_visible() {
                panel_visible.set(false);
            }
            if paintbrush_active() {
                paintbrush_active.set(false);
                selected_project.set(SelectedProject::None);
                is_dragging.set(false);
                drag_cells.set(Vec::new());
            }
            context_menu_visible.set(false);
            split_modal_visible.set(false);
            assign_modal_visible.set(false);
            keybindings_visible.set(false);
        }

        // Delete/Backspace - clear focused cell
        if matches!(evt.key(), Key::Delete | Key::Backspace) {
            if let Some((team_member_id, week_start)) = focused_cell() {
                // Clear allocation
                plan.with_mut(|p| {
                    p.allocations.retain(|a| {
                        !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                    });
                });
            }
        }

        // Cmd/Ctrl+C - copy assignment from focused cell
        if evt.key() == Key::Character("c".to_string())
            && (evt.modifiers().meta() || evt.modifiers().ctrl())
        {
            if let Some((team_member_id, week_start)) = focused_cell() {
                let plan_data = plan();
                let allocation_map: HashMap<(uuid::Uuid, chrono::NaiveDate), &Allocation> =
                    plan_data
                        .allocations
                        .iter()
                        .map(|a| ((a.team_member_id, a.week_start_date), a))
                        .collect();

                if let Some(alloc) = allocation_map.get(&(team_member_id, week_start)) {
                    clipboard.set(Some(alloc.assignments.clone()));
                }
            }
        }

        // Cmd/Ctrl+V - paste assignment to focused cell
        if evt.key() == Key::Character("v".to_string())
            && (evt.modifiers().meta() || evt.modifiers().ctrl())
        {
            if let Some((team_member_id, week_start)) = focused_cell() {
                if let Some(assignments) = clipboard() {
                    plan.with_mut(|p| {
                        // Remove existing allocation
                        p.allocations.retain(|a| {
                            !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                        });

                        // Add new allocation with copied assignments
                        let mut alloc = Allocation::new(team_member_id, week_start);
                        alloc.assignments = assignments;
                        p.allocations.push(alloc);
                    });
                }
            }
        }

        // ? - toggle keybindings overlay
        if evt.key() == Key::Character("?".to_string()) {
            keybindings_visible.set(!keybindings_visible());
        }
    };

    // Context menu action handler
    let handle_context_action = move |action: crate::components::ui::MenuAction| {
        use crate::components::ui::MenuAction;

        if let Some((team_member_id, week_start)) = context_menu_cell() {
            match action {
                MenuAction::AssignProject => {
                    // Open floating panel in assign mode
                    assign_mode.set(true);
                    panel_visible.set(true);
                    panel_search_query.set(String::new());
                    // Pre-fill selected project if cell has allocation
                    let plan_data = plan();
                    let allocation_map: std::collections::HashMap<
                        (uuid::Uuid, chrono::NaiveDate),
                        &crate::models::Allocation,
                    > = plan_data
                        .allocations
                        .iter()
                        .map(|a| ((a.team_member_id, a.week_start_date), a))
                        .collect();

                    if let Some(alloc) = allocation_map.get(&(team_member_id, week_start)) {
                        if let Some(assignment) = alloc.assignments.first() {
                            selected_project
                                .set(SelectedProject::Technical(assignment.technical_project_id));
                        } else {
                            selected_project.set(SelectedProject::None);
                        }
                    } else {
                        selected_project.set(SelectedProject::None);
                    }
                }
                MenuAction::SplitAllocation | MenuAction::EditSplit => {
                    // Open split modal
                    split_modal_visible.set(true);
                    // Pre-fill with current allocation if exists
                    let plan_data = plan();
                    let allocation_map: std::collections::HashMap<
                        (uuid::Uuid, chrono::NaiveDate),
                        &crate::models::Allocation,
                    > = plan_data
                        .allocations
                        .iter()
                        .map(|a| ((a.team_member_id, a.week_start_date), a))
                        .collect();

                    if let Some(alloc) = allocation_map.get(&(team_member_id, week_start)) {
                        if alloc.assignments.len() == 2 {
                            split_project1_id.set(Some(alloc.assignments[0].technical_project_id));
                            split_project2_id.set(Some(alloc.assignments[1].technical_project_id));
                            split_percentage.set(alloc.assignments[0].percentage);
                        } else {
                            split_project1_id.set(None);
                            split_project2_id.set(None);
                            split_percentage.set(50.0);
                        }
                    } else {
                        split_project1_id.set(None);
                        split_project2_id.set(None);
                        split_percentage.set(50.0);
                    }
                }
                MenuAction::ClearAssignment => {
                    // Clear allocation
                    plan.with_mut(|p| {
                        p.allocations.retain(|a| {
                            !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                        });
                    });
                }
            }
        }
    };

    // Assign project modal handlers
    let handle_assign_apply = move |_| {
        if let Some((team_member_id, week_start)) = context_menu_cell() {
            if let Some(proj_id) = assign_project_id() {
                plan.with_mut(|p| {
                    // Remove existing allocation
                    p.allocations.retain(|a| {
                        !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                    });

                    // Create new allocation with single project
                    let mut alloc = crate::models::Allocation::new(team_member_id, week_start);
                    alloc.assignments = vec![crate::models::Assignment::new(proj_id, 100.0)];
                    p.allocations.push(alloc);
                });
            }

            // Close modal and reset
            assign_modal_visible.set(false);
            assign_project_id.set(None);
        }
    };

    let handle_assign_cancel = move |_| {
        assign_modal_visible.set(false);
        assign_project_id.set(None);
    };

    // Split modal handlers
    let handle_split_apply = move |_| {
        if let Some((team_member_id, week_start)) = context_menu_cell() {
            if let (Some(proj1_id), Some(proj2_id)) = (split_project1_id(), split_project2_id()) {
                if proj1_id != proj2_id {
                    plan.with_mut(|p| {
                        // Remove existing allocation
                        p.allocations.retain(|a| {
                            !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                        });

                        // Create split allocation
                        let mut alloc = crate::models::Allocation::new(team_member_id, week_start);
                        alloc.assignments = vec![
                            crate::models::Assignment::new(proj1_id, split_percentage()),
                            crate::models::Assignment::new(proj2_id, 100.0 - split_percentage()),
                        ];
                        p.allocations.push(alloc);
                    });

                    // Close modal and reset
                    split_modal_visible.set(false);
                    split_project1_id.set(None);
                    split_project2_id.set(None);
                    split_percentage.set(50.0);
                }
            }
        }
    };

    let handle_split_cancel = move |_| {
        split_modal_visible.set(false);
        split_project1_id.set(None);
        split_project2_id.set(None);
        split_percentage.set(50.0);
    };

    // FAB click handler - toggle panel visibility and paintbrush mode (coupled)
    let handle_fab_click = move |_| {
        let new_state = !panel_visible();
        panel_visible.set(new_state);
        assign_mode.set(false); // FAB click is always paintbrush mode
        panel_search_query.set(String::new());

        if new_state {
            // Opening panel - activate paintbrush mode
            paintbrush_active.set(true);
        } else {
            // Closing panel - deactivate paintbrush mode
            paintbrush_active.set(false);
            selected_project.set(SelectedProject::None);
            is_dragging.set(false);
            drag_cells.set(Vec::new());
        }
    };

    // Panel project selection handler
    let handle_panel_project_select = move |project_id: uuid::Uuid| {
        selected_project.set(SelectedProject::Technical(project_id));
        if assign_mode() {
            // Direct assign mode - assign to cell and close panel
            if let Some((team_member_id, week_start)) = context_menu_cell() {
                plan.with_mut(|p| {
                    // Remove existing allocation
                    p.allocations.retain(|a| {
                        !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                    });

                    // Create new allocation
                    let mut alloc = crate::models::Allocation::new(team_member_id, week_start);
                    alloc.assignments = vec![crate::models::Assignment::new(project_id, 100.0)];
                    p.allocations.push(alloc);
                });
            }
            panel_visible.set(false);
            assign_mode.set(false);
            selected_project.set(SelectedProject::None);
        } else {
            // Paintbrush mode - activate paintbrush
            paintbrush_active.set(true);
        }
    };

    // Panel none/clear selection handler
    let handle_panel_none_select = move |_| {
        selected_project.set(SelectedProject::None);
        if assign_mode() {
            // Direct assign mode - clear cell and close panel
            if let Some((team_member_id, week_start)) = context_menu_cell() {
                plan.with_mut(|p| {
                    // Remove existing allocation
                    p.allocations.retain(|a| {
                        !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                    });
                });
            }
            panel_visible.set(false);
            assign_mode.set(false);
        } else {
            // Paintbrush mode - activate paintbrush for clearing
            paintbrush_active.set(true);
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
            // Always set focused cell for hover-based copy/paste
            focused_cell.set(Some((team_member_id, week_start)));

            // Handle paintbrush drag
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

    // Get selected project color for FAB
    let fab_project_color = match selected_project() {
        SelectedProject::Technical(id) => plan_data
            .technical_projects
            .iter()
            .find(|p| p.id == id)
            .and_then(|p| p.roadmap_project_id)
            .and_then(|rp_id| plan_data.get_roadmap_project(&rp_id))
            .map(|rp| rp.color.to_hex().to_string()),
        _ => None,
    };

    // Get selected project ID and mode for panel
    let panel_selected_id = match selected_project() {
        SelectedProject::Technical(id) => Some(id),
        _ => None,
    };
    let is_none_selected = matches!(selected_project(), SelectedProject::None);

    rsx! {
        div {
            class: if paintbrush_active() { "view active allocation-grid-view paintbrush-active" } else { "view active allocation-grid-view" },
            tabindex: 0,
            onkeydown: handle_keydown,
            onmouseup: handle_mouseup,
            onmouseleave: move |_| {
                // Reset drag state when cursor leaves window to prevent ghost allocations
                if is_dragging() {
                    is_dragging.set(false);
                    drag_cells.set(Vec::new());
                }
            },

            // Grid container
            div { class: "allocation-grid-container",
                div {
                    class: "allocation-grid",
                    style: "grid-template-columns: {grid_template_columns()};",

                    // Top-left corner cell
                    div { class: "grid-header-corner" }

                    // Column headers (engineers)
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
                            let header_class = if capacity_status == "error" {
                                "grid-engineer-header over-allocated"
                            } else {
                                "grid-engineer-header"
                            };

                            rsx! {
                                div { class: "{header_class}",
                                    div { class: "engineer-name-row",
                                        span { class: "engineer-name", "{engineer.name}" }
                                        span { class: "role-badge", "{engineer.role.short_name()}" }
                                    }
                                    div { class: "capacity-row",
                                        span {
                                            class: "capacity-text capacity-{capacity_status}",
                                            "{allocated:.1} / {capacity} w"
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
                        }
                    }

                    // Week rows
                    for i in 0..weeks().len() {
                        {
                            let weeks_list = weeks();
                            let week = &weeks_list[i];
                            let week_start_date = week.start_date;
                            let is_sprint_start = week.is_sprint_start();
                            let row_header_class = if is_sprint_start {
                                "grid-week-row-header sprint-separator"
                            } else {
                                "grid-week-row-header"
                            };

                            rsx! {
                                // Row header (week info)
                                div { class: "{row_header_class}",
                                    div { class: "week-date", "{week.format_date(true)}" }
                                    div { class: "week-progress", "{week.format_week_number()}" }
                                    if is_sprint_start {
                                        div { class: "sprint-label", "Sprint {week.sprint_number}" }
                                    }
                                }

                                // Grid cells (iterate over engineers)
                                for engineer in &plan_data.team_members {
                                    {
                                        let engineer_id = engineer.id;

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
                                        let cell_class_with_separator = if is_sprint_start {
                                            format!("{} sprint-separator", cell_class)
                                        } else {
                                            cell_class.to_string()
                                        };

                                        rsx! {
                                            div {
                                                class: "{cell_class_with_separator}",
                                                onmousedown: move |_| {
                                                    handle_cell_mousedown(engineer_id, week_start_date)
                                                },
                                                onmouseenter: move |_| {
                                                    handle_cell_mouseenter(engineer_id, week_start_date)
                                                },
                                                oncontextmenu: move |e| {
                                                    e.prevent_default();
                                                    context_menu_visible.set(true);
                                                    let coords = e.client_coordinates();
                                                    context_menu_x.set(coords.x as i32);
                                                    context_menu_y.set(coords.y as i32);
                                                    context_menu_cell.set(Some((engineer_id, week_start_date)));
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

            // Context Menu
            ContextMenu {
                x: context_menu_x(),
                y: context_menu_y(),
                visible: context_menu_visible(),
                has_allocation: {
                    if let Some((team_member_id, week_start)) = context_menu_cell() {
                        allocation_map.contains_key(&(team_member_id, week_start))
                    } else {
                        false
                    }
                },
                is_split: {
                    if let Some((team_member_id, week_start)) = context_menu_cell() {
                        allocation_map.get(&(team_member_id, week_start))
                            .map(|alloc| alloc.assignments.len() == 2)
                            .unwrap_or(false)
                    } else {
                        false
                    }
                },
                on_action: handle_context_action,
                on_close: move |_| context_menu_visible.set(false),
            }

            // Assign Project Modal
            AssignProjectModal {
                visible: assign_modal_visible(),
                selected_project_id: assign_project_id(),
                on_project_select: move |id| assign_project_id.set(Some(id)),
                on_apply: handle_assign_apply,
                on_cancel: handle_assign_cancel,
            }

            // Split Allocation Modal
            SplitAllocationModal {
                visible: split_modal_visible(),
                project1_id: split_project1_id(),
                project1_percentage: split_percentage(),
                project2_id: split_project2_id(),
                on_project1_select: move |id| split_project1_id.set(Some(id)),
                on_project2_select: move |id| split_project2_id.set(Some(id)),
                on_percentage_change: move |pct| split_percentage.set(pct),
                on_apply: handle_split_apply,
                on_cancel: handle_split_cancel,
            }

            // Floating Action Button
            FloatingFab {
                active: paintbrush_active(),
                project_color: fab_project_color,
                on_click: handle_fab_click,
            }

            // Floating Project Panel
            FloatingProjectPanel {
                visible: panel_visible(),
                selected_project_id: panel_selected_id,
                is_none_selected,
                search_query: panel_search_query(),
                on_search_change: move |q| panel_search_query.set(q),
                on_project_select: handle_panel_project_select,
                on_none_select: handle_panel_none_select,
                on_close: move |_| {
                    panel_visible.set(false);
                    panel_search_query.set(String::new());
                    // Close panel also deactivates paintbrush (coupled behavior)
                    paintbrush_active.set(false);
                    selected_project.set(SelectedProject::None);
                    is_dragging.set(false);
                    drag_cells.set(Vec::new());
                },
            }

            // Keybindings Overlay
            KeybindingsOverlay {
                visible: keybindings_visible(),
                on_close: move |_| keybindings_visible.set(false),
            }
        }
    }
}
