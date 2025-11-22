/// Paintbrush mode logic for interactive allocation editing
use dioxus::prelude::{Signal, WritableExt};
use uuid::Uuid;

use crate::models::{Allocation, Assignment, Plan};

/// Tracks which project is selected for paintbrush mode
#[derive(Clone, PartialEq, Debug)]
pub enum SelectedProject {
    None,
    Technical(Uuid),
}

/// Allocates a project to a specific cell
/// Returns true if successful, false if validation failed (e.g., project doesn't exist)
pub fn allocate_project_to_cell(
    plan: &mut Signal<Plan>,
    selected_project: &SelectedProject,
    team_member_id: Uuid,
    week_start: chrono::NaiveDate,
) -> bool {
    match selected_project {
        SelectedProject::None => {
            // Clear allocation - remove existing allocation for this cell
            plan.with_mut(|p| {
                p.allocations.retain(|a| {
                    !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                });
            });
            true
        }
        SelectedProject::Technical(project_id) => {
            // Validate that the project exists before creating allocation
            let project_exists = plan().get_technical_project(project_id).is_some();
            if !project_exists {
                return false;
            }

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
