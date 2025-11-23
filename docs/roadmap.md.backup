# Quarterly Planner - Development Roadmap

This roadmap outlines the path to v1.0 release. Completed milestones are summarized below; detailed implementation notes are in git history.

## Progress Status

**Current Status:** Milestone 8 Complete (8 of 17 milestones)

- ✅ Milestones 1-8 Complete (see summary below)
- 📋 **Milestones 9-17**: Planned for v1.0

**Build Status:** ✅ Compiles successfully

---

## Product Vision & 1.0 Goals

**Core Purpose:** Enable engineering managers to plan quarterly resource allocation through an intuitive, interactive interface with three coordinated views (Roadmap, Technical Projects, Allocation Grid).

**1.0 Success Criteria:**
- ✅ Visual allocation grid with paintbrush mode and keyboard shortcuts
- 🎯 Full CRUD operations for roadmap projects, technical projects, and team members
- 🎯 Two-tier state management (persistent team preferences + exportable plans)
- 🎯 Plan import/export for sharing and versioning
- 🎯 Validation and user feedback for allocation health
- 🎯 Comprehensive keyboard navigation and accessibility
- 🎯 Production-ready polish and documentation

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

### Milestone 8: Tooltip System ✅
**Key Deliverables:**
- Hover tooltips for grid cells (project info, allocation %, status)
- Engineer header tooltips (capacity, current projects, utilization)
- 500ms hover delay, glassmorphism effect
- Arrow pointers, max-width 280px

---

## Milestone 9: State Architecture Refactor
**Goal:** Split monolithic Plan state into two signals for optimal reactivity and prepare for persistence

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days (includes spike)

### Context
Currently, we use a single `Signal<Plan>` containing all state. This causes unnecessary re-renders: changing team members triggers re-renders in allocation grid components, even though they only read plan data.

**Dioxus Reactivity Limitation**: Signals track changes at the signal level, not at individual field granularity. When you write to `plan.write().team_members.push()`, Dioxus marks the ENTIRE signal as dirty, triggering re-renders for ALL components that call `plan()`.

**Solution**: Split into two independent signals:
- `preferences`: Team roster, capacity defaults, sprint config (rarely changes, persisted in localStorage)
- `plan_state`: Roadmap projects, technical projects, allocations (changes frequently, exported/imported)

This architectural change:
- Eliminates cross-concern re-renders (changing team members won't re-render the allocation grid)
- Aligns with our two-tier persistence model (preferences persist between sessions, plans are exported/imported)
- Provides clean foundation for CRUD operations in M10-12

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
    team_members, sprint_anchor_date, sprint_length_weeks, default_capacity
});

let plan_state = use_signal(|| PlanState {
    quarter_name, quarter_start_date, num_weeks,
    roadmap_projects, technical_projects, allocations
});

// Now: Only re-renders when team_members actually change
fn TeamMemberHeader(preferences: Signal<Preferences>) -> Element {
    let members = preferences().team_members; // Isolated reactivity!
    ...
}
```

### Tasks

#### 18.1: Spike - Storage & Serialization Strategy
**Duration:** 4-6 hours

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
- Migration plan for existing users (if applicable)

#### 18.2: Create Preferences Model
- **New file**: `src/models/preferences.rs`
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct Preferences {
      pub team_members: Vec<TeamMember>,
      pub sprint_anchor_date: NaiveDate,  // Global sprint start reference
      pub sprint_length_weeks: usize,     // Sprint duration (e.g., 2 weeks)
      pub default_capacity: f32,          // Default weeks per person
  }
  ```
- **Default values**:
  - `sprint_anchor_date`: Jan 1, 2024 (Monday)
  - `sprint_length_weeks`: 2
  - `default_capacity`: 12.0
- **Validation**: Capacity > 0, sprint length 1-4 weeks

