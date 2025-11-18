# State Management Guide

This guide provides practical patterns for working with application state in the Quarterly Planner.

## Overview

The application uses a single global `Signal<Plan>` for state management, accessed through the `use_plan_state()` hook.

**Key Principle:** All components access state through `use_plan_state()` and Dioxus automatically handles reactivity.

## Architecture Decision

See [ADR-002: State Management](./adrs/ADR-002-state-management.md) for the rationale behind using Dioxus Signals.

## Basic Usage

### Reading State

```rust
use dioxus::prelude::*;
use crate::state::use_plan_state;

#[component]
pub fn EngineerList() -> Element {
    // Get signal reference
    let plan = use_plan_state();

    // Get immutable snapshot of current state
    let plan_data = plan();

    rsx! {
        div {
            for engineer in &plan_data.engineers {
                div { "{engineer.name}" }
            }
        }
    }
}
```

**Important:**
- `plan()` returns a read guard that dereferences to `&Plan`
- The component will automatically re-render when the accessed state changes
- Multiple calls to `plan()` in the same component are fine

### Writing State

```rust
use dioxus::prelude::*;
use crate::state::use_plan_state;
use crate::models::{Engineer, Role};

#[component]
pub fn AddEngineerButton() -> Element {
    let mut plan = use_plan_state();

    rsx! {
        button {
            onclick: move |_| {
                // Get write access to state
                let new_engineer = Engineer {
                    id: format!("eng-{}", uuid::Uuid::new_v4()),
                    name: "New Engineer".to_string(),
                    role: Role::Engineer,
                    capacity: 12.0,
                };

                // Modify state - this triggers reactivity
                plan.write().engineers.push(new_engineer);
            },
            "Add Engineer"
        }
    }
}
```

**Key Points:**
- Use `plan.write()` to get mutable access
- Changes made in `write()` automatically trigger re-renders
- Write guard is dropped at end of scope, triggering updates

## Common Patterns

### Pattern 1: Display Computed Values

```rust
#[component]
pub fn CapacitySummary() -> Element {
    let plan = use_plan_state();
    let plan_data = plan();

    // Call computed methods on Plan
    let total_capacity = plan_data.calculate_total_capacity();
    let total_allocated = plan_data.calculate_total_allocated();
    let utilization = (total_allocated / total_capacity * 100.0).min(100.0);

    rsx! {
        div {
            "Capacity: {total_allocated} / {total_capacity} weeks"
            "Utilization: {utilization:.0}%"
        }
    }
}
```

**Why this works:**
- Computed values are calculated on-demand from current state
- No need to store derived state separately
- Always guaranteed to be consistent with source data

### Pattern 2: Update Specific Item

```rust
#[component]
pub fn EngineerCapacityEditor(engineer_id: String) -> Element {
    let mut plan = use_plan_state();
    let plan_data = plan();

    // Find engineer in current state
    let engineer = plan_data.engineers.iter()
        .find(|e| e.id == engineer_id)
        .cloned();

    let Some(engineer_data) = engineer else {
        return rsx! { div { "Engineer not found" } };
    };

    rsx! {
        div {
            "Capacity: {engineer_data.capacity}"
            button {
                onclick: move |_| {
                    // Update specific field
                    if let Some(eng) = plan.write().engineers.iter_mut()
                        .find(|e| e.id == engineer_id)
                    {
                        eng.capacity += 1.0;
                    }
                },
                "Increase Capacity"
            }
        }
    }
}
```

**Key Points:**
- Read state first to display current values
- Use `iter_mut()` in write block to update specific items
- Component re-renders when state changes

### Pattern 3: Batch Updates

```rust
fn allocate_project_to_weeks(
    plan: &mut Signal<Plan>,
    engineer_id: &str,
    project_id: &str,
    week_dates: Vec<NaiveDate>,
) {
    let mut plan_write = plan.write();

    // Make all changes in a single write block
    for week_date in week_dates {
        let allocation = Allocation {
            engineer_id: engineer_id.to_string(),
            week_start_date: week_date,
            assignments: vec![Assignment {
                technical_project_id: project_id.to_string(),
                percentage: 100.0,
            }],
        };

        plan_write.allocations.push(allocation);
    }

    // Drop write guard here - single re-render triggered
}
```

