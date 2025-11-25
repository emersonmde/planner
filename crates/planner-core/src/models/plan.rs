use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Epsilon for floating point percentage comparisons
#[allow(dead_code)] // Reserved for future validation UI
const PERCENTAGE_EPSILON: f32 = 0.01;

/// Valid percentage range (0.0 to 100.0 inclusive)
const VALID_PERCENTAGE_RANGE: std::ops::RangeInclusive<f32> = 0.0..=100.0;

/// Role of a team member (Software Engineer or Applied Scientist)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "eng")]
    Engineering,
    #[serde(rename = "sci")]
    Science,
}

impl Role {
    /// Short display name for badges (e.g., "SDE", "AS")
    pub fn short_name(&self) -> &'static str {
        match self {
            Role::Engineering => "SDE",
            Role::Science => "AS",
        }
    }
}

/// Project color for visual differentiation in the allocation grid
/// Reference: docs/ui-design.md section 2 (Project Color Palette)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectColor {
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Pink,
    Teal,
    Indigo,
}

impl ProjectColor {
    /// Convert to hex color string for CSS (candy bright palette - modern dark mode)
    pub fn to_hex(self) -> &'static str {
        match self {
            ProjectColor::Blue => "#5AC8FA",   // iOS blue - vibrant but refined
            ProjectColor::Green => "#4ADE80",  // Fresh green - energetic
            ProjectColor::Yellow => "#FBBF24", // Amber gold - premium
            ProjectColor::Orange => "#FB923C", // Warm orange - inviting
            ProjectColor::Red => "#F472B6",    // Vibrant pink - playful
            ProjectColor::Purple => "#A78BFA", // Soft purple - elegant
            ProjectColor::Pink => "#E879F9",   // Bright magenta - bold
            ProjectColor::Teal => "#2DD4BF",   // Aqua - refreshing
            ProjectColor::Indigo => "#818CF8", // Periwinkle - calm
        }
    }

    /// Convert to CSS variable name
    #[allow(dead_code)] // Reserved for future CSS-in-JS styling
    pub fn to_css_var(self) -> &'static str {
        match self {
            ProjectColor::Blue => "var(--project-cyan)",
            ProjectColor::Green => "var(--project-lime)",
            ProjectColor::Yellow => "var(--project-yellow)",
            ProjectColor::Orange => "var(--project-orange)",
            ProjectColor::Red => "var(--project-pink)",
            ProjectColor::Purple => "var(--project-violet)",
            ProjectColor::Pink => "var(--project-magenta)",
            ProjectColor::Teal => "var(--project-teal)",
            ProjectColor::Indigo => "var(--project-purple)",
        }
    }
}

/// Team member (Software Engineer or Applied Scientist) with capacity
/// Reference: docs/ui-design.md section 9.2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    /// Capacity in weeks for the quarter (e.g., 12 weeks for full-time team member)
    pub capacity: f32,
}

impl TeamMember {
    pub fn new(name: String, role: Role, capacity: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role,
            capacity,
        }
    }
}

/// High-level roadmap project (initiative)
/// Reference: docs/ui-design.md section 9.2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoadmapProject {
    pub id: Uuid,
    pub name: String,
    /// Engineering estimate in weeks
    pub eng_estimate: f32,
    /// Science estimate in weeks
    pub sci_estimate: f32,
    /// Project start date
    pub start_date: NaiveDate,
    /// Target launch date
    pub launch_date: NaiveDate,
    /// Visual color for this project
    pub color: ProjectColor,
    /// Optional notes
    pub notes: Option<String>,
}

impl RoadmapProject {
    pub fn new(
        name: String,
        eng_estimate: f32,
        sci_estimate: f32,
        start_date: NaiveDate,
        launch_date: NaiveDate,
        color: ProjectColor,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            eng_estimate,
            sci_estimate,
            start_date,
            launch_date,
            color,
            notes: None,
        }
    }

    /// Total estimated weeks (engineering + science)
    #[allow(dead_code)] // Reserved for roadmap calculations
    pub fn total_estimate(&self) -> f32 {
        self.eng_estimate + self.sci_estimate
    }
}