#### 18.3: Create PlanState Model
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
      pub metadata: PlanMetadata,          // created_at, modified_at
  }

  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  pub struct PlanMetadata {
      pub version: String,          // "1.0"
      pub created_at: DateTime<Utc>,
      pub modified_at: DateTime<Utc>,
  }
  ```

#### 18.4: Refactor Plan Model
- **Keep Plan struct** but make it a wrapper (for backward compatibility):
  ```rust
  // src/models/plan.rs
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
- **Update calculation methods**:
  - `calculate_total_capacity()` → reads `preferences.team_members`
  - `calculate_total_allocated()` → reads `plan_state.allocations`
  - `update_technical_project_dates()` → mutates `plan_state.technical_projects`

#### 18.5: Update State Management
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

#### 18.6: Update All Component State Access
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

**Estimated Effort**: ~4 hours (find/replace + manual verification)

#### 18.7: Implement Preferences Persistence
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

#### 18.8: Update Sprint Calculation Logic
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
- **Backward Compatibility**: Keep `Plan` wrapper struct to minimize breaking changes

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

#### 18.1: Add New Roadmap Project
- **Create "Add Roadmap Project" modal**
  - Trigger: "+ New Roadmap Project" button in RoadmapView
  - Form fields: Project name, Eng estimate, Sci estimate, Start date, Launch date, Notes (optional), Color picker
  - Validation: Required fields, dates (start < launch), estimates > 0
  - Cancel/Save buttons
- **Wire up to state management**
  - Call `plan.write().roadmap_projects.push(new_project)`
  - Close modal on save
  - Update RoadmapView table reactively

#### 18.2: Edit Existing Roadmap Project
- **Inline editing option**
  - Click project row to enter edit mode
  - Editable fields: All project properties
  - Save on Enter/blur, Cancel on Escape
- **OR modal editing (simpler implementation)**
  - Click "Edit" button (gear icon) in row
  - Reuse Add Project modal, pre-filled with current values
  - Update project in `plan.write().roadmap_projects`
- **Validation**
  - Same validation as Add
  - Prevent invalid states (empty name, negative estimates)

#### 18.3: Delete Roadmap Project
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
  - Remove from `plan.write().roadmap_projects`
  - Update dependent views

#### 18.4: Color Picker Component
- **Create reusable ColorPicker component** (`src/components/ui/color_picker.rs`)
  - Display all 9 ProjectColor variants as swatches
  - Selected state (border highlight)
  - Accessible (keyboard navigable, ARIA labels)
- **Integration**
  - Use in Add/Edit Roadmap Project modal
  - Default to Blue if not selected

### Acceptance Criteria
- ✅ User can add new roadmap projects via modal
- ✅ User can edit roadmap projects (inline or modal)
- ✅ User can delete roadmap projects with confirmation
- ✅ Validation prevents invalid data (empty names, bad dates, negative estimates)
- ✅ Technical projects are unlinked (not deleted) when roadmap project is removed
- ✅ All changes immediately reflect in RoadmapView table
- ✅ State persists during session (pre-export)
- ✅ Build passes (`dx build`)

### Design References
- Modal styling: `docs/ui-design.md` Section 5.4 (Split Allocation Modal)
- Form inputs: `docs/component-reference.md` Section 4 (Input)
- Color picker: `docs/ui-design.md` Section 2 (Project Color Palette)

---

## Milestone 10: Technical Projects Management (CRUD Operations)
**Goal:** Enable full create, read, update, delete operations for technical projects

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Technical projects are the implementation work linked to roadmap projects. Users need full CRUD to plan detailed work breakdown. The floating FAB exists but doesn't function yet.

### Tasks

#### 18.1: Add New Technical Project
- **Wire up Floating FAB**
  - Currently displays but doesn't open modal
  - Click should open "Add Technical Project" modal
- **Create "Add Technical Project" modal**
  - Form fields: Project name, Link to roadmap project (dropdown), Estimated weeks, Start date, Expected completion (optional), Notes (optional)
  - Roadmap project dropdown: All roadmap projects + "None" option
  - Color is inherited from linked roadmap project (display-only, not editable)
  - Validation: Required fields (name, estimate, start date), estimate > 0
