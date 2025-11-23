# Quarterly Planner - Development Roadmap

This roadmap outlines the path to v1.0 release. Completed milestones are summarized below; detailed implementation notes are in git history.

## Progress Status

**Current Status:** Milestone 8 Complete (8 of 16 milestones)

- ✅ Milestones 1-8 Complete (see summary below)
- 📋 **Milestones 9-16**: Planned for v1.0

**Build Status:** ✅ Compiles successfully

---

## Product Vision & 1.0 Goals

**Core Purpose:** Enable engineering managers to plan quarterly resource allocation through an intuitive, interactive interface with three coordinated views (Roadmap, Technical Projects, Allocation Grid).

**1.0 Success Criteria:**
- ✅ Visual allocation grid with paintbrush mode and keyboard shortcuts
- 🎯 Full CRUD operations for roadmap projects, technical projects, and team members
- 🎯 Two-signal state architecture (persistent preferences + exportable plan state)
- 🎯 Self-contained plan export/import for sharing and versioning
- 🎯 Inline validation and user feedback for allocation health
- 🎯 Undo/redo for all operations
- 🎯 Comprehensive keyboard navigation and accessibility
- 🎯 Production-ready polish and documentation

**Post-1.0 Vision:**
- Multi-team aggregation (Sr Manager view across multiple teams)
- Cloud sync and real-time collaboration
- Advanced reporting and analytics

---

## Completed Milestones (1-8) - Summary

### Milestone 1-4: Foundation & Core Views ✅
**Key Deliverables:**
- Project setup with Dioxus 0.7, design system (theme.css), component library
- Three main views: Roadmap, Technical Projects, Allocation Grid
- Static data display with mock data, view switching via TopNav
- CSS design tokens, spacing system, color palette

### Milestone 5: Technical Projects View ✅
**Key Deliverables:**
- Side panel with filters (All, On Track, At Risk, No Roadmap Link)
- Sorting options (Roadmap Project, Status, Allocation)
- Search functionality
- Project status badges with capacity calculations

### Milestone 6: Interactive Allocation Grid (Paintbrush Mode) ✅
**Key Deliverables:**
- Paintbrush mode for rapid weekly allocation
- Project selector with color-coded projects
- Cell click to allocate, state management for allocations
- Visual feedback (success pulse, project colors)

### Milestone 6.5: Grid Layout Rotation ✅
**Key Deliverables:**
- Rotated grid: engineers as columns, weeks as rows
- Vertical scrolling UX (more natural for 13+ weeks)
- Sticky headers (engineer headers top, week headers left)
- Horizontal sprint separators every 2 weeks

### Milestone 6.6: UI/UX Refinements (Floating FAB) ✅
**Key Deliverables:**
- Floating Action Button (FAB) in bottom-right for paintbrush mode
- Slide-out project selector panel (replaces horizontal bar)
- Reclaimed 80-100px vertical space
- Collapse/expand panel functionality
- Search and filter in panel

### Milestone 7: Context Menu & Keyboard Shortcuts ✅
**Key Deliverables:**
- Right-click context menu (Assign Project, Split Allocation, Clear)
- Keyboard shortcuts: Copy (Cmd/Ctrl+C), Paste (Cmd/Ctrl+V), Delete/Backspace
- Keybindings help overlay (press `?`)
- Split allocation modal with live preview
- Removed oncall special handling (now regular project)

### Milestone 8: Tooltip System & Auto-Date Updates ✅
**Key Deliverables:**
- Hover tooltips for grid cells (project info, allocation %, status)
- Engineer header tooltips (capacity, current projects, utilization)
- 500ms hover delay, glassmorphism effect
- Auto-update project start/end dates based on allocations
- Sprint boundary calculation helpers

---

## Milestone 9: State Architecture Refactor
**Goal:** Split monolithic Plan state into two signals for optimal reactivity, persistence, and prepare for 1.0 export/import

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days (includes spike)

### Context

Currently, we use a single `Signal<Plan>` containing all state. This has several issues:

**Problem 1 - Reactivity**: Dioxus tracks changes at the signal level. When you write to `plan.write().team_members.push()`, Dioxus marks the ENTIRE signal as dirty, triggering re-renders for ALL components that call `plan()`, even if they only read allocations.

**Problem 2 - Persistence**: We need two different persistence strategies:
- **Team preferences** (persisted in localStorage between sessions)
- **Planning data** (exported/imported per quarter, shareable)

**Problem 3 - Export Portability**: Plans must be self-contained for sharing. If Alice exports her plan and sends it to Bob, he needs to see all team member names/roles/capacity - not just UUIDs.

**Solution**: Split into two independent signals:
- `preferences`: Team roster, sprint config (persisted localStorage, rarely changes)
- `plan_state`: Roadmap projects, technical projects, allocations (exported/imported, changes frequently)

### Design Decision: Two-Signal Architecture

**Before (Current - Milestone 8)**:
```rust
// Single signal - any change triggers all consumers
let plan = use_signal(|| Plan {
    team_members, roadmap_projects, technical_projects, allocations, ...
});

// Problem: This re-renders even when only allocations change
fn TeamMemberHeader(plan: Signal<Plan>) -> Element {
    let members = plan().team_members; // Re-renders on ANY plan change!
    ...
}
```

**After (Milestone 9)**:
```rust
// Two independent signals
let preferences = use_signal(|| Preferences {
    team_name,          // NEW: "Backend Team"
    team_members,       // Engineers/scientists roster
    sprint_anchor_date, // Global sprint start reference
    sprint_length_weeks,
    default_capacity,
});

let plan_state = use_signal(|| PlanState {
    quarter_name, quarter_start_date, num_weeks,
    roadmap_projects, technical_projects, allocations,
    metadata,  // version, created_at, modified_at
});

// Now: Only re-renders when team_members actually change
fn TeamMemberHeader(preferences: Signal<Preferences>) -> Element {
    let members = preferences().team_members; // Isolated reactivity!
    ...
}
```

**Self-Contained Export Format**:
```rust
// Export combines both signals + includes team snapshot
pub struct PlanExport {
    pub version: String,  // "1.0"
    pub metadata: PlanMetadata,

    // TEAM CONTEXT (snapshot at export time)
    pub team_name: String,              // "Backend Team"
    pub team_members: Vec<TeamMember>,  // Full roster for portability

    // PLANNING DATA
    pub quarter_name: String,
    pub quarter_start_date: NaiveDate,
    pub num_weeks: usize,
    pub roadmap_projects: Vec<RoadmapProject>,
    pub technical_projects: Vec<TechnicalProject>,
    pub allocations: Vec<Allocation>,
}
```

**Why This Works**:
- ✅ **1.0 Sharing**: Alice exports → Bob imports and sees all names/roles correctly
- ✅ **1.0 Persistence**: Team roster persists in localStorage, plans are exported
- ✅ **Post-1.0 Multi-Team**: Sr Managers can load multiple team plans and aggregate (see Post-1.0 section)

