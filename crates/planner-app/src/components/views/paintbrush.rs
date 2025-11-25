/// Paintbrush mode logic for interactive allocation editing
use chrono::NaiveDate;
use dioxus::prelude::{Signal, WritableExt};
use uuid::Uuid;

use planner_core::models::{Allocation, Assignment, PlanState};

/// Tracks which project is selected for paintbrush mode
#[derive(Clone, PartialEq, Debug)]
pub enum SelectedProject {
    None,
    Technical(Uuid),
}

/// Allocates a project to a specific cell
/// Returns true if successful, false if validation failed (e.g., project doesn't exist)
pub fn allocate_project_to_cell(
    plan_state: &mut Signal<PlanState>,
    selected_project: &SelectedProject,
    team_member_id: Uuid,
    week_start: NaiveDate,
    sprint_anchor: NaiveDate,
    sprint_length: usize,
) -> bool {
    match selected_project {
        SelectedProject::None => {
            // Clear allocation - remove existing allocation for this cell
            plan_state.with_mut(|p| {
                // Find which project(s) were allocated to update their dates
                let affected_projects: Vec<Uuid> = p
                    .allocations
                    .iter()
                    .filter(|a| {
                        a.team_member_id == team_member_id && a.week_start_date == week_start
                    })
                    .flat_map(|a| &a.assignments)
                    .map(|assignment| assignment.technical_project_id)
                    .collect();

                // Remove the allocation
                p.allocations.retain(|a| {
                    !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                });

                // Update dates for all affected projects
                for project_id in affected_projects {
                    p.update_technical_project_dates(&project_id, sprint_anchor, sprint_length);
                }
            });
            true
        }
        SelectedProject::Technical(project_id) => {
            // Validate that the project exists before creating allocation
            let project_exists = plan_state().get_technical_project(project_id).is_some();
            if !project_exists {
                return false;
            }

            plan_state.with_mut(|p| {
                // Find which project(s) were previously allocated to update their dates
                let previous_projects: Vec<Uuid> = p
                    .allocations
                    .iter()
                    .filter(|a| {
                        a.team_member_id == team_member_id && a.week_start_date == week_start
                    })
                    .flat_map(|a| &a.assignments)
                    .map(|assignment| assignment.technical_project_id)
                    .collect();

                // Remove existing allocation if any
                p.allocations.retain(|a| {
                    !(a.team_member_id == team_member_id && a.week_start_date == week_start)
                });

                // Add new allocation
                let mut alloc = Allocation::new(team_member_id, week_start);
                alloc.assignments.push(Assignment::new(*project_id, 100.0));
                p.allocations.push(alloc);

                // Update dates for the newly assigned project
                p.update_technical_project_dates(project_id, sprint_anchor, sprint_length);

                // Update dates for previously assigned projects (in case they were replaced)
                for prev_project_id in previous_projects {
                    if prev_project_id != *project_id {
                        p.update_technical_project_dates(
                            &prev_project_id,
                            sprint_anchor,
                            sprint_length,
                        );
                    }
                }
            });
            true
        }
    }
}