- **State update**
  - Add to `plan.write().technical_projects`
  - Update TechnicalView and grid project selector

#### 18.2: Edit Existing Technical Project
- **Click project row in TechnicalView to edit**
  - Open modal pre-filled with current values
  - Reuse Add Project modal component
  - Allow editing all fields including roadmap link
- **Validation**
  - Same as Add
  - Warn if unlinking from roadmap project
- **State update**
  - Update in `plan.write().technical_projects`
  - Update project color in grid if roadmap link changes

#### 18.3: Delete Technical Project
- **Add delete button** to TechnicalView rows
- **Confirmation dialog**
  - "Delete [Project Name]?"
  - "This will remove X weeks of allocations. Continue?"
  - Show allocated weeks count
- **Cascade behavior**
  - Remove all allocations referencing this project
  - Clear from `plan.write().allocations`
  - Update team member utilization calculations
- **State update**
  - Remove from `plan.write().technical_projects`
  - Remove from grid project selector

#### 18.4: Link/Unlink Roadmap Project
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
- ✅ Validation prevents invalid data
- ✅ Allocated weeks recalculate correctly after deletions
- ✅ Build passes (`dx build`)

### Design References
- Modal: `docs/ui-design.md` Section 5.4
- Dropdown: `docs/component-reference.md` Section 5 (Dropdown)

---

## Milestone 11: Team Members Management (CRUD Operations)
**Goal:** Enable adding, editing, and removing team members

**Status:** 📋 Not Started
**Estimated Effort:** 2 days

### Context
Team members (engineers/scientists) are currently static. Users need to customize their team roster and capacity for their organization.

### Tasks

#### 18.1: Add Team Member
- **Create "+ Add Team Member" button** in TopNav or settings area
- **Add Team Member modal**
  - Form fields: Name, Role (Eng/Sci radio buttons), Capacity (weeks, default 12)
  - Validation: Required fields, capacity > 0
  - Cancel/Save buttons
- **State update**
  - Add to `plan.write().team_members`
  - New column appears in AllocationView grid
  - Updates capacity calculations in TopNav

#### 18.2: Edit Team Member
- **Click team member header in grid to edit**
  - Open modal pre-filled with current values
  - Allow editing name, role, capacity
  - Show warning if changing role affects allocations
- **Capacity change handling**
  - If reducing capacity below allocated weeks, show warning
  - Allow override (user may want to see over-allocation)
  - Update utilization badge color

#### 18.3: Delete Team Member
- **Add delete option** (right-click context menu on header, or settings panel)
- **Confirmation dialog**
  - "Delete [Engineer Name]?"
  - "This will remove X weeks of allocations. Continue?"
  - Show all projects currently assigned
- **Cascade behavior**
  - Remove all allocations for this team member
  - Update project allocated weeks
  - Remove column from grid

#### 18.4: Reorder Team Members (Optional)
- **Drag-and-drop team member columns** in grid
  - Visual feedback during drag
  - Persist order in team_members array
- **Or manual sort buttons** (up/down arrows)

### Acceptance Criteria
- ✅ User can add new team members
- ✅ User can edit team member details (name, role, capacity)
- ✅ User can delete team members with cascade to allocations
- ✅ Capacity changes trigger utilization recalculation
- ✅ Grid updates immediately (new column, removed column)
- ✅ TopNav capacity indicator updates
- ✅ Build passes (`dx build`)

### Design References
- Modal: `docs/ui-design.md` Section 5.4
- Grid headers: `docs/ui-design.md` Section 5.3

---

## Milestone 12: State Architecture & Persistence
**Goal:** Implement two-tier state system (persistent preferences + exportable plans)

**Status:** 📋 Not Started
**Estimated Effort:** 3-4 days (includes 1 day spike)