### Tasks

#### 9.1: Spike - Storage & Serialization Strategy
**Duration:** 2-4 hours

**Research:**
1. **Dioxus storage plugins**
   - Evaluate `dioxus-storage` crate for web localStorage
   - Test cross-platform compatibility (web + desktop)
   - Determine if we need custom wrapper for desktop

2. **Serialization format**
   - JSON (human-readable, debuggable) - RECOMMENDED for v1.0
   - MessagePack (smaller, faster) - consider for post-1.0

3. **File vs Base64 for plan export**
   - File I/O: Simpler UX, requires file system access
   - Base64: Copy/paste friendly, no file system needed
   - **Recommendation**: Support BOTH (file primary, base64 fallback)

**Deliverable**: ADR document (`docs/adrs/ADR-004-state-persistence.md`) with:
- Storage plugin choice (dioxus-storage vs custom)
- Serialization format (JSON for v1.0)
- Export strategy (both file + base64)
- Self-contained export rationale (portability + future aggregation)
- Migration plan for existing users (if applicable)

#### 9.2: Create Preferences Model
- **New file**: `src/models/preferences.rs`
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct Preferences {
      pub team_name: String,               // NEW: "Backend Team", "Data Science", etc.
      pub team_members: Vec<TeamMember>,   // Engineers/scientists roster
      pub sprint_anchor_date: NaiveDate,   // Global sprint start reference
      pub sprint_length_weeks: usize,      // Sprint duration (e.g., 2 weeks)
      pub default_capacity: f32,           // Default weeks per person
  }
  ```
- **Default values**:
  - `team_name`: "My Team" (user can edit in preferences)
  - `sprint_anchor_date`: Jan 1, 2024 (Monday)
  - `sprint_length_weeks`: 2
  - `default_capacity`: 12.0
- **Validation**: Capacity > 0, sprint length 1-4 weeks, team_name not empty

#### 9.3: Create PlanState Model
- **New file**: `src/models/plan_state.rs`
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct PlanState {
      pub quarter_name: String,            // "Q1 2025"
      pub quarter_start_date: NaiveDate,   // First Monday of quarter
      pub num_weeks: usize,                // Typically 13
      pub roadmap_projects: Vec<RoadmapProject>,
      pub technical_projects: Vec<TechnicalProject>,
      pub allocations: Vec<Allocation>,
      pub metadata: PlanMetadata,
  }

  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct PlanMetadata {
      pub version: String,          // "1.0"
      pub created_at: DateTime<Utc>,
      pub modified_at: DateTime<Utc>,
  }
  ```

#### 9.4: Create PlanExport Model
- **New file**: `src/models/plan_export.rs`
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct PlanExport {
      pub version: String,
      pub metadata: PlanMetadata,

      // Team context snapshot
      pub team_name: String,
      pub team_members: Vec<TeamMember>,

      // Planning data
      pub quarter_name: String,
      pub quarter_start_date: NaiveDate,
      pub num_weeks: usize,
      pub roadmap_projects: Vec<RoadmapProject>,
      pub technical_projects: Vec<TechnicalProject>,
      pub allocations: Vec<Allocation>,
  }

  impl PlanExport {
      pub fn from_signals(prefs: Preferences, state: PlanState) -> Self {
          // Combine preferences + plan_state into self-contained export
      }
  }
  ```

#### 9.5: Refactor Plan Model (Keep for Backward Compatibility)
- **Update** `src/models/plan.rs`:
  ```rust
  // Keep Plan struct as a temporary wrapper during migration
  pub struct Plan {
      pub preferences: Preferences,
      pub plan_state: PlanState,
  }

  impl Plan {
      // Keep ALL existing methods - forward to inner structs
      pub fn get_team_member(&self, id: &Uuid) -> Option<&TeamMember> {
          self.preferences.team_members.iter().find(|e| &e.id == id)
      }

      pub fn get_roadmap_project(&self, id: &Uuid) -> Option<&RoadmapProject> {
          self.plan_state.roadmap_projects.iter().find(|p| &p.id == id)
      }

      // ... all other methods stay the same, just delegate
  }
  ```
- **Note**: Plan struct can be deprecated in M10+ once all components use direct signal access

#### 9.6: Update State Management
- **Refactor** `src/state.rs`:
  ```rust
  // OLD: Single signal
  pub fn use_plan_state() -> Signal<Plan> { ... }

  // NEW: Two signals via context
  #[derive(Clone, Copy)]
  pub struct AppContext {
      pub preferences: Signal<Preferences>,
      pub plan_state: Signal<PlanState>,
  }

  pub fn use_preferences() -> Signal<Preferences> {
      use_context::<AppContext>().preferences
  }

  pub fn use_plan_state() -> Signal<PlanState> {
      use_context::<AppContext>().plan_state
  }
  ```
- **Update** `create_sample_plan()` to return `(Preferences, PlanState)` tuple
- **Update** `main.rs` app setup:
  ```rust
  fn App() -> Element {
      let (prefs, state) = create_sample_plan();
      use_context_provider(|| AppContext {
          preferences: Signal::new(prefs),
          plan_state: Signal::new(state),
      });
      ...
  }
  ```

#### 9.7: Update All Component State Access
**Affected Components** (~15 files):
- `allocation_view.rs`: Read both signals (team members + allocations)
- `roadmap_view.rs`: Read `plan_state` only (roadmap projects)
- `technical_view.rs`: Read `plan_state` only (technical projects)
- `top_nav.rs`: Read both signals (capacity calculations)
- `paintbrush.rs`: Mutate `plan_state` only
- `grid_helpers.rs`: Read both signals (cell variant calculation)
- All other views and components

**Pattern for Updates**:
```rust
// BEFORE (M8):
let plan = use_plan_state();
let members = plan().team_members;
plan.write().allocations.push(alloc);