**Best Practice:**
- Group related state changes in one `write()` block
- Reduces number of re-renders
- Maintains data consistency

### Pattern 4: Remove Items

```rust
#[component]
pub fn DeleteEngineerButton(engineer_id: String) -> Element {
    let mut plan = use_plan_state();

    rsx! {
        button {
            onclick: move |_| {
                plan.write().engineers.retain(|e| e.id != engineer_id);

                // Also remove related allocations
                plan.write().allocations.retain(|a| a.engineer_id != engineer_id);
            },
            "Delete Engineer"
        }
    }
}
```

**Important:**
- Consider cascading deletes for referential integrity
- Use `retain()` for efficient filtering
- Multiple `write()` calls are fine for logical grouping

### Pattern 5: Conditional Updates

```rust
fn toggle_oncall(plan: &mut Signal<Plan>, engineer_id: &str, week: NaiveDate) {
    let mut plan_write = plan.write();

    // Find existing allocation
    if let Some(alloc) = plan_write.allocations.iter_mut()
        .find(|a| a.engineer_id == engineer_id && a.week_start_date == week)
    {
        // Toggle oncall status
        alloc.assignments.clear();
        alloc.assignments.push(Assignment {
            technical_project_id: "oncall".to_string(),
            percentage: 100.0,
        });
    } else {
        // Create new oncall allocation
        plan_write.allocations.push(Allocation {
            engineer_id: engineer_id.to_string(),
            week_start_date: week,
            assignments: vec![Assignment {
                technical_project_id: "oncall".to_string(),
                percentage: 100.0,
            }],
        });
    }
}
```

## Computed Values

### Where to Put Computed Values

Computed values should be implemented as methods on the `Plan` struct:

```rust
// src/models/plan.rs
impl Plan {
    /// Calculate total allocated weeks for an engineer across the quarter
    pub fn calculate_allocated_weeks(&self, engineer_id: &str) -> f32 {
        self.allocations.iter()
            .filter(|a| a.engineer_id == engineer_id)
            .flat_map(|a| &a.assignments)
            .map(|a| a.percentage / 100.0)
            .sum()
    }

    /// Get technical project by ID
    pub fn get_technical_project(&self, id: &str) -> Option<&TechnicalProject> {
        self.technical_projects.iter().find(|p| p.id == id)
    }
}
```

**Benefits:**
- Computed values are always up-to-date
- No risk of stale derived state
- Easy to test independently
- Clearly documents what can be computed from state

### Caching Computed Values (Future)

If performance becomes an issue with large datasets:

```rust
use dioxus::prelude::*;

#[component]
pub fn CapacitySummary() -> Element {
    let plan = use_plan_state();

    // Use Memo to cache expensive computations
    let total_capacity = use_memo(move || {
        plan().calculate_total_capacity()
    });

    rsx! {
        div { "Total Capacity: {total_capacity}" }
    }
}
```

**Note:** Only add memoization when profiling shows it's needed.

## State Validation

### Validation on Update

Validation should happen before state updates:

```rust
fn set_allocation_percentage(
    plan: &mut Signal<Plan>,
    engineer_id: &str,
    week: NaiveDate,
    project_id: &str,
    percentage: f32,
) -> Result<(), String> {
    // Validate before updating
    if percentage < 0.0 || percentage > 100.0 {
        return Err("Percentage must be between 0 and 100".to_string());
    }

    let mut plan_write = plan.write();

    // Find and update allocation
    if let Some(alloc) = plan_write.allocations.iter_mut()
        .find(|a| a.engineer_id == engineer_id && a.week_start_date == week)
    {
        if let Some(assignment) = alloc.assignments.iter_mut()
            .find(|a| a.technical_project_id == project_id)
        {
            assignment.percentage = percentage;
            return Ok(());
        }
    }

    Err("Allocation not found".to_string())
}
```

### Validation Methods on Plan