### Context
Currently, all state is ephemeral (lost on refresh). We need:
1. **Persistent preferences** (team roster, capacities) - survives sessions, stored locally
2. **Exportable plan state** (projects, allocations) - per planning session, shareable

This enables users to maintain their team config while creating multiple quarterly plans.

### Design Decision: Two-Tier State

**Tier 1: Preferences (Long-term, Persisted)**
```rust
pub struct Preferences {
    team_members: Vec<TeamMember>,      // Engineers/Scientists roster
    default_capacity: f32,               // Default weeks per person
    sprint_length_weeks: usize,          // Sprint duration (e.g., 2 weeks)
    sprint_anchor_date: NaiveDate,      // Global sprint start (e.g., Jan 1, 2024)
    quarter_config: QuarterConfig,       // Start dates, week count
}
```
- **Storage:** Browser localStorage (web) or platform preferences (desktop)
- **Persistence:** Auto-save on change, debounced 1 second
- **Dioxus Plugin:** `dioxus-storage` or `dioxus-sdk`

**Sprint Configuration Logic:**
- `sprint_anchor_date`: A reference date when Sprint 1 started (can be any date, even years ago)
- Sprint boundaries for any week are calculated by adding/subtracting `sprint_length_weeks` from the anchor
- Example: If anchor is Jan 1, 2024 and sprint length is 2 weeks:
  - Sprint 1: Jan 1-14, 2024
  - Sprint 2: Jan 15-28, 2024
  - Any week in Q1 2025 falls into a calculated sprint number relative to the anchor

**Tier 2: Plan State (Short-term, Exportable)**
```rust
pub struct PlanState {
    roadmap_projects: Vec<RoadmapProject>,
    technical_projects: Vec<TechnicalProject>,
    allocations: Vec<Allocation>,
    metadata: PlanMetadata,  // created_at, modified_at, author
}
```
- **Export Format:** JSON or Base64-encoded JSON
- **Import/Export:** Manual (file download/upload or copy/paste)
- **Versioning:** Plan version field for future compatibility

### Tasks

#### 18.1: Spike - Storage Options
**Duration:** 4-6 hours

**Research:**
1. **Dioxus storage plugins**
   - Evaluate `dioxus-storage` crate
   - Test browser localStorage integration
   - Test cross-platform compatibility (web + desktop)
2. **File vs Base64 for plans**
   - File I/O: Simpler UX, requires file system access
   - Base64: Copy/paste friendly, no file system needed
   - Recommendation: Support BOTH (file primary, base64 fallback)
3. **Serialization format**
   - JSON (human-readable, debuggable)
   - MessagePack (smaller, faster)
   - Recommendation: JSON for v1.0 (simplicity)

**Deliverable:** ADR document (`docs/adrs/ADR-004-state-persistence.md`) with recommendations

#### 18.2: Implement Preferences Persistence
- **Create Preferences struct** in `src/models/preferences.rs`
  - Extract team_members, quarter config from Plan
  - Add `sprint_anchor_date` field (global sprint start)
  - Add default values and validation
- **Integrate storage plugin**
  - Add `dioxus-storage` dependency (or write custom localStorage wrapper)
  - Create `use_preferences()` hook
  - Auto-save on team member changes
- **Migration logic**
  - First-time users: Initialize with sample data
  - Existing users: Load from storage, merge with defaults
- **Settings UI**
  - "Preferences" modal (accessed from TopNav gear icon)
  - Edit quarter config, sprint length, sprint anchor date, default capacity
  - Sprint anchor date picker with explanation
  - Preview: "Sprint N will run from X to Y" for current quarter
  - Reset to defaults button
- **Update sprint calculation logic**
  - Refactor `get_sprint_boundaries()` to use `sprint_anchor_date` instead of `quarter_start_date`
  - Calculate sprint number globally (not quarter-relative)

#### 18.3: Implement Plan State Serialization
- **Refactor Plan struct**
  - Split into Preferences + PlanState
  - Update all state access points