// AFTER (M9):
let preferences = use_preferences();
let plan_state = use_plan_state();
let members = preferences().team_members;
plan_state.write().allocations.push(alloc);
```

**Estimated Effort**: ~4-6 hours (find/replace + manual verification)

#### 9.8: Implement Preferences Persistence
- **Add dependency**: `dioxus-storage` or implement custom localStorage wrapper
- **Auto-save preferences** on change (debounced 1 second):
  ```rust
  use_effect(move || {
      let prefs = preferences();
      save_to_localstorage("planner_preferences", &prefs);
  });
  ```
- **Load on startup**:
  ```rust
  fn App() -> Element {
      let saved_prefs = load_from_localstorage("planner_preferences")
          .unwrap_or_else(|| create_default_preferences());
      ...
  }
  ```
- **Migration logic**: First-time users get default sample data

#### 9.9: Update Sprint Calculation Logic
- **Refactor** `get_sprint_boundaries()` in `src/utils/date_helpers.rs`:
  ```rust
  // BEFORE: Uses quarter_start_date (quarter-relative)
  pub fn get_sprint_boundaries(
      week_start: NaiveDate,
      quarter_start: NaiveDate,  // ← Remove this
      sprint_length_weeks: usize,
  ) -> (NaiveDate, NaiveDate)

  // AFTER: Uses global sprint_anchor_date
  pub fn get_sprint_boundaries(
      week_start: NaiveDate,
      sprint_anchor_date: NaiveDate,  // ← New global anchor
      sprint_length_weeks: usize,
  ) -> (NaiveDate, NaiveDate) {
      // Calculate which sprint this week belongs to relative to anchor
      let days_since_anchor = (week_start - sprint_anchor_date).num_days();
      let weeks_since_anchor = days_since_anchor / 7;
      let sprint_index = weeks_since_anchor.div_floor(sprint_length_weeks as i64);

      let sprint_start = sprint_anchor_date + Duration::weeks(sprint_index * sprint_length_weeks as i64);
      let sprint_end = sprint_start + Duration::weeks(sprint_length_weeks as i64) - Duration::days(1);

      (sprint_start, sprint_end)
  }
  ```
- **Update all call sites** (~10 locations):
  - `allocation_view.rs`
  - `paintbrush.rs`
  - `plan.rs` (`update_technical_project_dates`)
- **Update unit tests** to use global anchor

### Acceptance Criteria
- ✅ ADR-004 documents storage and serialization decisions
- ✅ Preferences and PlanState models created with serde traits
- ✅ PlanExport model created (self-contained format)
- ✅ State split into two independent signals (preferences + plan_state)
- ✅ All components updated to use new two-signal pattern
- ✅ Preferences auto-save to localStorage on change
- ✅ Preferences load from localStorage on startup
- ✅ Sprint boundaries calculated from global anchor date
- ✅ All existing features work unchanged (allocation grid, paintbrush, etc.)
- ✅ Build passes (`dx build`)
- ✅ Tests pass (`cargo test`)
- ✅ No regressions in functionality

### Development Notes
- **Testing**: Update existing unit tests for new state structure as you refactor
- **Incremental Approach**: Refactor one component at a time, verify it works before moving to next
- **Backward Compatibility**: Keep `Plan` wrapper struct temporarily to minimize breaking changes
- **Why Two Signals (Not Three)**: VDOM diffing handles re-renders efficiently. Two signals match business domain (team vs plan) without over-engineering.

---

## Milestone 10: Roadmap Projects Management (CRUD Operations)
**Goal:** Enable full create, read, update, delete operations for roadmap projects

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Currently, roadmap projects are read-only with static sample data. Users need to add, edit, and remove roadmap projects to plan their quarter effectively.

**State Access Pattern** (after M9 refactor):
```rust
let plan_state = use_plan_state();
plan_state.write().roadmap_projects.push(new_project);
```

### Tasks

#### 10.1: Add New Roadmap Project
- **Create "Add Roadmap Project" modal**
  - Trigger: "+ New Roadmap Project" button in RoadmapView
  - Form fields: Project name, Eng estimate, Sci estimate, Start date, Launch date, Notes (optional), Color picker
  - **Inline validation**: Required fields (show error below field), dates (start < launch), estimates > 0
  - Cancel/Save buttons
- **Wire up to state management**
  - Call `plan_state.write().roadmap_projects.push(new_project)`
  - Close modal on save
  - Update RoadmapView table reactively
- **Success feedback**: Modal closes, new row appears in table (no toast needed)

#### 10.2: Edit Existing Roadmap Project
- **Modal editing** (simpler than inline)
  - Click "Edit" button (gear icon) in row
  - Reuse Add Project modal, pre-filled with current values
  - Update project in `plan_state.write().roadmap_projects`
- **Inline validation**
  - Same validation as Add
  - Prevent invalid states (empty name, negative estimates)
  - Show errors below fields, not in toasts

#### 10.3: Delete Roadmap Project
- **Add delete button** (trash icon) to each row
- **Confirmation dialog**
  - "Delete [Project Name]?"
  - "This will unlink X technical projects. Continue?"
  - Cancel/Delete buttons
- **Cascade behavior**
  - Unlink all technical projects (`roadmap_project_id = None`)
  - Do NOT delete technical projects or allocations
  - Show warning if technical projects will be orphaned
- **State update**
  - Remove from `plan_state.write().roadmap_projects`
  - Update dependent views
- **Success feedback**: Row animates out, table updates (no toast)

#### 10.4: Color Picker Component
- **Create reusable ColorPicker component** (`src/components/ui/color_picker.rs`)
  - Display all 9 ProjectColor variants as swatches
  - Selected state (border highlight)
  - Accessible (keyboard navigable, ARIA labels)
- **Integration**
  - Use in Add/Edit Roadmap Project modal
  - Default to Blue if not selected

### Acceptance Criteria
- ✅ User can add new roadmap projects via modal
- ✅ User can edit roadmap projects
- ✅ User can delete roadmap projects with confirmation
- ✅ Inline validation prevents invalid data (errors shown below fields)
- ✅ Technical projects are unlinked (not deleted) when roadmap project is removed
- ✅ All changes immediately reflect in RoadmapView table
- ✅ State persists during session (pre-export)
- ✅ Build passes (`dx build`)

### Design References
- Modal styling: `docs/ui-design.md` Section 5.4 (Split Allocation Modal)
- Form inputs: `docs/component-reference.md` Section 4 (Input)
- Color picker: `docs/ui-design.md` Section 2 (Project Color Palette)
- Validation: Inline errors (not toasts) for accessibility

### Development Notes
- **Testing**: Write unit tests for validation logic as you implement
- **No Toasts**: Use modal close + table updates for success, inline errors for validation failures

---

## Milestone 11: Technical Projects Management (CRUD Operations)
**Goal:** Enable full create, read, update, delete operations for technical projects

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Technical projects are the implementation work linked to roadmap projects. Users need full CRUD to plan detailed work breakdown. The floating FAB exists but doesn't function yet.

**State Access Pattern**:
```rust
let plan_state = use_plan_state();
plan_state.write().technical_projects.push(new_project);
```

### Tasks

#### 11.1: Add New Technical Project
- **Wire up Floating FAB**
  - Currently displays but doesn't open modal
  - Click should open "Add Technical Project" modal
- **Create "Add Technical Project" modal**
  - Form fields: Project name, Link to roadmap project (dropdown), Estimated weeks, Start date (optional - auto-calculated), Expected completion (optional - auto-calculated), Notes (optional)
  - Roadmap project dropdown: All roadmap projects + "None" option
  - Color is inherited from linked roadmap project (display-only, not editable)
  - **Inline validation**: Required fields (name, estimate), estimate > 0
  - **Note**: Start/end dates auto-update when allocations are created (from M8)
- **State update**
  - Add to `plan_state.write().technical_projects`
  - Update TechnicalView and grid project selector

#### 11.2: Edit Existing Technical Project
- **Click project row in TechnicalView to edit**
  - Open modal pre-filled with current values
  - Reuse Add Project modal component
  - Allow editing all fields including roadmap link
- **Inline validation**
  - Same as Add
  - Warn if unlinking from roadmap project
- **State update**
  - Update in `plan_state.write().technical_projects`
  - Update project color in grid if roadmap link changes

#### 11.3: Delete Technical Project
- **Add delete button** to TechnicalView rows
- **Confirmation dialog**
  - "Delete [Project Name]?"
  - "This will remove X weeks of allocations. Continue?"
  - Show allocated weeks count
- **Cascade behavior**
  - Remove all allocations referencing this project
  - Clear from `plan_state.write().allocations`
  - Update team member utilization calculations
- **State update**
  - Remove from `plan_state.write().technical_projects`
  - Remove from grid project selector

#### 11.4: Link/Unlink Roadmap Project
- **Dropdown in edit modal**
  - Current roadmap project (if any)
  - All available roadmap projects
  - "None" option to unlink
- **Visual feedback**
  - Color changes in TechnicalView and grid when link changes
  - Roadmap project column updates immediately

### Acceptance Criteria
- ✅ Floating FAB opens "Add Technical Project" modal
- ✅ User can create technical projects with roadmap links
- ✅ User can edit technical projects
- ✅ User can delete technical projects with cascade to allocations
- ✅ User can link/unlink roadmap projects
- ✅ Project colors update when roadmap link changes
- ✅ Inline validation prevents invalid data
- ✅ Allocated weeks recalculate correctly after deletions
- ✅ Build passes (`dx build`)

### Design References
- Modal: `docs/ui-design.md` Section 5.4
- Dropdown: `docs/component-reference.md` Section 5 (Dropdown)

### Development Notes
- **Testing**: Write unit tests for cascade delete logic
- **Auto-Dates**: Leverage M8's `update_technical_project_dates()` - dates auto-update on first allocation

---

## Milestone 12: Team Members Management (CRUD Operations)
**Goal:** Enable adding, editing, and removing team members

**Status:** 📋 Not Started
**Estimated Effort:** 2 days

### Context
Team members (engineers/scientists) are currently static. Users need to customize their team roster and capacity for their organization.

**State Access Pattern**:
```rust
let preferences = use_preferences();
preferences.write().team_members.push(new_member);
```

### Tasks

#### 12.1: Add Team Member
- **Create "+ Add Team Member" button** in TopNav or settings area
- **Add Team Member modal**
  - Form fields: Name, Role (Eng/Sci radio buttons), Capacity (weeks, default from preferences)
  - **Inline validation**: Required fields, capacity > 0
  - Cancel/Save buttons
- **State update**
  - Add to `preferences.write().team_members`
  - New column appears in AllocationView grid
  - Updates capacity calculations in TopNav

#### 12.2: Edit Team Member
- **Click team member header in grid to edit**
  - Open modal pre-filled with current values
  - Allow editing name, role, capacity
  - Show **inline warning** if changing role affects allocations
- **Capacity change handling**
  - If reducing capacity below allocated weeks, show **inline warning**
  - Allow override (user may want to see over-allocation)
  - Update utilization badge color

#### 12.3: Delete Team Member
- **Add delete option** (right-click context menu on header, or settings panel)
- **Confirmation dialog**
  - "Delete [Engineer Name]?"
  - "This will remove X weeks of allocations. Continue?"
  - Show all projects currently assigned
- **Cascade behavior**
  - Remove all allocations for this team member
  - Update project allocated weeks
  - Remove column from grid

#### 12.4: Reorder Team Members (Optional)
- **Drag-and-drop team member columns** in grid
  - Visual feedback during drag
  - Persist order in `preferences.team_members` array
- **Or manual sort buttons** (up/down arrows)

### Acceptance Criteria
- ✅ User can add new team members
- ✅ User can edit team member details (name, role, capacity)
- ✅ User can delete team members with cascade to allocations
- ✅ Capacity changes trigger utilization recalculation
- ✅ Grid updates immediately (new column, removed column)
- ✅ TopNav capacity indicator updates
- ✅ Changes persist in localStorage (from M9)
- ✅ Build passes (`dx build`)

### Design References
- Modal: `docs/ui-design.md` Section 5.4
- Grid headers: `docs/ui-design.md` Section 5.3

### Development Notes
- **Testing**: Write unit tests for cascade delete logic
- **Persistence**: Changes auto-save to localStorage via M9's effect hook

---

## Milestone 13: Plan Import/Export
**Goal:** Enable users to save, load, and share self-contained quarterly plans

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
With CRUD operations complete, users can now export/import plans for:
- **1.0**: Sharing with colleagues for review, versioning, archiving past quarters
- **Post-1.0**: Multi-team aggregation (Sr Managers load multiple team plans)

**Critical Design**: Exports must be **self-contained** (include team snapshot) so they're portable and support future aggregation.

### Self-Contained Export Format

```rust
// From M9: PlanExport model
pub struct PlanExport {
    pub version: String,  // "1.0"
    pub metadata: PlanMetadata,

