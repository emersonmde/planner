//! Global state management using Dioxus signals
//! Reference: Dioxus 0.7 context API for sharing state
//!
//! ## Two-Signal Architecture (Milestone 9)
//!
//! The application uses two independent signals:
//! - `preferences`: Team roster, sprint config (persisted to localStorage)
//! - `plan_state`: Projects, allocations (exported/imported per quarter)
//!
//! ## Viewing Mode (Milestone 13)
//!
//! When viewing an imported plan file:
//! - `viewing_session`: Tracks the loaded file and modification state
//! - Auto-save is disabled (localStorage untouched)
//! - On close, reload from localStorage to restore user's own plan
//!
//! See ADR-005 for design rationale.

use chrono::NaiveDate;
use dioxus::prelude::*;

use planner_core::models::*;
use planner_core::utils::get_quarter_start_date;

/// State for viewing an imported/loaded plan file
///
/// When viewing a file, the plan data is loaded into the main signals
/// but NOT persisted to localStorage. The original localStorage data
/// is preserved and can be restored by closing the viewing session.
#[derive(Clone, PartialEq)]
pub struct ViewingSession {
    /// Filename of the loaded plan (for display in UI)
    pub filename: String,

    /// Original export data (for detecting modifications)
    /// Stored as JSON string for easy comparison
    pub original_json: String,

    /// Whether the user has made changes to the loaded plan
    pub modified: bool,
}

/// Global application context with two independent signals + viewing mode
///
/// This replaces the old single `Signal<Plan>` with two signals:
/// - `preferences`: Long-term team configuration
/// - `plan_state`: Quarter-specific planning data
/// - `viewing_session`: Optional viewing state for imported plans
#[derive(Clone, Copy)]
pub struct AppContext {
    pub preferences: Signal<Preferences>,
    pub plan_state: Signal<PlanState>,
    pub viewing_session: Signal<Option<ViewingSession>>,
}

/// Hook to access team preferences (persisted to localStorage)
///
/// Preferences include:
/// - Team name
/// - Team roster (members with roles and capacity)
/// - Sprint configuration (anchor date, length)
/// - Default capacity
pub fn use_preferences() -> Signal<Preferences> {
    use_context::<AppContext>().preferences
}

/// Hook to access planning state (exported/imported per quarter)
///
/// Plan state includes:
/// - Quarter info (name, start date, duration)
/// - Roadmap projects
/// - Technical projects
/// - Allocations
/// - Metadata (version, timestamps)
pub fn use_plan_state() -> Signal<PlanState> {
    use_context::<AppContext>().plan_state
}

/// Hook to access viewing session (for imported plan files)
///
/// Returns `None` when working on local plan, `Some(ViewingSession)` when
/// viewing an imported file. Use this to:
/// - Show/hide viewing mode UI indicators
/// - Disable auto-save when viewing
/// - Track modifications to loaded plans
pub fn use_viewing_session() -> Signal<Option<ViewingSession>> {
    use_context::<AppContext>().viewing_session
}