- **Serialization**
  - Implement `serde` serialize/deserialize for PlanState
  - Add version field ("1.0")
  - Add metadata (created_at, modified_at)
- **JSON format**
  - Follow schema from `docs/ui-design.md` Section 9.2
  - Validate on deserialize
- **Base64 encoding**
  - Encode JSON as base64 for copy/paste
  - Decode and validate on paste

#### 18.4: "New Plan" Functionality
- **File menu: "New Plan"** (Cmd+N)
  - Confirmation if unsaved changes
  - Clear all roadmap projects, technical projects, allocations
  - Keep team members (from preferences)
  - Reset to empty state

### Acceptance Criteria
- ✅ ADR-004 documents storage decisions
- ✅ Team members persist between sessions
- ✅ Preferences modal allows editing quarter config and defaults
- ✅ Plan state can be serialized to JSON
- ✅ "New Plan" clears project data but keeps team roster
- ✅ Changes auto-save to localStorage
- ✅ Build passes, no breaking changes to existing features

### Design References
- Preferences modal: `docs/ui-design.md` Section 9.3
- File format: `docs/ui-design.md` Section 9.2

---

## Milestone 13: Plan Import/Export
**Goal:** Enable users to save, load, and share quarterly plans

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
With CRUD operations and state serialization complete, users can now export/import plans for versioning, sharing with colleagues, or archiving past quarters.

### Tasks

#### 18.1: Export Plan (Download File)
- **File menu: "Export Plan"** (Cmd+E)
  - Serialize current PlanState to JSON
  - Trigger browser download: `plan-q1-2025.json`
  - File name format: `plan-{quarter}-{date}.json`
- **Success notification**
  - Toast: "Plan exported successfully"
- **Error handling**
  - Serialization errors: Show user-friendly message
  - Log error details to console

#### 18.2: Import Plan (Upload File)
- **File menu: "Import Plan"** (Cmd+Shift+I)
  - Open file picker (browser input or native dialog)
  - Validate JSON structure
  - Confirm if current plan has unsaved changes
- **Merge strategy**
  - Replace: Overwrite all plan state (default)
  - Merge: Add projects to existing plan (future enhancement)
- **Validation**
  - Check version compatibility
  - Validate UUIDs don't conflict
  - Ensure technical projects reference valid roadmap projects
- **Error handling**
  - Invalid JSON: "File is not a valid plan"
  - Version mismatch: "Plan was created with a different version"
  - Corrupted data: "Plan file is corrupted"

#### 18.3: Copy/Paste Plan (Base64)
- **File menu: "Copy Plan to Clipboard"**
  - Serialize PlanState to JSON
  - Encode as base64
  - Copy to clipboard
  - Toast: "Plan copied to clipboard"
- **File menu: "Paste Plan from Clipboard"**
  - Paste base64 string from clipboard (or text input modal)
  - Decode and parse JSON
  - Same validation as Import
  - Replace current plan
- **Use cases**
  - Share via Slack/email
  - Quick backup without file system access
  - Cross-platform transfer

#### 18.4: Recent Plans List (Optional)
- **File menu: "Recent Plans" submenu**
  - Store last 5 imported plans in localStorage
  - Show plan name, date, quick load
  - "Clear Recent" option

### Acceptance Criteria
- ✅ User can export plan as JSON file
- ✅ User can import plan from JSON file
- ✅ User can copy plan as base64 to clipboard
- ✅ User can paste plan from clipboard
- ✅ Validation prevents corrupted or incompatible plans
- ✅ Confirmation dialog prevents accidental data loss
- ✅ Error messages are user-friendly
- ✅ File names include quarter and date
- ✅ Build passes (`dx build`)

### Design References
- File menu: `docs/ui-design.md` Section 9.1
- Toast notifications: `docs/ui-design.md` Section 13 (to be created)

---