    // TEAM CONTEXT (snapshot at export time)
    pub team_name: String,              // "Backend Team"
    pub team_members: Vec<TeamMember>,  // Full roster for portability

    // PLANNING DATA
    pub quarter_name: String,
    pub quarter_start_date: NaiveDate,
    pub num_weeks: usize,
    pub roadmap_projects: Vec<RoadmapProject>,
    pub technical_projects: Vec<TechnicalProject>,
    pub allocations: Vec<Allocation>,
}
```

**Why Self-Contained**:
- ✅ **Portability**: Bob can import Alice's plan without having her team in his preferences
- ✅ **Future Aggregation**: Sr Manager can load multiple team plans and group by `team_name`
- ✅ **Audit Trail**: Historical record of who was on the team during that quarter

### Tasks

#### 13.1: Export Plan (Download File)
- **File menu: "Export Plan"** (Cmd+E)
  - Combine `preferences()` + `plan_state()` into `PlanExport`
  - Serialize to JSON
  - Trigger browser download: `plan-{team_name}-{quarter}-{date}.json`
  - File name format: `plan-backend-q1-2025-01-15.json`
- **Success feedback**: Modal closes OR inline message "Plan exported" (no toast)
- **Error handling**
  - Serialization errors: Show inline message with details
  - Log error details to console

#### 13.2: Import Plan (Upload File)
- **File menu: "Import Plan"** (Cmd+Shift+I)
  - Open file picker (browser input or native dialog)
  - Validate JSON structure
  - Confirm if current plan has unsaved changes
- **Team Mismatch Handling**:
  ```rust
  if export.team_name != local_preferences().team_name {
      // Show modal:
      // "This plan is for team 'Backend' but your preferences are for 'Frontend'"
      // Options:
      // - View Read-Only (use imported team context, don't modify preferences)
      // - Merge Teams (update local preferences.team_members with imported roster)
      // - Cancel Import
  }
  ```
- **Import Modes**:
  - **Replace**: Overwrite all plan_state (default)
  - **Read-Only** (if team mismatch): Display using imported team context
- **Validation**
  - Check version compatibility
  - Validate UUIDs don't conflict
  - Ensure technical projects reference valid roadmap projects
- **Error handling**
  - Invalid JSON: "File is not a valid plan"
  - Version mismatch: "Plan was created with version X, this app supports version Y"
  - Corrupted data: "Plan file is corrupted: [specific error]"

#### 13.3: Copy/Paste Plan (Base64)
- **File menu: "Copy Plan to Clipboard"**
  - Serialize PlanExport to JSON
  - Encode as base64
  - Copy to clipboard
  - Show inline message: "Plan copied to clipboard"
- **File menu: "Paste Plan from Clipboard"**
  - Paste base64 string from clipboard (or text input modal)
  - Decode and parse JSON
  - Same validation and team mismatch handling as Import
  - Replace current plan
- **Use cases**
  - Share via Slack/email
  - Quick backup without file system access
  - Cross-platform transfer

#### 13.4: Recent Plans List (Optional)
- **File menu: "Recent Plans" submenu**
  - Store last 5 imported plans in localStorage (metadata only)
  - Show: team name, quarter, date, quick load
  - "Clear Recent" option

### Acceptance Criteria
- ✅ User can export plan as self-contained JSON file
- ✅ User can import plan from JSON file
- ✅ User can copy plan as base64 to clipboard
- ✅ User can paste plan from clipboard
- ✅ Exports include team snapshot (name + full roster)
- ✅ Import handles team mismatch (read-only or merge options)
- ✅ Validation prevents corrupted or incompatible plans
- ✅ Confirmation dialog prevents accidental data loss
- ✅ Error messages are user-friendly and inline (not toasts)
- ✅ File names include team name, quarter, and date
- ✅ Build passes (`dx build`)

### Design References
- File menu: `docs/ui-design.md` Section 9.1
- Export format: `src/models/plan_export.rs` (from M9)
- Team mismatch modal: New modal component

### Development Notes
- **Testing**: Write integration tests for export → import round-trip
- **Future-Proofing**: Self-contained format enables post-1.0 multi-team aggregation (see Post-1.0 section)

---

## Milestone 14: Validation & User Feedback
**Goal:** Add comprehensive inline validation, warnings, and visual feedback

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Users need real-time feedback on allocation health, over/under-allocation, and data validity. This prevents errors and guides users to complete plans.

**Design Decision**: Use **inline validation and visual indicators** instead of toasts for better accessibility and usability.

### Tasks

#### 14.1: Inline Form Validation
- **Form fields validation** (used in M10-12 modals)
  - Required fields: Show error below field if empty on blur
  - Date ranges: Show error if start >= end
  - Positive numbers: Show error if estimates/capacity <= 0
  - Unique names: Show warning (not error) if duplicate detected
- **Validation component** (`src/components/ui/form_field.rs`)
  - Props: value, validation_fn, error_message
  - Displays error message below input with red text
  - ARIA live region for screen readers
- **Split allocation validation**
  - Percentages total 100%: Show error if not
  - No negative percentages: Show error on blur
  - Live preview updates as user types

#### 14.2: Over-Allocation Warnings
- **Real-time detection**
  - After each allocation change, check team member utilization
  - If allocated > capacity, show warning badge
- **Visual indicators**
  - Team member header: Red badge if over-allocated
  - Tooltip: "Alice is over-allocated by 2.5 weeks"
  - TopNav capacity bar: Red if total > capacity
- **Warnings panel** (optional)
  - List all over-allocated team members
  - Click to jump to that column in grid

#### 14.3: Under-Allocation Alerts
- **Detection**
  - If allocated < 90% of capacity, show warning
  - Configurable threshold in preferences
- **Visual indicators**
  - Team member header: Orange badge if under-allocated
  - Tooltip: "Bob has 3 unallocated weeks"
- **Suggestions** (in tooltip)
  - "Consider adding more work or oncall weeks"

#### 14.4: Project Progress Tracking
- **Visual indicators in TechnicalView**
  - Progress bar: allocated / estimated weeks
  - Color: Green (on track), Orange (at risk), Red (over)
- **Tooltips**
  - "Payment API: 4.5 / 8 weeks allocated (56%)"
  - "Still need 3.5 weeks"
- **Roadmap rollup**
  - Show aggregated progress for roadmap projects
  - "Q1 Platform: 18 / 24 engineering weeks allocated"

#### 14.5: Import Validation
- **Schema validation** (in M13 import)
  - Validate all required fields present
  - Check field types match expected
  - Show inline error: "Invalid plan file: missing 'allocations' field"
- **Referential integrity**
  - Validate all UUIDs reference existing entities
  - Show inline error: "Corrupted plan: allocation references unknown project"
- **Version compatibility**
  - Check version matches supported range
  - Show inline error: "Plan version 2.0 not supported, please upgrade app"

### Acceptance Criteria
- ✅ Form fields show inline errors below inputs (not toasts)
- ✅ Over-allocated team members show red badge indicators
- ✅ Under-allocated team members show orange badge indicators
- ✅ Project progress bars show allocated vs estimated
- ✅ Split allocation validates percentages total 100%
- ✅ Import validation catches corrupted files with specific error messages
- ✅ All warnings have clear, actionable messages in tooltips
- ✅ ARIA live regions announce validation errors for screen readers
- ✅ Build passes (`dx build`)

### Design References
- Form validation: Material Design / GitHub patterns (inline errors below fields)
- Status indicators: `docs/ui-design.md` Section 5.5
- Tooltips: Already implemented in M8

### Development Notes
- **Testing**: Write unit tests for all validation logic
- **Accessibility**: Use ARIA live regions for dynamic validation messages
- **No Toasts**: All feedback is inline or in tooltips (better for accessibility, prevents information loss)

---

## Milestone 15: Undo/Redo System
**Goal:** Add undo/redo functionality for allocations and CRUD operations

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Users make mistakes or want to try different allocation strategies. Undo/redo provides safety net and encourages experimentation.

**⚠️ Expected Refactor**: Milestones 9-14 use direct state mutations. M15 introduces command pattern, requiring refactoring ~15 mutation points. This is expected work (1 day) to enable undo/redo.

### Tasks

#### 15.1: Command Pattern Implementation
- **Create Command trait** (`src/commands/mod.rs`)
  ```rust
  pub trait Command: Send + Sync {
      fn execute(&self, ctx: &mut AppContext);
      fn undo(&self, ctx: &mut AppContext);
      fn description(&self) -> String;
  }
  ```
- **Implement commands** (1 per operation type):
  - `AddAllocationCommand`
  - `RemoveAllocationCommand`
  - `UpdateAllocationCommand`
  - `AddRoadmapProjectCommand`
  - `DeleteRoadmapProjectCommand`
  - `UpdateRoadmapProjectCommand`
  - `AddTechnicalProjectCommand`
  - `DeleteTechnicalProjectCommand`
  - `UpdateTechnicalProjectCommand`
  - `AddTeamMemberCommand`
  - `DeleteTeamMemberCommand`
  - `UpdateTeamMemberCommand`

#### 15.2: Undo/Redo Stack
- **Create history state** (`src/state/history.rs`)
  - Undo stack (`Vec<Box<dyn Command>>`)
  - Redo stack
  - Maximum history size (50 commands)
  - Add to AppContext: `pub history: Signal<History>`
- **State integration**
  - All mutations go through command pattern
  - Push to undo stack on execute
  - Clear redo stack on new command
- **Memory management**
  - Limit stack size to prevent memory leaks (drop oldest commands)

#### 15.3: Refactor Mutation Points
**Affected code locations** (~15 mutation points from M9-14):
- Paintbrush mode (M6): `allocate_project_to_cell()`
- Context menu (M7): Assign, Split, Clear
- Keyboard shortcuts (M7): Paste, Delete
- Roadmap CRUD (M10): Add, Edit, Delete projects
- Technical CRUD (M11): Add, Edit, Delete projects
- Team CRUD (M12): Add, Edit, Delete members

**Refactoring pattern**:
```rust
// BEFORE (M9-14 direct mutations):
plan_state.with_mut(|p| {
    p.roadmap_projects.push(new_project);
});