/// Initialize the application state with sample data
/// This creates team members, projects, and allocations for Q1 2025
///
/// Returns (Preferences, PlanState) tuple for two-signal architecture
pub fn create_sample_plan() -> (Preferences, PlanState) {
    let quarter_start = get_quarter_start_date(2025, 1).expect("Valid Q1 2025 date");

    // Create preferences (team config)
    let mut preferences = Preferences {
        schema_version: planner_core::models::PREFERENCES_SCHEMA_VERSION.to_string(),
        team_name: "Engineering Team".to_string(),
        team_members: Vec::new(),
        sprint_anchor_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("Valid anchor date"),
        sprint_length_weeks: 2,
        default_capacity: 12.0,
    };

    // Create plan state (quarter-specific data)
    let mut plan_state = PlanState::new("Q1 2025".to_string(), quarter_start, 13);

    // Create team members
    let alice = TeamMember::new("Alice Kim".to_string(), Role::Engineering, 12.0);
    let bob = TeamMember::new("Bob Martinez".to_string(), Role::Engineering, 12.0);
    let carol = TeamMember::new("Carol Smith".to_string(), Role::Science, 6.0);
    let dave = TeamMember::new("Dave Roberts".to_string(), Role::Engineering, 12.0);

    let alice_id = alice.id;
    let bob_id = bob.id;
    let carol_id = carol.id;
    let dave_id = dave.id;

    preferences.team_members.push(alice);
    preferences.team_members.push(bob);
    preferences.team_members.push(carol);
    preferences.team_members.push(dave);

    // Create roadmap projects
    let platform_project = RoadmapProject::new(
        "Q1 Platform Improvements".to_string(),
        24.0, // Engineering estimate
        8.0,  // Science estimate
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
        NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        ProjectColor::Blue,
    );

    let payment_project = RoadmapProject::new(
        "Payment Gateway".to_string(),
        8.0,
        0.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 28).unwrap(),
        ProjectColor::Green,
    );

    let data_project = RoadmapProject::new(
        "Data Pipeline Overhaul".to_string(),
        16.0,
        6.0,
        NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(),
        NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(),
        ProjectColor::Yellow,
    );

    let platform_roadmap_id = platform_project.id;
    let payment_roadmap_id = payment_project.id;
    let data_roadmap_id = data_project.id;

    plan_state.roadmap_projects.push(platform_project);
    plan_state.roadmap_projects.push(payment_project);
    plan_state.roadmap_projects.push(data_project);

    // Create technical projects (with eng_estimate, sci_estimate split)
    let auth_service = TechnicalProject::new(
        "Auth Service Refactor".to_string(),
        Some(platform_roadmap_id),
        6.0, // eng_estimate
        0.0, // sci_estimate
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let payment_api = TechnicalProject::new(
        "Payment API Integration".to_string(),
        Some(payment_roadmap_id),
        8.0, // eng_estimate
        0.0, // sci_estimate
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let ml_pipeline = TechnicalProject::new(
        "ML Pipeline Optimization".to_string(),
        Some(platform_roadmap_id),
        6.0, // eng_estimate
        6.0, // sci_estimate
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let data_pipeline = TechnicalProject::new(
        "Data Pipeline Migration".to_string(),
        Some(data_roadmap_id),
        6.0, // eng_estimate
        4.0, // sci_estimate
        NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(),
    );

    let research = TechnicalProject::new(
        "Algorithm Research".to_string(),
        Some(data_roadmap_id),
        0.0, // eng_estimate
        6.0, // sci_estimate
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let auth_tech_id = auth_service.id;
    let payment_tech_id = payment_api.id;
    let ml_tech_id = ml_pipeline.id;
    let data_pipe_tech_id = data_pipeline.id;
    let research_tech_id = research.id;

    plan_state.technical_projects.push(auth_service);
    plan_state.technical_projects.push(payment_api);
    plan_state.technical_projects.push(ml_pipeline);
    plan_state.technical_projects.push(data_pipeline);
    plan_state.technical_projects.push(research);

    // Create sample allocations for first few weeks
    // Alice: Payment API for weeks 1-3, then switches to split allocation
    for week_num in 0..3 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(alice_id, week_start);
        alloc
            .assignments
            .push(Assignment::new(payment_tech_id, 100.0));
        plan_state.allocations.push(alloc);
    }

    // Alice: Split allocation in week 4 (60% Payment, 40% Data)
    let week_4_start = quarter_start + chrono::Duration::weeks(3);
    let mut alice_split = Allocation::new(alice_id, week_4_start);
    alice_split
        .assignments
        .push(Assignment::new(payment_tech_id, 60.0));
    alice_split
        .assignments
        .push(Assignment::new(data_pipe_tech_id, 40.0));
    plan_state.allocations.push(alice_split);

    // Bob: ML Pipeline for weeks 1-4
    for week_num in 0..4 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(bob_id, week_start);
        alloc.assignments.push(Assignment::new(ml_tech_id, 100.0));
        plan_state.allocations.push(alloc);
    }

    // Carol: Research for weeks 1-2, unallocated week 3, research week 4
    for week_num in 0..4 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(carol_id, week_start);
        if week_num != 2 {
            // Skip week 3 (unallocated)
            alloc
                .assignments
                .push(Assignment::new(research_tech_id, 100.0));
        }
        plan_state.allocations.push(alloc);
    }

    // Dave: Auth Service for weeks 1-3
    for week_num in 0..3 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(dave_id, week_start);
        alloc.assignments.push(Assignment::new(auth_tech_id, 100.0));
        plan_state.allocations.push(alloc);
    }

    (preferences, plan_state)
}

/// Validation errors for the plan
#[allow(dead_code)] // Reserved for validation UI
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Allocation doesn't sum to 100%
    InvalidAllocationPercentage {
        engineer_id: uuid::Uuid,
        week_start: NaiveDate,
        total: f32,
    },
    /// Engineer over-allocated (exceeds capacity)
    OverAllocated {
        engineer_name: String,
        capacity: f32,
        allocated: f32,
    },
    /// Allocation before project start date
    BeforeStartDate {
        engineer_name: String,
        project_name: String,
        week_start: NaiveDate,
        project_start: NaiveDate,
    },
}

/// Validate the entire plan
#[allow(dead_code)] // Reserved for validation UI
pub fn validate_plan(plan: &Plan) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Check allocation percentages
    for allocation in &plan.allocations {
        if !allocation.is_valid() {
            errors.push(ValidationError::InvalidAllocationPercentage {
                engineer_id: allocation.team_member_id,
                week_start: allocation.week_start_date,
                total: allocation.total_percentage(),
            });
        }
    }

    // Check engineer capacity
    for engineer in &plan.team_members {
        let allocated = plan.calculate_allocated_weeks(&engineer.id);
        if allocated > engineer.capacity {
            errors.push(ValidationError::OverAllocated {
                engineer_name: engineer.name.clone(),
                capacity: engineer.capacity,
                allocated,
            });
        }
    }

    // Check allocation dates vs project start dates
    for allocation in &plan.allocations {
        for assignment in &allocation.assignments {
            if let Some(tech_project) = plan.get_technical_project(&assignment.technical_project_id)
            {
                if allocation.week_start_date < tech_project.start_date {
                    if let Some(engineer) = plan.get_team_member(&allocation.team_member_id) {
                        errors.push(ValidationError::BeforeStartDate {
                            engineer_name: engineer.name.clone(),
                            project_name: tech_project.name.clone(),
                            week_start: allocation.week_start_date,
                            project_start: tech_project.start_date,
                        });
                    }
                }
            }
        }
    }

    errors
}