/// Technical project (implementation work linked to a roadmap project)
/// Reference: docs/ui-design.md section 9.2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechnicalProject {
    pub id: Uuid,
    pub name: String,
    /// Optional link to parent roadmap project
    pub roadmap_project_id: Option<Uuid>,
    /// Estimated engineering weeks
    pub eng_estimate: f32,
    /// Estimated science weeks
    pub sci_estimate: f32,
    /// Project start date
    pub start_date: NaiveDate,
    /// Optional expected completion date
    pub expected_completion: Option<NaiveDate>,
    /// Optional notes
    pub notes: Option<String>,
}

#[allow(dead_code)] // Methods used in future milestones
impl TechnicalProject {
    pub fn new(
        name: String,
        roadmap_project_id: Option<Uuid>,
        eng_estimate: f32,
        sci_estimate: f32,
        start_date: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            roadmap_project_id,
            eng_estimate,
            sci_estimate,
            start_date,
            expected_completion: None,
            notes: None,
        }
    }

    /// Get total estimated weeks (eng + sci)
    pub fn total_estimate(&self) -> f32 {
        self.eng_estimate + self.sci_estimate
    }

    /// Get the color for this technical project from PlanState
    /// Returns Blue if no roadmap project is linked
    pub fn get_color_from_state(&self, state: &super::PlanState) -> ProjectColor {
        self.roadmap_project_id
            .and_then(|id| state.get_roadmap_project(&id))
            .map(|rp| rp.color)
            .unwrap_or(ProjectColor::Blue)
    }
}

/// Assignment of a team member to a project for a specific week
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub technical_project_id: Uuid,
    /// Percentage of the week (0.0 to 100.0)
    /// For split allocations, multiple assignments for same week must sum to 100
    pub percentage: f32,
}

impl Assignment {
    pub fn new(technical_project_id: Uuid, percentage: f32) -> Self {
        assert!(
            VALID_PERCENTAGE_RANGE.contains(&percentage),
            "Percentage must be between 0 and 100, got {}",
            percentage
        );
        Self {
            technical_project_id,
            percentage,
        }
    }
}

/// Weekly allocation for a team member
/// Reference: docs/ui-design.md section 9.2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    pub team_member_id: Uuid,
    /// Start date of the week (Monday)
    pub week_start_date: NaiveDate,
    /// Assignments for this week (can be 0, 1, or 2 for split allocation)
    pub assignments: Vec<Assignment>,
}

impl Allocation {
    pub fn new(team_member_id: Uuid, week_start_date: NaiveDate) -> Self {
        Self {
            team_member_id,
            week_start_date,
            assignments: Vec::new(),
        }
    }

    /// Helper to check if total percentage equals target within epsilon
    #[allow(dead_code)] // Reserved for validation logic
    fn percentage_equals(&self, target: f32) -> bool {
        (self.total_percentage() - target).abs() < PERCENTAGE_EPSILON
    }

    /// Validate that assignments sum to 100% (or 0% if empty)
    #[allow(dead_code)] // Reserved for validation UI
    pub fn is_valid(&self) -> bool {
        self.is_empty() || self.percentage_equals(100.0)
    }

    /// Get total allocated percentage
    pub fn total_percentage(&self) -> f32 {
        self.assignments.iter().map(|a| a.percentage).sum()
    }

    /// Check if this week is fully allocated
    #[allow(dead_code)] // Reserved for editing UI
    pub fn is_full(&self) -> bool {
        self.percentage_equals(100.0)
    }

    /// Check if this week is unallocated
    #[allow(dead_code)] // Reserved for editing UI
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}