// AFTER (M15 command pattern):
let cmd = AddRoadmapProjectCommand::new(new_project);
history.write().execute(cmd, &mut plan_state);
```

**Estimated Effort**: 1 day (mechanical refactor with clear pattern)

#### 15.4: Keyboard Shortcuts
- **Undo: Cmd/Ctrl+Z**
  - Pop from undo stack
  - Call `command.undo(&mut ctx)`
  - Push to redo stack
  - Update UI reactively
- **Redo: Cmd/Ctrl+Shift+Z**
  - Pop from redo stack
  - Call `command.execute(&mut ctx)`
  - Push to undo stack
- **Visual feedback** (inline, not toast)
  - Update status bar: "Undid: Add allocation to Alice, Week 1"
  - Disabled state when stack is empty

#### 15.5: Undo/Redo UI
- **Edit menu** (or TopNav)
  - Undo button (grayed if stack empty)
  - Redo button (grayed if stack empty)
  - Show last action description on hover
- **History panel** (optional)
  - List last 10 commands
  - Click to undo/redo to that point
  - Clear history button

### Acceptance Criteria
- ✅ All state mutations use command pattern
- ✅ Cmd/Ctrl+Z undoes last action
- ✅ Cmd/Ctrl+Shift+Z redoes last undone action
- ✅ Undo/Redo buttons show correct enabled/disabled state
- ✅ Status bar shows action description (not toast)
- ✅ History limited to prevent memory issues
- ✅ Complex operations (delete project with cascade) undo correctly
- ✅ Undo works across all operation types (allocations, projects, team)
- ✅ Build passes (`dx build`)

### Design References
- Command pattern: Gang of Four design patterns
- History management: Git-like undo model

### Development Notes
- **Expected Refactor**: 1 day to migrate ~15 mutation points to command pattern
- **Testing**: Write unit tests for each command's execute/undo logic
- **Risk**: Low - mechanical refactor with clear pattern

---

## Milestone 16: Testing, Accessibility & Performance
**Goal:** Comprehensive testing, accessibility improvements, and performance optimization

**Status:** 📋 Not Started
**Estimated Effort:** 4-5 days

### Context
Ensure production-ready quality through testing, accessibility compliance, and performance tuning.

### Tasks

#### 16.1: Unit Tests
- **Test data models** (`src/models/`)
  - Plan calculations (allocated weeks, capacity)
  - Validation logic (all inline validators from M14)
  - Edge cases (empty allocations, split percentages)
  - Command execute/undo logic (from M15)
- **Test utilities** (`src/utils/`)
  - Date helpers (week calculations, sprint boundaries)
  - Capacity calculations
- **Coverage target:** 80%+ for models and utils

#### 16.2: Integration Tests
- **CRUD operations**
  - Add/edit/delete projects
  - Add/edit/delete team members
  - Allocation changes update calculations
- **State persistence**
  - Preferences save/load from localStorage
  - Plan export/import round-trip
- **Validation**
  - Invalid data rejected with correct error messages
  - Warnings trigger correctly (over/under allocation)
- **Undo/Redo**
  - Complex operations undo correctly
  - Redo after undo restores state

#### 16.3: Accessibility (A11y)
- **Keyboard navigation**
  - Tab order logical (top-nav → grid → modals)
  - All interactive elements focusable
  - Focus indicators visible (2px outline)
  - Skip links for large grids
- **Screen reader support**
  - ARIA labels for grid cells
  - Role attributes (grid, gridcell, row)
  - ARIA live regions for validation errors and status updates
  - Button labels clear and descriptive
- **Color contrast**
  - WCAG AA compliance (4.5:1 for text)
  - Audit all color combinations
  - Ensure warnings are distinguishable without color (icons + text)
- **Testing tools**
  - axe-core (accessibility linter)
  - Manual testing with screen reader (VoiceOver/NVDA)

#### 16.4: Performance Optimization
- **Grid rendering**
  - Measure render time (target <100ms for 10 engineers × 13 weeks)
  - Virtualize if >20 team members or >20 weeks
  - Memoize cell components if needed
  - Debounce paintbrush mode (16ms / 60fps)
- **State updates**
  - Verify two-signal architecture prevents unnecessary re-renders
  - Use browser DevTools to profile component updates
  - Use `use_memo` for expensive computed values
- **Bundle size**
  - Code splitting for modals (if needed)
  - Lazy load heavy components (if needed)
  - Target: <500KB initial bundle (web)
- **Benchmarking**
  - Lighthouse score 90+ (performance, accessibility)

#### 16.5: Cross-Platform Testing
- **Web browsers**
  - Chrome, Firefox, Safari, Edge
  - Mobile Safari, Chrome Android (read-only view)
- **Desktop**
  - macOS, Windows (if desktop build implemented)
- **Screen sizes**
  - 1920×1080 (primary)
  - 1440×900 (laptop)
  - 2560×1440 (4K scaled)

### Acceptance Criteria
- ✅ Unit test coverage >80% for models and utils
- ✅ Integration tests cover all CRUD operations
- ✅ Keyboard navigation works for all features
- ✅ Screen reader announces content correctly
- ✅ WCAG AA compliance verified
- ✅ Grid renders <100ms with 10 team members × 13 weeks
- ✅ Lighthouse performance score 90+
- ✅ Works on Chrome, Firefox, Safari, Edge
- ✅ Build passes on all platforms

### Design References
- Testing: `docs/testing.md` (to be created during this milestone)
- Accessibility: WCAG 2.1 AA guidelines
- Performance: Web Vitals metrics

### Development Notes
- **Incremental Testing**: Write tests as you implement features in M9-15 (not all at once)
- **Performance**: Two-signal architecture should prevent most unnecessary re-renders

---

## Milestone 17: Final Polish & Release Preparation
**Goal:** Final refinements, documentation, and v1.0 release

**Status:** 📋 Not Started
**Estimated Effort:** 3-4 days

### Context
Polish the UI, complete documentation, create demo materials, and prepare for v1.0 launch.

### Tasks

#### 17.1: UI/UX Refinements
- **Visual polish**
  - Consistent spacing (8px grid)
  - Hover states on all interactive elements
  - Smooth animations (250ms transitions)
  - Loading states for async operations (import/export)
- **Empty states**
  - No team members: "Add your first team member to get started"
  - No projects: "Create a roadmap project to begin planning"
  - No allocations: "Use paintbrush mode to allocate work"
- **Error states**
  - Import errors (invalid file, version mismatch)
  - Browser compatibility warnings (if needed)
- **Responsive behavior**
  - Grid scrolling (vertical for weeks, horizontal if many team members)
  - Modal sizing on small screens
  - Collapsible panels

#### 17.2: Documentation
- **User guide** (`docs/user-guide.md`)
  - Getting started (create team, add projects, allocate)
  - Core workflows (weekly planning, quarter planning)
  - Keyboard shortcuts reference
  - Export/import guide (sharing plans)
  - Tips and best practices
- **Developer documentation**
  - Architecture overview (two-signal pattern, command pattern)
  - Component reference (update existing)
  - State management patterns
  - Contributing guide
- **ADRs**
  - ADR-004: State persistence (from M9)
  - Any other missing ADRs
- **API documentation**
  - Rust docs (`cargo doc`)
  - Public APIs for models and state

#### 17.3: Example Data & Templates
- **Sample plans**
  - Q1 2025 example plan (10 engineers, 15 projects)
  - Q2 2025 template (empty but structured)
- **Starter templates**
  - Small team (3-5 engineers)
  - Large team (15+ engineers)
  - Science-heavy team
- **Include in distribution**
  - `/examples/` directory
  - Load from "File → Open Example"

#### 17.4: Demo & Marketing Materials
- **Demo video** (3-5 minutes)
  - Show core workflows (add team, create projects, allocate, export)
  - Highlight key features (paintbrush, undo/redo, validation)
  - Upload to YouTube, embed in README
- **Screenshots**
  - All three views (Roadmap, Technical, Allocation Grid)
  - Modals and tooltips
  - Before/after allocation examples
- **README**
  - Project description
  - Features list (with checkboxes)
  - Installation instructions
  - Quick start guide
  - Link to demo video
  - Contribution guidelines

#### 17.5: Release Preparation
- **Version bumping**
  - Update `Cargo.toml` to 1.0.0
  - Update all documentation versions
  - Tag git commit: `v1.0.0`
- **Changelog**
  - Complete CHANGELOG.md with all features
  - Group by milestone
  - Migration notes (from M9 state refactor)
- **Build artifacts**
  - Web: Build and deploy to GitHub Pages
  - Desktop: Package with `cargo bundle` (if applicable)
  - Test on fresh installations
- **Release notes**
  - GitHub release with changelog
  - Binaries attached (if desktop)
  - Link to demo and docs

### Acceptance Criteria
- ✅ All UI states are polished (empty, loading, error)
- ✅ User guide covers all core workflows
- ✅ Developer documentation is complete
- ✅ Example plans and templates included
- ✅ Demo video recorded and uploaded
- ✅ README is comprehensive and beginner-friendly
- ✅ v1.0.0 tag created in git
- ✅ Release artifacts built and tested
- ✅ GitHub release published

### Design References
- Empty states: `docs/ui-design.md` Section 10.2 (to be created)
- Documentation: `docs/` directory structure

---

## Post-1.0 Enhancements (Future Roadmap)

These features are out of scope for v1.0 but planned for future releases. Documenting now to ensure 1.0 architecture supports them.

### v1.1 - Quality of Life
- Light mode theme
- Grid cell drag & drop (reorder allocations)
- Batch operations (assign project to multiple weeks at once)
- Project templates (clone existing projects)
- Advanced search/filter in views
- Toast notification system (deferred from 1.0 due to accessibility concerns)

### v1.2 - Collaboration
- Export to Google Sheets / Excel
- Import from JIRA / GitHub Projects
- Share link (read-only view with URL)
- Comments and annotations on allocations

### v2.0 - Multi-Team Aggregation ⭐

**Goal**: Sr Managers can load multiple team plans and view organization-level capacity.

**Enabled by 1.0 Self-Contained Exports**: No data model changes needed! The `PlanExport` format from M9/M13 already includes everything required.

#### Architecture

**Load Multiple Plans**:
```rust
// Load multiple plan files (each team's export)
let plans: Vec<PlanExport> = vec![
    load_plan("backend-q1-2025.json"),
    load_plan("frontend-q1-2025.json"),
    load_plan("data-science-q1-2025.json"),
];
```

**Aggregate by Team**:
```rust
pub struct OrgView {
    pub teams: Vec<TeamSummary>,     // One per loaded plan
    pub weeks: Vec<NaiveDate>,       // Union of all weeks
    pub allocations: OrgAllocationGrid,
}

