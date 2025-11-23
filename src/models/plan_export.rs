use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{
    Allocation, PlanMetadata, PlanState, Preferences, RoadmapProject, TeamMember, TechnicalProject,
};

/// Self-contained plan export format
///
/// Combines Preferences + PlanState into a portable package.
/// Includes team snapshot (name + full roster) for portability and future aggregation.
///
/// Use cases:
/// - 1.0: Share with colleagues, archive past quarters, version control
/// - 2.0+: Multi-team aggregation (Sr Manager loads multiple team plans)
#[allow(dead_code)] // Used in M13 for plan import/export
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanExport {
    /// File format version (e.g., "1.0")
    pub version: String,

    /// Plan metadata (timestamps)
    pub metadata: PlanMetadata,

    // ========== TEAM CONTEXT (snapshot at export time) ==========
    /// Team name (e.g., "Backend Team", "Data Science")
    /// Used for multi-team aggregation in v2.0
    pub team_name: String,

    /// Full team roster (snapshot)
    /// Includes names, roles, capacity for portability
    /// Allows importing user to see all context without requiring local team data
    pub team_members: Vec<TeamMember>,

    // ========== PLANNING DATA ==========
    /// Quarter name (e.g., "Q1 2025")
    pub quarter_name: String,

    /// Quarter start date (first Monday)
    pub quarter_start_date: NaiveDate,

    /// Number of weeks in the quarter (typically 13)
    pub num_weeks: usize,

    /// All roadmap projects
    pub roadmap_projects: Vec<RoadmapProject>,

    /// All technical projects
    pub technical_projects: Vec<TechnicalProject>,

    /// All allocations
    pub allocations: Vec<Allocation>,
}

#[allow(dead_code)] // Methods used in M13 for plan import/export
impl PlanExport {
    /// Create a self-contained export from Preferences and PlanState signals
    ///
    /// Takes snapshots of both signals to create a portable, self-contained export
    /// that can be shared with other users or loaded for multi-team aggregation.
    pub fn from_signals(prefs: Preferences, state: PlanState) -> Self {
        Self {
            version: state.metadata.version.clone(),
            metadata: state.metadata.clone(),

            // Team context snapshot
            team_name: prefs.team_name,
            team_members: prefs.team_members,

            // Planning data
            quarter_name: state.quarter_name,
            quarter_start_date: state.quarter_start_date,
            num_weeks: state.num_weeks,
            roadmap_projects: state.roadmap_projects,
            technical_projects: state.technical_projects,
            allocations: state.allocations,
        }
    }

    /// Split export into Preferences and PlanState for import
    ///
    /// When importing, you can choose to:
    /// - Merge team members into local preferences
    /// - Use exported team context as read-only view
    /// - Replace local preferences entirely
    pub fn into_signals(self) -> (Preferences, PlanState) {
        let prefs = Preferences {
            team_name: self.team_name,
            team_members: self.team_members,
            // Use defaults for sprint config (user should configure these locally)
            sprint_anchor_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                .expect("Valid sprint anchor"),
            sprint_length_weeks: 2,
            default_capacity: 12.0,
        };

        let state = PlanState {
            quarter_name: self.quarter_name,
            quarter_start_date: self.quarter_start_date,
            num_weeks: self.num_weeks,
            roadmap_projects: self.roadmap_projects,
            technical_projects: self.technical_projects,
            allocations: self.allocations,
            metadata: self.metadata,
        };

        (prefs, state)
    }