/// Complete quarterly plan data
///
/// **DEPRECATED (Milestone 9)**: This struct is kept for backward compatibility during
/// the migration to two-signal architecture. New code should use `Preferences` and `PlanState`
/// directly via the `use_preferences()` and `use_plan_state()` hooks.
///
/// This wrapper will be removed in Milestone 10+ once all components have migrated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    /// File format version for future compatibility
    pub version: String,
    /// Quarter (e.g., "Q1 2025")
    pub quarter: String,
    /// Quarter start date (first Monday)
    pub quarter_start_date: NaiveDate,
    /// Number of weeks in the quarter (typically 13)
    pub weeks_in_quarter: usize,
    /// Sprint length in weeks (typically 2)
    #[serde(default = "default_sprint_length")]
    pub sprint_length_weeks: usize,
    /// Sprint anchor date (global)
    #[serde(default = "default_sprint_anchor")]
    pub sprint_anchor_date: NaiveDate,
    /// Team name
    #[serde(default = "default_team_name")]
    pub team_name: String,
    /// All team members (engineers and scientists)
    pub team_members: Vec<TeamMember>,
    /// All roadmap projects
    pub roadmap_projects: Vec<RoadmapProject>,
    /// All technical projects
    pub technical_projects: Vec<TechnicalProject>,
    /// All allocations
    pub allocations: Vec<Allocation>,
}

fn default_sprint_length() -> usize {
    2
}

fn default_sprint_anchor() -> NaiveDate {
    NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid sprint anchor")
}

fn default_team_name() -> String {
    "My Team".to_string()
}

#[allow(dead_code)] // Plan wrapper kept for backward compatibility, will be removed in M10+
impl Plan {
    /// Get team member by ID
    pub fn get_team_member(&self, id: &Uuid) -> Option<&TeamMember> {
        self.team_members.iter().find(|e| &e.id == id)
    }

    /// Get roadmap project by ID
    pub fn get_roadmap_project(&self, id: &Uuid) -> Option<&RoadmapProject> {
        self.roadmap_projects.iter().find(|p| &p.id == id)
    }

    /// Get technical project by ID
    pub fn get_technical_project(&self, id: &Uuid) -> Option<&TechnicalProject> {
        self.technical_projects.iter().find(|p| &p.id == id)
    }

    /// Get all allocations for a specific team member
    pub fn get_team_member_allocations(&self, team_member_id: &Uuid) -> Vec<&Allocation> {
        self.allocations
            .iter()
            .filter(|a| &a.team_member_id == team_member_id)
            .collect()
    }