pub struct TeamSummary {
    pub team_name: String,           // From PlanExport.team_name
    pub total_capacity: f32,         // Sum of all team_members.capacity
    pub total_allocated: f32,        // Sum of all allocations
    pub member_count: usize,
}

pub struct OrgAllocationGrid {
    // (team_name, week_start) -> ProjectBreakdown
    cells: HashMap<(String, NaiveDate), ProjectBreakdown>,
}

pub struct ProjectBreakdown {
    // project_name -> allocated weeks
    projects: HashMap<String, f32>,
}
```

#### UI View

**Grid Layout**:
- **Columns**: Team names (from `PlanExport.team_name`)
- **Rows**: Weeks (union of all quarters - may span Q1-Q4)
- **Cells**: Project allocation breakdown

**Example Cell Content**:
```
Backend Team | Week of Jan 6
──────────────────────────────
Q1 Platform:     3.5 weeks (35%)
Payment API:     2.0 weeks (20%)
ML Pipeline:     1.5 weeks (15%)
Oncall:          1.0 weeks (10%)
──────────────────────────────
Total:           8.0 / 10.0 weeks (80%)
```

**Interactions**:
- Click cell → Drill down to see which engineers allocated to each project
- Click team → View full team plan (switch to single-team view)
- Filter by project name → See which teams are working on it
- Export org view → Combined report (CSV/PDF)

#### Aggregation Logic

```rust
impl OrgView {
    pub fn from_plans(plans: Vec<PlanExport>) -> Self {
        let mut org_allocations = HashMap::new();

        for plan in plans {
            let team_name = plan.team_name;

            // Aggregate allocations by (team, week)
            for allocation in plan.allocations {
                let week = allocation.week_start_date;
                let key = (team_name.clone(), week);

                // Find which project(s) this allocation belongs to
                for assignment in allocation.assignments {
                    let project = plan.technical_projects.iter()
                        .find(|p| p.id == assignment.technical_project_id);

                    let project_name = project.map(|p| p.name.clone())
                        .unwrap_or("Unknown".to_string());

                    let weeks = assignment.percentage / 100.0;

                    // Add to breakdown
                    org_allocations.entry(key)
                        .or_insert_with(ProjectBreakdown::default)
                        .add(project_name, weeks);
                }
            }
        }

        OrgView { /* ... */ }
    }
}
```

#### Benefits of 1.0 Self-Contained Format

✅ **No Format Changes**: `PlanExport` already has `team_name` and full `team_members`
✅ **No Server Required**: Pure client-side aggregation (load files from disk)
✅ **Flexible**: Can load any combination of teams (backend + frontend, or all 10 teams)
✅ **Historical**: Can compare Q1 vs Q2 by loading both quarters
✅ **Portable**: Works exactly like 1.0 import (same validation, same code)

#### Implementation Plan (v2.0)

**M2.0.1: Org View UI**
- New "Org View" mode (toggle from single-team mode)
- Grid with teams as columns, weeks as rows
- Load multiple plan files dialog

**M2.0.2: Aggregation Logic**
- Implement `OrgView::from_plans()`
- Project breakdown calculation
- Team summary calculations (capacity, utilization)

**M2.0.3: Drill-Down & Filtering**
- Click cell → Show team member breakdown
- Filter by project name
- Search across all teams

**M2.0.4: Org-Level Export**
- Export aggregated view as CSV
- Export as PDF report
- Quarterly comparison reports

**No Breaking Changes**: All 1.0 plans load in 2.0 without conversion!

### v2.1 - Advanced Planning
- Project dependencies (Gantt chart view)
- Resource contention detection (same person on multiple teams)
- What-if scenarios (sandbox mode)
- Monte Carlo simulation for estimates

### v3.0 - Enterprise Features
- Real-time collaboration (multiplayer editing)
- Cloud sync (Firebase, Supabase, or custom backend)
- Role-based access control (viewer, editor, admin)
- Audit log (who changed what, when)
- Integration with HR systems (org chart, time-off)
- Advanced analytics and reporting
- Mobile app (React Native or native)

---

## Development Guidelines

### Milestone Execution Process

1. **Before Starting**:
   - Read milestone context and design decisions
   - Review referenced documentation
   - Ensure all dependencies complete

2. **During Development**:
   - Work incrementally (one task at a time)
   - Write unit tests as you implement (don't defer to M16)
   - Test manually after each task
   - Update documentation if patterns change

3. **After Completion**:
   - Verify all acceptance criteria met
   - Run full build (`dx build`)
   - Run tests (`cargo test`)
   - Manual smoke test (all views, key workflows)
   - Update roadmap status

### Code Quality Standards

- **Inline validation** for user errors (not toasts)
- **Visual feedback** through UI updates (modal close, row animation)
- **Accessibility-first**: ARIA labels, keyboard navigation, screen reader support
- **Performance**: Profile before optimizing (VDOM handles most cases)
- **Testing**: 80%+ coverage for models/utils, integration tests for CRUD

### State Management Patterns

**After M9** (two-signal architecture):
```rust
// Read preferences
let preferences = use_preferences();
let team_name = preferences().team_name;