## Milestone 14: Validation & User Feedback
**Goal:** Add comprehensive validation, warnings, and user feedback system

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Users need real-time feedback on allocation health, over/under-allocation, and data validity. This prevents errors and guides users to complete plans.

### Tasks

#### 18.1: Toast Notification System
- **Create Toast component** (`src/components/ui/toast.rs`)
  - Position: Bottom-right, stacked
  - Variants: Success, Warning, Error, Info
  - Auto-dismiss after 3-5 seconds
  - Dismiss button (X)
  - Animation: Slide in from right, fade out
- **Toast queue management**
  - Global toast state (signal)
  - `show_toast(message, variant)` helper
  - Maximum 3 toasts visible at once

#### 18.2: Over-Allocation Warnings
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

#### 18.3: Under-Allocation Alerts
- **Detection**
  - If allocated < 90% of capacity, show warning
  - Configurable threshold in preferences
- **Visual indicators**
  - Team member header: Orange badge if under-allocated
  - Tooltip: "Bob has 3 unallocated weeks"
- **Suggestions**
  - "Consider adding more work or oncall weeks"

#### 18.4: Project Progress Tracking
- **Visual indicators in TechnicalView**
  - Progress bar: allocated / estimated weeks
  - Color: Green (on track), Orange (at risk), Red (over)
- **Tooltips**
  - "Payment API: 4.5 / 8 weeks allocated (56%)"
  - "Still need 3.5 weeks"
- **Roadmap rollup**
  - Show aggregated progress for roadmap projects
  - "Q1 Platform: 18 / 24 engineering weeks allocated"

#### 18.5: Data Validation
- **Form validation**
  - Required fields
  - Date ranges (start < end)
  - Positive numbers (estimates, capacity)
  - Unique names (warn on duplicates)
- **Allocation validation**
  - Split allocation totals 100%
  - No negative percentages
  - Project start date before allocations
- **Import validation**
  - Schema validation (correct fields)
  - Referential integrity (UUIDs exist)
  - Version compatibility

### Acceptance Criteria
- ✅ Toast notifications appear for user actions
- ✅ Over-allocated team members show red indicators
- ✅ Under-allocated team members show orange indicators
- ✅ Project progress bars show allocated vs estimated
- ✅ Form validation prevents invalid data entry
- ✅ Import validation catches corrupted files
- ✅ All warnings have clear, actionable messages
- ✅ Build passes (`dx build`)

### Design References
- Toast: `docs/ui-design.md` Section 13 (to be created)
- Validation: `docs/validation.md` (to be created)
- Status indicators: `docs/ui-design.md` Section 5.5

---

## Milestone 15: Undo/Redo System
**Goal:** Add undo/redo functionality for allocations and CRUD operations

**Status:** 📋 Not Started
**Estimated Effort:** 2-3 days

### Context
Users make mistakes or want to try different allocation strategies. Undo/redo provides safety net and encourages experimentation.

### Tasks

#### 18.1: Command Pattern Implementation
- **Create Command trait** (`src/commands/mod.rs`)
  ```rust
  pub trait Command {
      fn execute(&self, plan: &mut Plan);
      fn undo(&self, plan: &mut Plan);
      fn description(&self) -> String;
  }
  ```
- **Implement commands**
  - `AddAllocationCommand`
  - `RemoveAllocationCommand`
  - `UpdateAllocationCommand`
  - `AddProjectCommand`
  - `DeleteProjectCommand`
  - `UpdateProjectCommand`
  - `AddTeamMemberCommand`
  - etc.

#### 18.2: Undo/Redo Stack
- **Create history state** (`src/state/history.rs`)
  - Undo stack (Vec<Box<dyn Command>>)
  - Redo stack
  - Maximum history size (50 commands)
- **State integration**
  - All mutations go through command pattern
  - Push to undo stack on execute
  - Clear redo stack on new command
- **Memory management**
  - Limit stack size to prevent memory leaks
  - Serialize old commands if needed

