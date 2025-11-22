/// Helper functions for grid rendering and cell variant calculation
use chrono::NaiveDate;

use crate::components::ui::GridCellVariant;
use crate::models::{Allocation, Plan, ProjectColor};

/// Calculate the appropriate GridCellVariant for a given allocation
pub fn calculate_cell_variant(
    allocation: Option<&Allocation>,
    plan: &Plan,
    week_start_date: NaiveDate,
) -> GridCellVariant {
    let Some(alloc) = allocation else {
        return GridCellVariant::Empty;
    };

    // Oncall assignment
    if alloc.is_oncall() {
        return GridCellVariant::Oncall;
    }

    // Split allocation (2 projects)
    if alloc.assignments.len() == 2 {
        return create_split_variant(alloc, plan);
    }

    // Single project allocation
    if let Some(assignment) = alloc.assignments.first() {
        return create_single_week_variant(assignment, plan, week_start_date);
    }

    GridCellVariant::Empty
}

/// Create a split allocation cell variant
fn create_split_variant(alloc: &Allocation, plan: &Plan) -> GridCellVariant {
    let assignment1 = &alloc.assignments[0];
    let assignment2 = &alloc.assignments[1];

    let project1 = plan.get_technical_project(&assignment1.technical_project_id);
    let project2 = plan.get_technical_project(&assignment2.technical_project_id);

    match (project1, project2) {
        (Some(p1), Some(p2)) => {
            let rp1 = p1
                .roadmap_project_id
                .and_then(|id| plan.get_roadmap_project(&id));
            let rp2 = p2
                .roadmap_project_id
                .and_then(|id| plan.get_roadmap_project(&id));

            GridCellVariant::Split {
                project1_name: p1.name.clone(),
                project1_color: rp1.map(|rp| rp.color).unwrap_or(ProjectColor::Blue),
                project1_percentage: assignment1.percentage,
                project2_name: p2.name.clone(),
                project2_color: rp2.map(|rp| rp.color).unwrap_or(ProjectColor::Green),
                project2_percentage: assignment2.percentage,
            }
        }
        _ => GridCellVariant::Empty,
    }
}

/// Create a single week allocation cell variant
fn create_single_week_variant(
    assignment: &crate::models::Assignment,
    plan: &Plan,
    week_start_date: NaiveDate,
) -> GridCellVariant {
    let Some(proj) = plan.get_technical_project(&assignment.technical_project_id) else {
        return GridCellVariant::Empty;
    };

    let roadmap_project = proj
        .roadmap_project_id
        .and_then(|id| plan.get_roadmap_project(&id));
    let project_color = roadmap_project
        .map(|rp| rp.color)
        .unwrap_or(ProjectColor::Blue);
    let is_before_start = week_start_date < proj.start_date;

    GridCellVariant::SingleWeek {
        project_name: proj.name.clone(),
        project_color,
        percentage: assignment.percentage,
        is_before_start,
    }
}

/// Calculate cell CSS class based on state
pub fn calculate_cell_class(
    is_error: bool,
    is_success: bool,
    is_drag_target: bool,
) -> &'static str {
    if is_error {
        "grid-cell-error"
    } else if is_success {
        "grid-cell-success"
    } else if is_drag_target {
        "grid-cell-drag-target"
    } else {
        ""
    }
}