// Read plan state
let plan_state = use_plan_state();
let projects = plan_state().roadmap_projects;

// Mutate plan state
plan_state.write().allocations.push(new_allocation);

// Mutate preferences (rare)
preferences.write().team_members.push(new_member);
```

**After M15** (command pattern):
```rust
let history = use_history();
let cmd = AddAllocationCommand::new(...);
history.write().execute(cmd);
```

### Validation Patterns

**Inline validation** (not toasts):
```rust
// In modal/form component
let error = use_signal(|| None::<String>);

// Validate on blur
oninput: move |e| {
    let value = e.value();
    if value.is_empty() {
        error.set(Some("Project name is required".to_string()));
    } else {
        error.set(None);
    }
}

// Display error below field
if let Some(msg) = error() {
    div { class: "error-message", "{msg}" }
}
```

---

## Appendix: ADR-004 Outline

**Title**: State Architecture, Persistence, and Export Format

**Status**: Proposed (to be written in M9)

**Context**:
- Single signal causes unnecessary re-renders
- Need two persistence strategies (team config vs plans)
- Plans must be portable for sharing and future aggregation

**Decision**:
1. **Two-Signal Architecture**
   - `preferences`: Team roster, sprint config (localStorage)
   - `plan_state`: Projects, allocations (exported)

2. **Self-Contained Export Format**
   - Include team snapshot (name + full roster)
   - Enables portability (share with colleagues)
   - Enables future multi-team aggregation (no format changes)

3. **Storage Strategy**
   - Preferences: `dioxus-storage` + localStorage
   - Plans: JSON file export + base64 clipboard

**Consequences**:
- ✅ Isolated reactivity (team changes don't trigger plan re-renders)
- ✅ Natural persistence model (aligns with product requirements)
- ✅ Portable exports (self-contained, no external dependencies)
- ✅ Future-proof (v2.0 multi-team aggregation works out-of-box)
- ⚠️ Slightly more complex state access (two signals vs one)
- ⚠️ Export files larger (include team snapshot), but human-readable JSON

**Alternatives Considered**:
1. **Three signals** (prefs + projects + allocations):
   - Rejected: Adds complexity without solving real problem (VDOM already optimizes)

2. **Server-side state**:
   - Deferred to post-1.0: Want client-side-first for simplicity

3. **HashMap for allocations**:
   - Deferred: O(n) vs O(1) negligible at 130 allocations, Vec is simpler

**References**:
- Dioxus signals documentation
- WCAG accessibility guidelines (informed "no toasts" decision)
- Post-1.0 multi-team aggregation design

---

## Version History

- **2025-01-XX**: Roadmap created (M1-8 complete)
- **2025-01-XX**: M9-17 planned, two-signal architecture designed
- **2025-01-XX**: Self-contained export format designed for v2.0 multi-team
- **2025-01-XX**: Toast system removed from 1.0 (accessibility + usability concerns)