    /// Validate the export format
    pub fn validate(&self) -> Result<(), ExportValidationError> {
        // Check version
        if self.version.is_empty() {
            return Err(ExportValidationError::InvalidVersion);
        }

        // Check team context
        if self.team_name.trim().is_empty() {
            return Err(ExportValidationError::EmptyTeamName);
        }

        if self.team_members.is_empty() {
            return Err(ExportValidationError::NoTeamMembers);
        }

        // Check planning data
        if self.quarter_name.trim().is_empty() {
            return Err(ExportValidationError::EmptyQuarterName);
        }

        if self.num_weeks == 0 {
            return Err(ExportValidationError::InvalidNumWeeks);
        }

        // Validate referential integrity
        // All allocations should reference valid team members
        for allocation in &self.allocations {
            if !self
                .team_members
                .iter()
                .any(|m| m.id == allocation.team_member_id)
            {
                return Err(ExportValidationError::InvalidTeamMemberReference(
                    allocation.team_member_id,
                ));
            }
        }

        // All assignments should reference valid technical projects
        for allocation in &self.allocations {
            for assignment in &allocation.assignments {
                if !self
                    .technical_projects
                    .iter()
                    .any(|p| p.id == assignment.technical_project_id)
                {
                    return Err(ExportValidationError::InvalidTechnicalProjectReference(
                        assignment.technical_project_id,
                    ));
                }
            }
        }

        // All technical projects with roadmap links should reference valid roadmap projects
        for tech_project in &self.technical_projects {
            if let Some(roadmap_id) = tech_project.roadmap_project_id {
                if !self.roadmap_projects.iter().any(|p| p.id == roadmap_id) {
                    return Err(ExportValidationError::InvalidRoadmapProjectReference(
                        roadmap_id,
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Validation errors for plan exports
#[allow(dead_code)] // Used in M13-14 for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ExportValidationError {
    InvalidVersion,
    EmptyTeamName,
    NoTeamMembers,
    EmptyQuarterName,
    InvalidNumWeeks,
    InvalidTeamMemberReference(uuid::Uuid),
    InvalidTechnicalProjectReference(uuid::Uuid),
    InvalidRoadmapProjectReference(uuid::Uuid),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Role;

    fn create_sample_export() -> PlanExport {
        let prefs = Preferences {
            team_name: "Backend Team".to_string(),
            team_members: vec![TeamMember::new(
                "Alice Kim".to_string(),
                Role::Engineering,
                12.0,
            )],
            sprint_anchor_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            sprint_length_weeks: 2,
            default_capacity: 12.0,
        };

        let quarter_start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let state = PlanState::new("Q1 2025".to_string(), quarter_start, 13);

        PlanExport::from_signals(prefs, state)
    }

    #[test]
    fn test_plan_export_creation() {
        let export = create_sample_export();

        assert_eq!(export.version, "1.0");
        assert_eq!(export.team_name, "Backend Team");
        assert_eq!(export.team_members.len(), 1);
        assert_eq!(export.quarter_name, "Q1 2025");
        assert_eq!(export.num_weeks, 13);
    }

    #[test]
    fn test_export_validation_valid() {
        let export = create_sample_export();
        assert!(export.validate().is_ok());
    }

    #[test]
    fn test_export_validation_empty_team_name() {
        let mut export = create_sample_export();
        export.team_name = "".to_string();
        assert_eq!(export.validate(), Err(ExportValidationError::EmptyTeamName));
    }

    #[test]
    fn test_export_validation_no_team_members() {
        let mut export = create_sample_export();
        export.team_members.clear();
        assert_eq!(export.validate(), Err(ExportValidationError::NoTeamMembers));
    }

    #[test]
    fn test_export_round_trip() {
        let original_prefs = Preferences::default();
        let quarter_start = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let original_state = PlanState::new("Q1 2025".to_string(), quarter_start, 13);

        let export = PlanExport::from_signals(original_prefs.clone(), original_state.clone());
        let (restored_prefs, restored_state) = export.into_signals();

        assert_eq!(original_prefs.team_name, restored_prefs.team_name);
        assert_eq!(original_prefs.team_members, restored_prefs.team_members);
        assert_eq!(original_state.quarter_name, restored_state.quarter_name);
        assert_eq!(
            original_state.quarter_start_date,
            restored_state.quarter_start_date
        );
    }

    #[test]
    fn test_export_serialization() {
        let export = create_sample_export();

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&export).expect("Failed to serialize");

        // Deserialize back
        let deserialized: PlanExport = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(export, deserialized);
    }
}
