# Allocation Grid Architecture

This document describes the technical implementation of the Allocation Grid, the core interface for viewing and editing engineer time allocations.

## Overview

The Allocation Grid displays a two-dimensional view of engineer allocations across weeks in a quarter:
- **Columns**: Weeks in the quarter (typically 13 weeks)
- **Rows**: Engineers in the organization
- **Cells**: Allocation state for a specific engineer and week

## Architecture Decision

See [ADR-003: Grid Layout](./adrs/ADR-003-grid-layout.md) for the rationale behind horizontal timeline orientation and CSS Grid usage.

## Component Structure

```
AllocationView
├── allocation-grid-container (scroll container)
└── allocation-grid (CSS Grid)
    ├── grid-header-corner (sticky top-left)
    ├── grid-week-header × N (sticky top, one per week)
    └── For each engineer:
        ├── grid-row-header (sticky left)
        └── GridCell × N (one per week)
```

## Grid Rendering Pipeline

### Step 1: Generate Week Data

```rust
// src/components/views/allocation_view.rs
let weeks = generate_quarter_weeks(plan_data.quarter_start_date, plan_data.weeks_in_quarter);
```

The `generate_quarter_weeks` utility creates a `Vec<WeekInfo>` containing:
- `start_date`: Monday of the week
- `sprint_number`: Calculated based on 2-week sprints
- `week_number`: Sequential week number (1-13)
- Helper methods: `is_sprint_start()`, `format_date()`, `format_week_number()`

See `src/utils/date_helpers.rs` for implementation.

### Step 2: Calculate Dynamic Grid Columns

```rust
let grid_template_columns = format!("180px repeat({}, 120px)", weeks.len());
```

This generates CSS like:
```css
grid-template-columns: 180px repeat(13, 120px);
/*                     ^row     ^week columns  */
/*                     header                   */
```

### Step 3: Render Grid Structure

#### 3.1 Corner Cell

```rust
div { class: "grid-header-corner" }
```

Empty cell at top-left (sticky to both axes with `z-index: 3`).

#### 3.2 Column Headers (Weeks)

```rust
for week in weeks.iter() {
    let is_sprint_start = week.is_sprint_start();
    let sprint_class = if is_sprint_start {
        "grid-week-header sprint-separator"
    } else {
        "grid-week-header"
    };

    rsx! {
        div { class: "{sprint_class}",
            if is_sprint_start {
                div { class: "sprint-label", "Sprint {week.sprint_number}" }
            }
            div { class: "week-date", "{week.format_date(true)}" }
            div { class: "week-progress", "{week.format_week_number()}" }
        }
    }
}
```

Each week header shows:
- Sprint number (only on first week of 2-week sprint)
- Week date (e.g., "Jan 6")
- Week progress (e.g., "Wk 1/13")

Sprint separators add a vertical dashed line via CSS.

#### 3.3 Engineer Rows

For each engineer, render:

**Row Header:**
```rust
div { class: "grid-row-header",
    div { class: "engineer-info",
        div { class: "engineer-name-row",
            span { class: "engineer-name", "{engineer.name}" }
            span { class: "role-badge", "{engineer.role.short_name()}" }
        }
        div { class: "capacity-row",
            span {
                class: "capacity-text capacity-{capacity_status}",
                "{allocated} / {capacity} weeks"
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
```

**Capacity Status:**
- `success`: within 0.5 weeks of capacity
- `warning`: 0.5-1 week off
- `error`: >1 week off

**Grid Cells:**
```rust
for week in &weeks {
    // 1. Find allocation for this engineer and week
    let allocation = plan_data.allocations.iter()
        .find(|a| a.engineer_id == engineer_id && a.week_start_date == week.start_date);

    // 2. Determine cell variant based on allocation
    let variant = determine_cell_variant(allocation, week, &plan_data);

    // 3. Render cell
    rsx! {
        GridCell { variant }
    }
}
```

## Cell State Machine

Each grid cell can be in one of several states, represented by `GridCellVariant`:

```rust
pub enum GridCellVariant {
    Empty,
    SingleWeek { project_name, project_color, percentage, is_before_start },
    MultiWeek { project_name, project_color, percentage, position, duration_weeks, is_before_start },
    Split { project1_name, project1_color, project1_percentage, project2_name, project2_color, project2_percentage },
    Oncall,
}
```

### Cell Variant Determination Logic

```rust
let variant = if let Some(alloc) = allocation {
    if alloc.is_oncall() {
        GridCellVariant::Oncall
    } else if alloc.assignments.len() == 2 {
        // Split allocation
        let assignment1 = &alloc.assignments[0];
        let assignment2 = &alloc.assignments[1];
        // Lookup projects and create Split variant
    } else if let Some(assignment) = alloc.assignments.first() {
        // Single project allocation
        let project = plan_data.get_technical_project(&assignment.technical_project_id);
        let is_before_start = week.start_date < proj.start_date;

        // TODO: Detect multi-week series (deferred to Phase 6)
        GridCellVariant::SingleWeek {
            project_name: proj.name.clone(),
            project_color,
            percentage: assignment.percentage,
            is_before_start,
        }
    } else {
        GridCellVariant::Empty
    }
} else {
    GridCellVariant::Empty
};
```

