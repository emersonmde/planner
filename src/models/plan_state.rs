use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Allocation, RoadmapProject, TechnicalProject};

/// Plan metadata for versioning and audit trail
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanMetadata {
    /// File format version (e.g., "1.0")
    pub version: String,

    /// When this plan was first created
    pub created_at: DateTime<Utc>,

    /// When this plan was last modified
    pub modified_at: DateTime<Utc>,
}

impl PlanMetadata {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            version: "1.0".to_string(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Mark the plan as modified (updates modified_at timestamp)
    pub fn mark_modified(&mut self) {
        self.modified_at = Utc::now();
    }
}

impl Default for PlanMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Planning state for a single quarter
/// Exported/imported per quarter, changes frequently during planning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanState {
    /// Quarter name (e.g., "Q1 2025")
    pub quarter_name: String,

    /// Quarter start date (first Monday)
    pub quarter_start_date: NaiveDate,

    /// Number of weeks in the quarter (typically 13)
    pub num_weeks: usize,

    /// All roadmap projects for this quarter
    pub roadmap_projects: Vec<RoadmapProject>,

    /// All technical projects for this quarter
    pub technical_projects: Vec<TechnicalProject>,

    /// All allocations for this quarter
    pub allocations: Vec<Allocation>,

    /// Plan metadata (version, timestamps)
    pub metadata: PlanMetadata,
}

#[allow(dead_code)] // Some methods used in future milestones (M10+)
impl PlanState {
    pub fn new(quarter_name: String, quarter_start_date: NaiveDate, num_weeks: usize) -> Self {
        Self {
            quarter_name,
            quarter_start_date,
            num_weeks,
            roadmap_projects: Vec::new(),
            technical_projects: Vec::new(),
            allocations: Vec::new(),
            metadata: PlanMetadata::new(),
        }
    }

    /// Get roadmap project by ID
    pub fn get_roadmap_project(&self, id: &Uuid) -> Option<&RoadmapProject> {
        self.roadmap_projects.iter().find(|p| &p.id == id)
    }

    /// Get mutable roadmap project by ID
    pub fn get_roadmap_project_mut(&mut self, id: &Uuid) -> Option<&mut RoadmapProject> {
        self.roadmap_projects.iter_mut().find(|p| &p.id == id)
    }

    /// Get technical project by ID
    pub fn get_technical_project(&self, id: &Uuid) -> Option<&TechnicalProject> {
        self.technical_projects.iter().find(|p| &p.id == id)
    }

    /// Get mutable technical project by ID
    pub fn get_technical_project_mut(&mut self, id: &Uuid) -> Option<&mut TechnicalProject> {
        self.technical_projects.iter_mut().find(|p| &p.id == id)
    }

    /// Calculate total allocated weeks for a technical project
    pub fn calculate_project_allocated_weeks(&self, technical_project_id: &Uuid) -> f32 {
        self.allocations
            .iter()
            .flat_map(|alloc| &alloc.assignments)
            .filter(|assignment| &assignment.technical_project_id == technical_project_id)
            .map(|assignment| assignment.percentage / 100.0)
            .sum()
    }

    /// Calculate allocated weeks by role for a technical project
    /// Returns (eng_allocated, sci_allocated, total_allocated)
    pub fn calculate_technical_project_allocated_by_role(
        &self,
        technical_project_id: &Uuid,
        get_member_role: impl Fn(&Uuid) -> Option<super::Role>,
    ) -> (f32, f32, f32) {
        use super::Role;
        let mut eng_allocated = 0.0;
        let mut sci_allocated = 0.0;

        for allocation in &self.allocations {
            if let Some(role) = get_member_role(&allocation.team_member_id) {
                for assignment in &allocation.assignments {
                    if &assignment.technical_project_id == technical_project_id {
                        let weeks = assignment.percentage / 100.0;
                        match role {
                            Role::Engineering => eng_allocated += weeks,
                            Role::Science => sci_allocated += weeks,
                        }
                    }
                }
            }
        }

        let total_allocated = eng_allocated + sci_allocated;
        (eng_allocated, sci_allocated, total_allocated)
    }

    /// Get unique team member IDs assigned to a technical project
    pub fn get_assigned_team_members(&self, technical_project_id: &Uuid) -> Vec<Uuid> {
        let mut member_ids: Vec<Uuid> = self
            .allocations
            .iter()
            .filter(|alloc| {
                alloc
                    .assignments
                    .iter()
                    .any(|a| &a.technical_project_id == technical_project_id)
            })
            .map(|alloc| alloc.team_member_id)
            .collect();

        // Remove duplicates
        member_ids.sort();
        member_ids.dedup();
        member_ids
    }

    /// Get the date range for a technical project based on its allocations
    /// Returns (first_allocation_week, last_allocation_week) or None if no allocations
    pub fn get_project_allocation_date_range(
        &self,
        technical_project_id: &Uuid,
    ) -> Option<(chrono::NaiveDate, chrono::NaiveDate)> {
        let mut allocation_weeks: Vec<chrono::NaiveDate> = self
            .allocations
            .iter()
            .filter(|alloc| {
                alloc
                    .assignments
                    .iter()
                    .any(|a| &a.technical_project_id == technical_project_id)
            })
            .map(|alloc| alloc.week_start_date)
            .collect();

        if allocation_weeks.is_empty() {
            return None;
        }

        allocation_weeks.sort();
        let first_week = allocation_weeks[0];
        let last_week = allocation_weeks[allocation_weeks.len() - 1];
        Some((first_week, last_week))
    }

