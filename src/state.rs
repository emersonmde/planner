//! Global state management using Dioxus signals
//! Reference: Dioxus 0.7 context API for sharing state

use chrono::NaiveDate;
use dioxus::prelude::*;

use crate::models::*;
use crate::utils::get_quarter_start_date;

/// Global application state
#[allow(dead_code)] // Reserved for future state management refactoring
#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub plan: Signal<Plan>,
}

/// Hook to access the global plan state
/// This uses Dioxus 0.7's context API
pub fn use_plan_state() -> Signal<Plan> {
    use_context::<Signal<Plan>>()
}

/// Initialize the application state with sample data
/// This creates team members, projects, and allocations for Q1 2025
pub fn create_sample_plan() -> Plan {
    let quarter_start = get_quarter_start_date(2025, 1).expect("Valid Q1 2025 date");
    let mut plan = Plan::new("Q1 2025".to_string(), quarter_start, 13);

    // Create team members
    let alice = TeamMember::new("Alice Kim".to_string(), Role::Engineering, 12.0);
    let bob = TeamMember::new("Bob Martinez".to_string(), Role::Engineering, 12.0);
    let carol = TeamMember::new("Carol Smith".to_string(), Role::Science, 6.0);
    let dave = TeamMember::new("Dave Roberts".to_string(), Role::Engineering, 12.0);

    let alice_id = alice.id;
    let bob_id = bob.id;
    let carol_id = carol.id;
    let dave_id = dave.id;

    plan.team_members.push(alice);
    plan.team_members.push(bob);
    plan.team_members.push(carol);
    plan.team_members.push(dave);

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

    plan.roadmap_projects.push(platform_project);
    plan.roadmap_projects.push(payment_project);
    plan.roadmap_projects.push(data_project);

    // Create technical projects
    let auth_service = TechnicalProject::new(
        "Auth Service Refactor".to_string(),
        Some(platform_roadmap_id),
        6.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let payment_api = TechnicalProject::new(
        "Payment API Integration".to_string(),
        Some(payment_roadmap_id),
        8.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let ml_pipeline = TechnicalProject::new(
        "ML Pipeline Optimization".to_string(),
        Some(platform_roadmap_id),
        12.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let data_pipeline = TechnicalProject::new(
        "Data Pipeline Migration".to_string(),
        Some(data_roadmap_id),
        10.0,
        NaiveDate::from_ymd_opt(2025, 1, 20).unwrap(),
    );

    let research = TechnicalProject::new(
        "Algorithm Research".to_string(),
        Some(data_roadmap_id),
        6.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let oncall = TechnicalProject::new(
        "Oncall".to_string(),
        None, // Not linked to a roadmap project
        0.0,
        NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
    );

    let auth_tech_id = auth_service.id;
    let payment_tech_id = payment_api.id;
    let ml_tech_id = ml_pipeline.id;
    let data_pipe_tech_id = data_pipeline.id;
    let research_tech_id = research.id;
    let _oncall_tech_id = oncall.id;

    plan.technical_projects.push(auth_service);
    plan.technical_projects.push(payment_api);
    plan.technical_projects.push(ml_pipeline);
    plan.technical_projects.push(data_pipeline);
    plan.technical_projects.push(research);
    plan.technical_projects.push(oncall);

    // Create sample allocations for first few weeks
    // Alice: Payment API for weeks 1-3, then switches to split allocation
    for week_num in 0..3 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(alice_id, week_start);
        alloc
            .assignments
            .push(Assignment::new(payment_tech_id, 100.0));
        plan.allocations.push(alloc);
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
    plan.allocations.push(alice_split);

    // Bob: ML Pipeline for weeks 1-4
    for week_num in 0..4 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(bob_id, week_start);
        if week_num == 2 {
            // Oncall in week 3
            alloc.assignments.push(Assignment::oncall());
        } else {
            alloc.assignments.push(Assignment::new(ml_tech_id, 100.0));
        }
        plan.allocations.push(alloc);
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
        plan.allocations.push(alloc);
    }

    // Dave: Auth Service for weeks 1-3
    for week_num in 0..3 {
        let week_start = quarter_start + chrono::Duration::weeks(week_num);
        let mut alloc = Allocation::new(dave_id, week_start);
        alloc.assignments.push(Assignment::new(auth_tech_id, 100.0));
        plan.allocations.push(alloc);
    }

    plan
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
            if assignment.is_oncall {
                continue; // Skip oncall
            }

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