#### 18.3: Keyboard Shortcuts
- **Undo: Cmd/Ctrl+Z**
  - Pop from undo stack
  - Call `command.undo(plan)`
  - Push to redo stack
  - Update UI reactively
- **Redo: Cmd/Ctrl+Shift+Z**
  - Pop from redo stack
  - Call `command.execute(plan)`
  - Push to undo stack
- **Visual feedback**
  - Toast: "Undid: Add allocation to Alice, Week 1"
  - Disabled state when stack is empty

#### 18.4: Undo/Redo UI
- **Edit menu** (or TopNav)
  - Undo button (grayed if stack empty)
  - Redo button (grayed if stack empty)
  - Show last action description
- **History panel** (optional)
  - List last 10 commands
  - Click to undo/redo to that point
  - Clear history button

### Acceptance Criteria
- ✅ All state mutations use command pattern
- ✅ Cmd/Ctrl+Z undoes last action
- ✅ Cmd/Ctrl+Shift+Z redoes last undone action
- ✅ Undo/Redo buttons show correct state
- ✅ Toast shows action description
- ✅ History limited to prevent memory issues
- ✅ Complex operations (delete project) undo correctly
- ✅ Build passes (`dx build`)

### Design References
- Command pattern: Gang of Four design patterns
- History management: Git-like undo model

---

## Milestone 16: Testing, Accessibility & Performance
**Goal:** Comprehensive testing, accessibility improvements, and performance optimization

**Status:** 📋 Not Started
**Estimated Effort:** 4-5 days

### Context
Ensure production-ready quality through testing, accessibility compliance, and performance tuning.

### Tasks

#### 18.1: Unit Tests
- **Test data models** (`src/models/`)
  - Plan calculations (allocated weeks, capacity)
  - Validation logic
  - Edge cases (empty allocations, split percentages)
- **Test utilities** (`src/utils/`)
  - Date helpers (week calculations, sprint boundaries)
  - Capacity calculations
- **Coverage target:** 80%+ for models and utils

#### 18.2: Integration Tests
- **CRUD operations**
  - Add/edit/delete projects
  - Add/edit/delete team members
  - Allocation changes update calculations
- **State persistence**
  - Preferences save/load
  - Plan export/import round-trip
- **Validation**
  - Invalid data rejected
  - Warnings trigger correctly

#### 18.3: Accessibility (A11y)
- **Keyboard navigation**
  - Tab order logical (top-nav → grid → modals)
  - All interactive elements focusable
  - Focus indicators visible (2px outline)
  - Skip links for large grids
- **Screen reader support**
  - ARIA labels for grid cells
  - Role attributes (grid, gridcell, row)
  - Live regions for toasts
  - Button labels clear
- **Color contrast**
  - WCAG AA compliance (4.5:1 for text)
  - Audit all color combinations
  - Pattern overlays for colorblind users
- **Testing tools**
  - axe-core (accessibility linter)
  - Manual testing with screen reader (VoiceOver/NVDA)

#### 18.4: Performance Optimization
- **Grid rendering**
  - Virtualize if >20 team members or >20 weeks
  - Memoize cell components
  - Debounce paintbrush mode (16ms / 60fps)
- **State updates**
  - Batch allocation changes
  - Avoid unnecessary re-renders
  - Use `use_memo` for computed values
- **Bundle size**
  - Code splitting for modals
  - Lazy load heavy components
  - Target: <500KB initial bundle (web)
- **Benchmarking**
  - Measure grid render time (target <100ms)
  - Lighthouse score 90+ (performance, accessibility)

#### 18.5: Cross-Platform Testing
- **Web browsers**
  - Chrome, Firefox, Safari, Edge
  - Mobile Safari, Chrome Android (read-only)
- **Desktop**
  - macOS, Windows, Linux (if Electron wrapper)
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
- Testing: `docs/testing.md` (to be created)
- Accessibility: WCAG 2.1 AA guidelines
- Performance: Web Vitals metrics