**Key Points:**
- Oncall takes precedence (checked first)
- Two assignments = split allocation
- One assignment = single or multi-week (multi-week detection TODO)
- No allocation = empty cell

### Multi-Week Detection (Planned for Phase 6)

Algorithm for detecting connected multi-week projects:

```rust
// Pseudo-code for future implementation
fn detect_multi_week_series(
    allocations: &[Allocation],
    engineer_id: &str,
    weeks: &[WeekInfo],
) -> Vec<(Range<usize>, String)> {
    // 1. Group allocations by project
    // 2. For each project, find consecutive weeks
    // 3. If consecutive weeks > 1, mark as multi-week series
    // 4. Return vec of (week_range, project_id)
}
```

Then modify cell rendering:
```rust
if in_multi_week_series {
    let position = if first_in_series {
        CellPosition::First
    } else if last_in_series {
        CellPosition::Last
    } else {
        CellPosition::Middle
    };

    GridCellVariant::MultiWeek {
        position,
        duration_weeks: if position == CellPosition::Last {
            Some(series_length)
        } else {
            None
        },
        // ...
    }
}
```

## GridCell Component

The `GridCell` component is a pure presentation component that renders based on variant.

**Location:** `src/components/ui/grid_cell.rs`

### Props

```rust
#[component]
pub fn GridCell(
    variant: GridCellVariant,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element
```

**Note:** `onclick` is optional and will be used for editing in future phases.

### Rendering Logic

```rust
match variant {
    GridCellVariant::Empty => {
        // Dashed border, "+" icon
    }
    GridCellVariant::SingleWeek { project_name, project_color, percentage, is_before_start } => {
        // Solid background with project color
        // Project name + percentage badge
        // Optional before-start overlay
    }
    GridCellVariant::MultiWeek { position, duration_weeks, .. } => {
        // Rounded corners based on position
        // Duration badge on last cell
        // Percentage badge (except middle cells)
    }
    GridCellVariant::Split { project1_name, project2_name, .. } => {
        // Vertical split with two sections
        // Each section: name + percentage
    }
    GridCellVariant::Oncall => {
        // Purple background
        // Phone icon
        // "Oncall" text
    }
}
```

### CSS Class Strategy

Each variant applies specific classes:
- `.grid-cell` - Base class (all cells)
- `.grid-cell-empty` - Empty state
- `.grid-cell-allocated` - Has allocation
- `.grid-cell-multi` - Multi-week cell
- `.grid-cell-multi-first` - First in series
- `.grid-cell-multi-middle` - Middle of series
- `.grid-cell-multi-last` - Last in series
- `.grid-cell-split` - Split allocation
- `.grid-cell-oncall` - Oncall assignment
- `.grid-cell-before-start` - Before project start date

**CSS Custom Properties:**
```css
.grid-cell-allocated {
  background: var(--project-color);
}
```

The `--project-color` variable is set via inline styles:
```rust
style: "--project-color: {color_hex};"
```

## Sticky Positioning Strategy

### The Z-Index Tower

```
z-index: 3  ┌─────────────┐  Corner cell (overlaps everything)
            │             │
z-index: 2  ├─────────────┤  Column headers (weeks)
            │             │
z-index: 1  ├─────────────┤  Row headers (engineers)
            │             │
z-index: 0  └─────────────┘  Grid cells (base layer)
```

### CSS Implementation

```css
.grid-header-corner {
  position: sticky;
  top: 0;
  left: 0;
  z-index: 3;
  background: var(--color-background-secondary);
}

.grid-week-header {
  position: sticky;
  top: 0;
  z-index: 2;
  background: var(--color-background-secondary);
}

.grid-row-header {
  position: sticky;
  left: 0;
  z-index: 1;
  background: var(--color-background-primary);
}
```

**Why this works:**
- When scrolling horizontally: row headers stay left, column headers scroll up/down
- When scrolling vertically: column headers stay top, row headers scroll left/right
- Corner cell stays anchored to top-left at all times
- Z-index ensures proper overlap when scrolling both directions

## Sprint Separators

Sprint separators are vertical dashed lines that appear every 2 weeks.

### Detection

```rust
let is_sprint_start = week.is_sprint_start();
// Returns true if week_number is odd (1, 3, 5, 7, ...)
```

### Visual Implementation

```css
.sprint-separator {
  border-left: 2px dashed var(--color-border-emphasis);
  padding-left: 8px;
}
```

The border appears on the left edge of the first week of each sprint.