```rust
impl Plan {
    /// Validate that all allocations for a week sum to <= 100%
    pub fn validate_week_allocation(&self, engineer_id: &str, week: NaiveDate) -> Result<(), String> {
        let total: f32 = self.allocations.iter()
            .filter(|a| a.engineer_id == engineer_id && a.week_start_date == week)
            .flat_map(|a| &a.assignments)
            .map(|a| a.percentage)
            .sum();

        if total > 100.0 {
            Err(format!("Week allocation exceeds 100%: {:.1}%", total))
        } else {
            Ok(())
        }
    }
}
```

## Performance Considerations

### Dioxus Reactivity is Fine-Grained

Dioxus tracks which parts of state each component accesses:

```rust
// This component only re-renders when engineers change
#[component]
pub fn EngineerCount() -> Element {
    let plan = use_plan_state();
    let count = plan().engineers.len();

    rsx! { div { "Engineers: {count}" } }
}
```

**You don't need to:**
- Manually optimize which components subscribe to what
- Split state unnecessarily
- Add memoization prematurely

**You should:**
- Profile before optimizing
- Trust Dioxus's reactivity system
- Only split state if measurements show it's needed

### When to Consider State Splitting

Consider multiple signals if:
- Profiling shows performance issues
- Updates to unrelated data are causing re-renders
- You have truly independent state domains

Example:
```rust
// Only if performance profiling shows this is needed
use_context_provider(|| Signal::new(plan_data));
use_context_provider(|| Signal::new(ui_state));  // Separate UI state
```

## Testing State

### Unit Testing Computed Methods

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_allocated_weeks() {
        let mut plan = Plan::default();
        plan.engineers.push(Engineer {
            id: "eng-1".to_string(),
            name: "Alice".to_string(),
            role: Role::Engineer,
            capacity: 12.0,
        });

        plan.allocations.push(Allocation {
            engineer_id: "eng-1".to_string(),
            week_start_date: NaiveDate::from_ymd_opt(2025, 1, 6).unwrap(),
            assignments: vec![Assignment {
                technical_project_id: "proj-1".to_string(),
                percentage: 50.0,
            }],
        });

        let allocated = plan.calculate_allocated_weeks("eng-1");
        assert_eq!(allocated, 0.5);
    }
}
```

### Integration Testing with Signals (Future)

For testing components that use state:

```rust
#[cfg(test)]
mod tests {
    use dioxus::prelude::*;

    #[test]
    fn test_engineer_list() {
        let mut vdom = VirtualDom::new_with_props(
            EngineerList,
            EngineerListProps { /* ... */ }
        );

        // Test rendering and interactions
        vdom.rebuild();
        // Assert on output
    }
}
```

## Common Pitfalls

### ❌ Don't: Hold Long-Lived Read Guards

```rust
// BAD: Holding read guard across event handlers
let plan_data = plan();
rsx! {
    button {
        onclick: move |_| {
            // This won't work - plan_data is read-only
            // plan_data.engineers.push(...); // Error!
        }
    }
}
```

### ✅ Do: Get Fresh References

```rust
// GOOD: Get write access when needed
let mut plan = use_plan_state();
rsx! {
    button {
        onclick: move |_| {
            plan.write().engineers.push(...);  // Works!
        }
    }
}
```

### ❌ Don't: Clone Entire State Unnecessarily

```rust
// BAD: Cloning entire state
let plan_clone = plan().clone();
// Work with plan_clone...
```

### ✅ Do: Clone Only What You Need

```rust
// GOOD: Clone specific items
let engineers = plan().engineers.clone();
// Work with engineers...
```

## Future Enhancements

### Undo/Redo (Phase 12)

The current state architecture supports undo/redo through the command pattern:

```rust
trait Command {
    fn execute(&self, plan: &mut Plan);
    fn undo(&self, plan: &mut Plan);
}

// Usage:
let command = AllocateCommand { /* ... */ };
command.execute(&mut plan.write());
history.push(command);
```

See Phase 12 in the roadmap for details.

### Auto-Save (Future)

```rust
// Pseudo-code for future auto-save
use_effect(move || {
    let plan_snapshot = plan().clone();
    spawn(async move {
        save_to_file(&plan_snapshot).await;
    });
});
```

## References

- [ADR-002: State Management](./adrs/ADR-002-state-management.md)
- Data models: `src/models/plan.rs`
- State hook: `src/state.rs`
- Dioxus Signals: https://dioxuslabs.com/learn/0.7/reference/hooks