    /// Calculate total allocated weeks for a team member
    pub fn calculate_allocated_weeks(&self, team_member_id: &Uuid) -> f32 {
        self.get_team_member_allocations(team_member_id)
            .iter()
            .map(|a| a.total_percentage() / 100.0)
            .sum()
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

    /// Calculate allocated weeks for a roadmap project (sum of all linked technical projects)
    /// Returns (eng_allocated, sci_allocated, total_allocated)
    pub fn calculate_roadmap_allocated_weeks(&self, roadmap_project_id: &Uuid) -> (f32, f32, f32) {
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
                if let Some(engineer) = self.get_team_member(&allocation.team_member_id) {
                    for assignment in &allocation.assignments {
                        if assignment.technical_project_id == tech_project.id {
                            let weeks = assignment.percentage / 100.0;
                            match engineer.role {
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

    /// Calculate total capacity by role
    /// Returns (eng_capacity, sci_capacity, total_capacity)
    pub fn calculate_total_capacity(&self) -> (f32, f32, f32) {
        let mut eng_capacity = 0.0;
        let mut sci_capacity = 0.0;

        for member in &self.team_members {
            match member.role {
                Role::Engineering => eng_capacity += member.capacity,
                Role::Science => sci_capacity += member.capacity,
            }
        }

        let total_capacity = eng_capacity + sci_capacity;
        (eng_capacity, sci_capacity, total_capacity)
    }

    /// Calculate total allocated weeks across all allocations by role
    /// Returns (eng_allocated, sci_allocated, total_allocated)
    pub fn calculate_total_allocated(&self) -> (f32, f32, f32) {
        let mut eng_allocated = 0.0;
        let mut sci_allocated = 0.0;

        for allocation in &self.allocations {
            if let Some(engineer) = self.get_team_member(&allocation.team_member_id) {
                let weeks = allocation.total_percentage() / 100.0;
                match engineer.role {
                    Role::Engineering => eng_allocated += weeks,
                    Role::Science => sci_allocated += weeks,
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
    ///
    /// **Note**: Uses sprint_anchor_date (global) instead of quarter_start_date
    pub fn update_technical_project_dates(&mut self, technical_project_id: &Uuid) {
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

        // Calculate sprint boundaries using global anchor
        let (first_sprint_start, _) = get_sprint_boundaries(
            first_week,
            self.sprint_anchor_date,
            self.sprint_length_weeks,
        );
        let (_, last_sprint_end) =
            get_sprint_boundaries(last_week, self.sprint_anchor_date, self.sprint_length_weeks);

        // Update the technical project
        if let Some(project) = self
            .technical_projects
            .iter_mut()
            .find(|p| p.id == *technical_project_id)
        {
            project.start_date = first_sprint_start;
            project.expected_completion = Some(last_sprint_end);
        }
    }
}

/// Determine capacity badge status based on allocated vs estimated
/// Neutral: 0/0 (no estimate, no allocation)
/// Success: within 5% (±0.5 weeks per 10 weeks)
/// Warning: 5-25% off, or allocated without estimate
/// Error: >25% off
pub fn get_capacity_status(allocated: f32, estimated: f32) -> super::BadgeType {
    use super::BadgeType;

    // Handle zero estimate cases
    if estimated == 0.0 {
        return if allocated == 0.0 {
            // No estimate and no allocation - neutral state
            BadgeType::Neutral
        } else {
            // Allocated without an estimate - warning
            BadgeType::Warning
        };
    }

    // Calculate how close allocation is to estimate
    let diff_pct = ((allocated - estimated) / estimated * 100.0).abs();

    if diff_pct <= 5.0 {
        BadgeType::Success // Within 5% of estimate
    } else if diff_pct <= 25.0 {
        BadgeType::Warning // Within 25% of estimate
    } else {
        BadgeType::Error // More than 25% off from estimate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::BadgeType;
    use chrono::NaiveDate;

    // ===========================================
    // Allocation Tests
    // ===========================================

    #[test]
    fn test_allocation_new() {
        let member_id = Uuid::new_v4();
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let alloc = Allocation::new(member_id, week);

        assert_eq!(alloc.team_member_id, member_id);
        assert_eq!(alloc.week_start_date, week);
        assert!(alloc.assignments.is_empty());
    }

    #[test]
    fn test_allocation_is_empty() {
        let member_id = Uuid::new_v4();
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();

        let empty_alloc = Allocation::new(member_id, week);
        assert!(empty_alloc.is_empty());

        let mut full_alloc = Allocation::new(member_id, week);
        full_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 100.0));
        assert!(!full_alloc.is_empty());
    }

    #[test]
    fn test_allocation_total_percentage() {
        let member_id = Uuid::new_v4();
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();

        // Empty allocation
        let empty_alloc = Allocation::new(member_id, week);
        assert_eq!(empty_alloc.total_percentage(), 0.0);

        // Single 100% allocation
        let mut full_alloc = Allocation::new(member_id, week);
        full_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 100.0));
        assert_eq!(full_alloc.total_percentage(), 100.0);

        // Split allocation (50/50)
        let mut split_alloc = Allocation::new(member_id, week);
        split_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 50.0));
        split_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 50.0));
        assert_eq!(split_alloc.total_percentage(), 100.0);

        // Partial allocation
        let mut partial_alloc = Allocation::new(member_id, week);
        partial_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 60.0));
        assert_eq!(partial_alloc.total_percentage(), 60.0);
    }

    #[test]
    fn test_allocation_is_full() {
        let member_id = Uuid::new_v4();
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();

        // Empty is not full
        let empty_alloc = Allocation::new(member_id, week);
        assert!(!empty_alloc.is_full());

        // 100% is full
        let mut full_alloc = Allocation::new(member_id, week);
        full_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 100.0));
        assert!(full_alloc.is_full());