---

## Milestone 17: Final Polish & Release Preparation
**Goal:** Final refinements, documentation, and v1.0 release

**Status:** 📋 Not Started
**Estimated Effort:** 3-4 days

### Context
Polish the UI, complete documentation, create demo materials, and prepare for v1.0 launch.

### Tasks

#### 18.1: UI/UX Refinements
- **Visual polish**
  - Consistent spacing (8px grid)
  - Hover states on all interactive elements
  - Smooth animations (250ms transitions)
  - Loading states for async operations
- **Empty states**
  - No team members: "Add your first team member to get started"
  - No projects: "Create a roadmap project to begin planning"
  - No allocations: "Use paintbrush mode to allocate work"
- **Error states**
  - Network errors (import/export)
  - Invalid files
  - Browser compatibility warnings
- **Responsive behavior**
  - Grid scrolling (horizontal for many weeks)
  - Modal sizing on small screens
  - Collapsible panels

#### 18.2: Documentation
- **User guide** (`docs/user-guide.md`)
  - Getting started
  - Core workflows (add team, create projects, allocate)
  - Keyboard shortcuts reference
  - Tips and best practices
- **Developer documentation**
  - Architecture overview
  - Component reference (update existing)
  - State management patterns
  - Contributing guide
- **ADRs**
  - Complete any missing ADRs
  - ADR-004: State persistence (from Milestone 12)
- **API documentation**
  - Rust docs (`cargo doc`)
  - Public APIs for models and state

#### 18.3: Example Data & Templates
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

#### 18.4: Demo & Marketing Materials
- **Demo video** (3-5 minutes)
  - Show core workflows
  - Highlight key features
  - Upload to YouTube, embed in README
- **Screenshots**
  - All three views
  - Modals and tooltips
  - Before/after allocation examples
- **README**
  - Project description
  - Features list
  - Installation instructions
  - Quick start guide
  - Link to demo video
  - Contribution guidelines

#### 18.5: Release Preparation
- **Version bumping**
  - Update `Cargo.toml` to 1.0.0
  - Update all documentation versions
  - Tag git commit: `v1.0.0`
- **Changelog**
  - Complete CHANGELOG.md with all features
  - Group by phase
  - Migration notes (if any)
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
- Empty states: `docs/ui-design.md` Section 10.2
- Documentation: `docs/` directory structure

---

## Post-1.0 Enhancements (Future Roadmap)

These features are out of scope for v1.0 but planned for future releases:

### v1.1 - Quality of Life
- Light mode theme
- Grid cell drag & drop (reorder allocations)
- Batch operations (assign project to multiple weeks at once)
- Project templates (clone existing projects)
- Advanced search/filter in views

### v1.2 - Collaboration
- Export to Google Sheets / Excel
- Import from JIRA / GitHub Projects
- Share link (read-only view)
- Comments and annotations

### v1.3 - Advanced Planning
- Multiple teams support
- Project dependencies (Gantt chart view)
- Resource contention detection
- What-if scenarios (sandbox mode)

### v2.0 - Enterprise Features
- Real-time collaboration (multiplayer)
- Role-based access control
- Audit log
- Advanced analytics and reporting
- Integration with HR systems (org chart, time-off)

---

## Development Guidelines

### Before Starting Each Phase:
1. Review relevant documentation
2. Create feature branch
3. Read through all tasks

### During Development:
1. Build frequently (`dx build`)
2. Test manually after each task
3. Keep commits atomic
4. Update documentation alongside code

### After Completing Each Phase:
1. Run full test suite
2. Build release
3. Complete all manual testing
4. Check off all acceptance criteria
5. Merge to main
6. Create git tag

---

## References

- **UI Design Spec**: `docs/ui-design.md`
- **Component Reference**: `docs/component-reference.md`
- **Visual Mockup**: `docs/mockup.html`
- **Dioxus Docs**: https://dioxuslabs.com/learn/0.7/