    /// Calculate total allocated weeks for a team member
    pub fn calculate_team_member_allocated_weeks(&self, team_member_id: &Uuid) -> f32 {
        self.allocations
            .iter()
            .filter(|alloc| &alloc.team_member_id == team_member_id)
            .map(|alloc| alloc.total_percentage() / 100.0)
            .sum()
    }

    /// Get unique project names assigned to a team member
    pub fn get_assigned_project_names_for_member(&self, team_member_id: &Uuid) -> Vec<String> {
        use std::collections::HashSet;

        self.allocations
            .iter()
            .filter(|a| &a.team_member_id == team_member_id)
            .flat_map(|a| &a.assignments)
            .filter_map(|assignment| {
                self.technical_projects
                    .iter()
                    .find(|p| p.id == assignment.technical_project_id)
                    .map(|p| p.name.clone())
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Calculate allocated weeks for a roadmap project (sum of all linked technical projects)
    /// Returns (eng_allocated, sci_allocated, total_allocated)
    /// Note: Requires team member data from Preferences to determine role
    pub fn calculate_roadmap_allocated_weeks(
        &self,
        roadmap_project_id: &Uuid,
        get_member_role: impl Fn(&Uuid) -> Option<super::Role>,
    ) -> (f32, f32, f32) {
        use super::Role;
        let mut eng_allocated = 0.0;
        let mut sci_allocated = 0.0;

        // Find all technical projects linked to this roadmap project
        let linked_tech_projects: Vec<&TechnicalProject> = self
            .technical_projects
            .iter()
            .filter(|tp| tp.roadmap_project_id.as_ref() == Some(roadmap_project_id))
            .collect();

        // For each linked technical project, sum up allocations by engineer role
        for tech_project in linked_tech_projects {
            for allocation in &self.allocations {
                if let Some(role) = get_member_role(&allocation.team_member_id) {
                    for assignment in &allocation.assignments {
                        if assignment.technical_project_id == tech_project.id {
                            let weeks = assignment.percentage / 100.0;
                            match role {
                                Role::Engineering => eng_allocated += weeks,
                                Role::Science => sci_allocated += weeks,
                            }
                        }
                    }
                }
            }
        }

        let total_allocated = eng_allocated + sci_allocated;
        (eng_allocated, sci_allocated, total_allocated)
    }

    /// Update a technical project's start_date and expected_completion based on its allocations
    ///
    /// - start_date: Set to the start of the sprint containing the first allocation
    /// - expected_completion: Set to the end of the sprint containing the last allocation
    ///
    /// Call this after adding, removing, or modifying allocations for a project.
    pub fn update_technical_project_dates(
        &mut self,
        technical_project_id: &Uuid,
        sprint_anchor_date: NaiveDate,
        sprint_length_weeks: usize,
    ) {
        use crate::utils::date_helpers::get_sprint_boundaries;

        // Find all weeks with allocations for this project
        let mut allocation_weeks: Vec<NaiveDate> = self
            .allocations
            .iter()
            .filter(|alloc| {
                alloc
                    .assignments
                    .iter()
                    .any(|a| a.technical_project_id == *technical_project_id)
            })
            .map(|alloc| alloc.week_start_date)
            .collect();

        // If no allocations, leave dates unchanged
        if allocation_weeks.is_empty() {
            return;
        }

        // Sort to find first and last weeks
        allocation_weeks.sort();
        let first_week = allocation_weeks[0];
        let last_week = allocation_weeks[allocation_weeks.len() - 1];

        // Calculate sprint boundaries
        let (first_sprint_start, _) =
            get_sprint_boundaries(first_week, sprint_anchor_date, sprint_length_weeks);
        let (_, last_sprint_end) =
            get_sprint_boundaries(last_week, sprint_anchor_date, sprint_length_weeks);

        // Update the technical project
        if let Some(project) = self.get_technical_project_mut(technical_project_id) {
            project.start_date = first_sprint_start;
            project.expected_completion = Some(last_sprint_end);
            self.metadata.mark_modified();
        }
    }

    /// Mark the plan as modified
    pub fn mark_modified(&mut self) {
        self.metadata.mark_modified();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_state_creation() {
        let quarter_start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let state = PlanState::new("Q1 2025".to_string(), quarter_start, 13);

        assert_eq!(state.quarter_name, "Q1 2025");
        assert_eq!(state.quarter_start_date, quarter_start);
        assert_eq!(state.num_weeks, 13);
        assert_eq!(state.roadmap_projects.len(), 0);
        assert_eq!(state.technical_projects.len(), 0);
        assert_eq!(state.allocations.len(), 0);
        assert_eq!(state.metadata.version, "1.0");
    }

    #[test]
    fn test_metadata_mark_modified() {
        let mut metadata = PlanMetadata::new();
        let created = metadata.created_at;
        let modified = metadata.modified_at;

        assert_eq!(created, modified);

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));
        metadata.mark_modified();

        assert_eq!(metadata.created_at, created);
        assert!(metadata.modified_at > modified);
    }
}