        // 50/50 split is full
        let mut split_alloc = Allocation::new(member_id, week);
        split_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 50.0));
        split_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 50.0));
        assert!(split_alloc.is_full());

        // 60% is not full
        let mut partial_alloc = Allocation::new(member_id, week);
        partial_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 60.0));
        assert!(!partial_alloc.is_full());
    }

    #[test]
    fn test_allocation_is_valid() {
        let member_id = Uuid::new_v4();
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();

        // Empty is valid
        let empty_alloc = Allocation::new(member_id, week);
        assert!(empty_alloc.is_valid());

        // 100% is valid
        let mut full_alloc = Allocation::new(member_id, week);
        full_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 100.0));
        assert!(full_alloc.is_valid());

        // 60% is invalid (not empty, not 100%)
        let mut partial_alloc = Allocation::new(member_id, week);
        partial_alloc
            .assignments
            .push(Assignment::new(Uuid::new_v4(), 60.0));
        assert!(!partial_alloc.is_valid());
    }

    // ===========================================
    // get_capacity_status Tests
    // ===========================================

    #[test]
    fn test_capacity_status_neutral() {
        // 0 allocated, 0 estimated -> Neutral
        assert_eq!(get_capacity_status(0.0, 0.0), BadgeType::Neutral);
    }

    #[test]
    fn test_capacity_status_warning_no_estimate() {
        // Allocated without estimate -> Warning
        assert_eq!(get_capacity_status(5.0, 0.0), BadgeType::Warning);
        assert_eq!(get_capacity_status(0.1, 0.0), BadgeType::Warning);
    }

    #[test]
    fn test_capacity_status_success() {
        // Within 5% of estimate -> Success
        assert_eq!(get_capacity_status(10.0, 10.0), BadgeType::Success); // Exact match
        assert_eq!(get_capacity_status(10.5, 10.0), BadgeType::Success); // 5% over
        assert_eq!(get_capacity_status(9.5, 10.0), BadgeType::Success); // 5% under
        assert_eq!(get_capacity_status(10.4, 10.0), BadgeType::Success); // 4% over
    }

    #[test]
    fn test_capacity_status_warning() {
        // 5-25% off from estimate -> Warning
        assert_eq!(get_capacity_status(10.6, 10.0), BadgeType::Warning); // 6% over
        assert_eq!(get_capacity_status(9.4, 10.0), BadgeType::Warning); // 6% under
        assert_eq!(get_capacity_status(12.5, 10.0), BadgeType::Warning); // 25% over
        assert_eq!(get_capacity_status(7.5, 10.0), BadgeType::Warning); // 25% under
    }

    #[test]
    fn test_capacity_status_error() {
        // More than 25% off from estimate -> Error
        assert_eq!(get_capacity_status(12.6, 10.0), BadgeType::Error); // 26% over
        assert_eq!(get_capacity_status(7.4, 10.0), BadgeType::Error); // 26% under
        assert_eq!(get_capacity_status(20.0, 10.0), BadgeType::Error); // 100% over
        assert_eq!(get_capacity_status(1.0, 10.0), BadgeType::Error); // 90% under
    }

    // ===========================================
    // Plan Calculation Tests
    // ===========================================

    fn create_test_plan() -> Plan {
        let eng1 = TeamMember {
            id: Uuid::new_v4(),
            name: "Engineer 1".to_string(),
            role: Role::Engineering,
            capacity: 10.0,
        };
        let eng2 = TeamMember {
            id: Uuid::new_v4(),
            name: "Engineer 2".to_string(),
            role: Role::Engineering,
            capacity: 8.0,
        };
        let sci1 = TeamMember {
            id: Uuid::new_v4(),
            name: "Scientist 1".to_string(),
            role: Role::Science,
            capacity: 6.0,
        };

        Plan {
            version: "1.0".to_string(),
            quarter: "Q1 2025".to_string(),
            quarter_start_date: NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
            weeks_in_quarter: 13,
            sprint_length_weeks: 2,
            sprint_anchor_date: NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
            team_name: "Test Team".to_string(),
            team_members: vec![eng1, eng2, sci1],
            roadmap_projects: vec![],
            technical_projects: vec![],
            allocations: vec![],
        }
    }

    #[test]
    fn test_calculate_total_capacity() {
        let plan = create_test_plan();

        let (eng_cap, sci_cap, total_cap) = plan.calculate_total_capacity();

        assert_eq!(eng_cap, 18.0); // 10 + 8
        assert_eq!(sci_cap, 6.0);
        assert_eq!(total_cap, 24.0);
    }

    #[test]
    fn test_calculate_total_capacity_empty_team() {
        let mut plan = create_test_plan();
        plan.team_members.clear();

        let (eng_cap, sci_cap, total_cap) = plan.calculate_total_capacity();

        assert_eq!(eng_cap, 0.0);
        assert_eq!(sci_cap, 0.0);
        assert_eq!(total_cap, 0.0);
    }

    #[test]
    fn test_calculate_total_allocated_no_allocations() {
        let plan = create_test_plan();

        let (eng_alloc, sci_alloc, total_alloc) = plan.calculate_total_allocated();

        assert_eq!(eng_alloc, 0.0);
        assert_eq!(sci_alloc, 0.0);
        assert_eq!(total_alloc, 0.0);
    }

    #[test]
    fn test_calculate_total_allocated_with_allocations() {
        let mut plan = create_test_plan();
        let eng1_id = plan.team_members[0].id;
        let sci1_id = plan.team_members[2].id;
        let project_id = Uuid::new_v4();

        // Add allocations: 2 weeks for eng1 (100% each), 1 week for sci1 (50%)
        let week1 = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let week2 = NaiveDate::from_ymd_opt(2025, 1, 13).unwrap();

        let mut alloc1 = Allocation::new(eng1_id, week1);
        alloc1.assignments.push(Assignment::new(project_id, 100.0));
        plan.allocations.push(alloc1);

        let mut alloc2 = Allocation::new(eng1_id, week2);
        alloc2.assignments.push(Assignment::new(project_id, 100.0));
        plan.allocations.push(alloc2);

        let mut alloc3 = Allocation::new(sci1_id, week1);
        alloc3.assignments.push(Assignment::new(project_id, 50.0));
        plan.allocations.push(alloc3);

        let (eng_alloc, sci_alloc, total_alloc) = plan.calculate_total_allocated();

        assert_eq!(eng_alloc, 2.0); // 2 weeks at 100%
        assert_eq!(sci_alloc, 0.5); // 1 week at 50%
        assert_eq!(total_alloc, 2.5);
    }

    #[test]
    fn test_calculate_allocated_weeks_for_member() {
        let mut plan = create_test_plan();
        let eng1_id = plan.team_members[0].id;
        let eng2_id = plan.team_members[1].id;
        let project_id = Uuid::new_v4();

        // eng1 gets 3 weeks of allocation
        for i in 0..3 {
            let week = NaiveDate::from_ymd_opt(2025, 1, 6 + i * 7).unwrap();
            let mut alloc = Allocation::new(eng1_id, week);
            alloc.assignments.push(Assignment::new(project_id, 100.0));
            plan.allocations.push(alloc);
        }

        // eng2 gets 1 week at 50%
        let week = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
        let mut alloc = Allocation::new(eng2_id, week);
        alloc.assignments.push(Assignment::new(project_id, 50.0));
        plan.allocations.push(alloc);

        assert_eq!(plan.calculate_allocated_weeks(&eng1_id), 3.0);
        assert_eq!(plan.calculate_allocated_weeks(&eng2_id), 0.5);
    }

    #[test]
    fn test_calculate_project_allocated_weeks() {
        let mut plan = create_test_plan();
        let eng1_id = plan.team_members[0].id;
        let project1_id = Uuid::new_v4();
        let project2_id = Uuid::new_v4();

        // Project 1: 2 full weeks from eng1
        for i in 0..2 {
            let week = NaiveDate::from_ymd_opt(2025, 1, 6 + i * 7).unwrap();
            let mut alloc = Allocation::new(eng1_id, week);
            alloc.assignments.push(Assignment::new(project1_id, 100.0));
            plan.allocations.push(alloc);
        }

        // Project 2: 1 week split allocation (50%)
        let week = NaiveDate::from_ymd_opt(2025, 1, 20).unwrap();
        let mut alloc = Allocation::new(eng1_id, week);
        alloc.assignments.push(Assignment::new(project1_id, 50.0));
        alloc.assignments.push(Assignment::new(project2_id, 50.0));
        plan.allocations.push(alloc);

        assert_eq!(plan.calculate_project_allocated_weeks(&project1_id), 2.5); // 2 full + 0.5 split
        assert_eq!(plan.calculate_project_allocated_weeks(&project2_id), 0.5); // 0.5 from split
    }
}