## Capacity Indicators

Each row header includes a capacity indicator showing:
- Allocated weeks / Total capacity
- Color-coded status (success/warning/error)
- Progress bar visualization

### Calculation

```rust
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
```

### Visual Representation

```
Alice  SW    ████████░░ 8.5 / 10 weeks
       ^     ^          ^
       |     |          └─ Text indicator
       |     └──────────── Progress bar
       └────────────────── Role badge
```

Progress bar width = `utilization_pct`% of container width.

## Performance Considerations

### Current Performance Profile

**Dataset size:**
- 13 weeks × 10 engineers = 130 cells ✅ Excellent
- 26 weeks × 20 engineers = 520 cells ✅ Good
- 52 weeks × 20 engineers = 1040 cells ✅ Acceptable

**Rendering:**
- All cells render on initial load
- Dioxus reactivity re-renders only affected cells on state changes
- CSS Grid handles layout (no JavaScript layout calculation)

### Optimization Strategies (If Needed)

If performance becomes an issue with large datasets:

1. **Virtual Scrolling for Rows:**
   ```rust
   // Only render visible engineers
   let visible_engineers = engineers[viewport_start..viewport_end];
   ```

2. **Memoize Cell Components:**
   ```rust
   let cell_memo = use_memo(move || {
       determine_cell_variant(allocation, week, plan_data)
   });
   ```

3. **Lazy Render Cells:**
   ```rust
   // Render cells outside viewport as simple placeholders
   if cell_in_viewport {
       rsx! { GridCell { variant } }
   } else {
       rsx! { div { class: "grid-cell-placeholder" } }
   }
   ```

4. **Canvas Rendering (Last Resort):**
   - Only for truly massive datasets (100+ engineers, 100+ weeks)
   - Loses DOM benefits (accessibility, tooltips, etc.)

**Rule:** Profile before optimizing. Current implementation is sufficient for expected use cases.

## Data Flow

### Read Path (Rendering)

```
Plan State (Signal)
  ↓
AllocationView reads via use_plan_state()
  ↓
For each engineer × week:
  ↓
Find allocation in plan_data.allocations
  ↓
Determine GridCellVariant
  ↓
Render GridCell component
  ↓
Display in CSS Grid
```

### Write Path (Future - Phase 6)

```
User clicks cell
  ↓
Determine action (allocate, clear, edit)
  ↓
Validate action
  ↓
Mutate plan state via plan.write()
  ↓
Dioxus reactivity triggers re-render of affected cells
  ↓
Grid updates automatically
```

## Testing Strategy

### Unit Tests

Test individual helper functions:
```rust
#[test]
fn test_generate_quarter_weeks() {
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 6).unwrap();
    let weeks = generate_quarter_weeks(start_date, 13);
    assert_eq!(weeks.len(), 13);
    assert_eq!(weeks[0].start_date, start_date);
    assert_eq!(weeks[0].sprint_number, 1);
}

#[test]
fn test_is_sprint_start() {
    // Week 1, 3, 5 should be sprint starts
    // Week 2, 4, 6 should not
}
```

### Integration Tests (Future)

Test full grid rendering:
```rust
#[test]
fn test_allocation_grid_renders() {
    // Create test plan with sample data
    // Render AllocationView
    // Assert grid structure is correct
}
```

### Manual Testing Checklist

- [ ] Grid renders with correct number of columns (weeks)
- [ ] Grid renders with correct number of rows (engineers)
- [ ] Week headers show sprint numbers, dates, progress
- [ ] Row headers show engineer names, roles, capacity
- [ ] Empty cells show dashed border and "+" icon
- [ ] Allocated cells show project name and color
- [ ] Split cells show vertical division
- [ ] Oncall cells show purple background and icon
- [ ] Sticky headers work on horizontal scroll
- [ ] Sticky headers work on vertical scroll
- [ ] Sprint separators appear every 2 weeks
- [ ] Capacity indicators show correct colors
- [ ] Progress bars show correct width

## Future Enhancements

### Phase 6: Interactive Editing
- Click to allocate
- Drag to paint
- Keyboard navigation

### Phase 7: Context Menu
- Right-click cell options
- Split allocation dialog

### Phase 8: Tooltips
- Hover to show project details
- Engineer capacity breakdown

### Phase 14: Animations
- Cell hover effects
- Success glow on allocation
- Drag preview

### Phase 18: Mobile Support
- Consider vertical timeline on small screens
- Simplified touch interface

## References

- [ADR-003: Grid Layout](./adrs/ADR-003-grid-layout.md)
- UI Design Spec: `docs/ui-design.md` (sections 5.3, 7.3)
- GridCell component: `src/components/ui/grid_cell.rs`
- AllocationView: `src/components/views/allocation_view.rs`
- Date utilities: `src/utils/date_helpers.rs`
- CSS implementation: `assets/styling/main.css`
