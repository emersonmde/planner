# Quarterly Planner - Development Roadmap

This roadmap breaks down the development of the Quarterly Planner application into logical, incrementally deliverable phases. Each phase builds successfully, passes all tests, and results in a working application.

## Progress Status

**Current Status:** Phase 4 Complete (4 of 16 phases)

- ✅ **Phase 1**: Foundation & Design System
- ✅ **Phase 2**: Data Models & State Management
- ✅ **Phase 3**: Roadmap View (Table Display)
- ✅ **Phase 4**: Allocation Grid (Read-Only Display)
- 🔜 **Phase 5**: Technical Projects View

**Build Status:** ✅ Compiles successfully with 20 warnings (unused code - expected during incremental development)

---

## Overview

The Quarterly Planner is a Dioxus-based desktop/web application for engineering managers to plan quarterly resource allocation across roadmap projects, technical projects, and individual engineers. The application features:

- **Three main views**: Roadmap, Technical Projects, and Allocation Grid
- **Dark mode design system** with Apple-inspired aesthetics
- **Interactive allocation grid** with paintbrush mode for quick assignment
  - **Timeline convention**: Time flows horizontally (weeks across top), engineers vertically (left side)
  - **Multi-week project connections**: Visual continuity shows project duration at a glance
  - **Sticky headers**: Both axes remain visible during scroll
- **Capacity tracking** and validation
- **File-based persistence** (JSON format)

**Key Documentation References:**
- UI Design: `docs/ui-design.md` (authoritative design spec with updated grid layout)
- Component Reference: `docs/component-reference.md`
- Visual Mockup: `docs/mockup.html` (note: shows old vertical layout, use ui-design.md for current spec)

---

## Development Workflow

### Quality Assurance

The project uses a multi-layered approach to ensure code quality:

1. **Pre-commit Hook** (`.githooks/pre-commit`)
   - Runs automatically before each commit (requires one-time setup: `git config core.hooksPath .githooks`)
   - Checks: formatting, linting (web + desktop), tests, documentation, security audit, WASM build
   - Prevents commits that don't pass all checks
   - **Required tools**: `cargo-audit`, `dioxus-cli` (dx)

2. **Continuous Integration** (`.github/workflows/ci.yml`)
   - Triggered on: push to main, pull requests
   - Jobs: test, fmt, clippy (web + desktop), cargo-audit, WASM build
   - **GitHub Pages Deployment**: Automatically deploys to GitHub Pages on push to main
   - All CI checks mirror the pre-commit hook for consistency

### Setup

```bash
# One-time setup: Configure git to use pre-commit hooks
git config core.hooksPath .githooks

# Install required tools
cargo install cargo-audit --locked
cargo install dioxus-cli --locked
```

### Manual Quality Checks

```bash
# Format code
cargo fmt

# Lint (web target)
cargo clippy --target wasm32-unknown-unknown --features web -- -D warnings

# Lint (desktop target)
cargo clippy --features desktop --all-targets -- -D warnings

# Run tests
cargo test --verbose --features desktop

# Security audit
cargo audit

# Build documentation
cargo doc --no-deps

# Build WASM bundle
dx bundle --release
```

---

## Phase 1: Foundation & Design System ✅ COMPLETE
**Goal:** Establish the core architecture, design system, and layout structure

### Tasks
1. **Set up project structure** ✅
   - Create module structure: `models/`, `components/ui/`, `components/layout/`, `components/views/`
   - Set up data models (see `docs/ui-design.md` section 9.2)
   - Create design token CSS file

2. **Implement design tokens** ✅ (Reference: `docs/ui-design.md` section 12.1)
   - Create `/assets/styling/theme.css` with all CSS variables
   - Colors (backgrounds, borders, text, semantic, project palette)
   - Typography system
   - Spacing system
   - Shadows and border radii

3. **Build layout foundation** ✅ (Reference: `docs/ui-design.md` section 7)
   - Create `TopNav` component with basic structure
   - Implement view switching (Roadmap | Technical | Allocation tabs)
   - Create main content container
   - Add quarter selector dropdown

4. **Create basic UI components** ✅ (Reference: `docs/component-reference.md` sections 2-3)
   - Button component (primary, secondary, icon variants)
   - Badge component (success, warning, error variants)
   - Input component (search, text)
   - Basic dropdown/select component

### Acceptance Criteria
- [x] Application builds successfully with `dx build`
- [x] TopNav renders with all elements (title, quarter selector, view tabs, capacity indicator placeholder)
- [x] View tabs switch between three empty view containers
- [x] All CSS design tokens are defined and accessible
- [x] Button, Badge, and Input components render correctly in all variants
- [x] Dark mode theme displays correctly

### Manual Testing
**Run:** `dx serve`
1. Verify top navigation displays correctly ✅
2. Click each view tab - container should switch (empty for now) ✅
3. Check button hover states and variants ✅
4. Verify badge colors match design spec ✅
5. Test input focus states ✅

### Documentation Updates
- [x] Update README.md with setup instructions
- [x] Document component props in code comments
- [x] Create `docs/architecture.md` describing module structure (documented in CLAUDE.md)
- [x] Create `docs/adrs/ADR-001-design-system-structure.md` - Document design token approach, CSS architecture, module organization

---

## Phase 2: Data Models & State Management ✅ COMPLETE
**Goal:** Implement core data structures and state management

### Tasks
1. **Define data models** ✅ (Reference: `docs/ui-design.md` section 9.2)
   - Create `models/plan.rs` with:
     - `Engineer` struct (id, name, role, capacity)
     - `RoadmapProject` struct (id, name, estimates, dates, color)
     - `TechnicalProject` struct (id, name, roadmap_project_id, estimates)
     - `Allocation` struct (engineer_id, week_start_date, assignments)
     - `ProjectColor` enum with hex conversion methods
   - Implement serialization/deserialization (serde)

2. **Set up state management** ✅
   - Create global state using Dioxus signals
   - Implement `use_plan_state` hook
   - Add state initialization with sample data
   - Create utility functions for capacity calculations

3. **Week/Sprint calculation utilities** ✅ (Reference: `docs/ui-design.md` section 7.3)
   - Create `utils/date_helpers.rs`
   - Implement quarter start date to week list conversion
   - Sprint number calculation (2-week sprints)
   - Week number formatting (e.g., "Wk 1/13")

4. **Validation logic** ✅
   - Allocation percentage validation (must sum to 100%)
   - Capacity overflow detection
   - Start date vs allocation date validation

### Acceptance Criteria
- [x] Application builds successfully
- [x] All data models compile with proper traits (Debug, Clone, Serialize, Deserialize)
- [x] Sample data initializes correctly in global state
- [x] Week/sprint calculations produce correct results
- [x] Validation functions return appropriate errors

### Manual Testing
**Run:** `dx serve`
1. Open browser console and verify state initializes ✅
2. Check that sample data is accessible (add temporary debug output) ✅

### Documentation Updates
- [x] Document data model schema in `docs/architecture.md` (documented in CLAUDE.md)
- [x] Add code comments explaining validation rules
- [x] Document state management patterns used (documented in CLAUDE.md)
- [x] Create `docs/adrs/ADR-002-state-management.md` - Document why Dioxus Signals, alternatives considered, state structure decisions
- [x] Create `docs/state-management.md` - Document patterns for using `use_plan_state`, signal updates, computed values

---

## Phase 3: Roadmap View (Table Display) ✅ COMPLETE
**Goal:** Implement the Roadmap Projects view with full table functionality

### Tasks
1. **Create data table component** ✅ (Reference: `docs/component-reference.md` section 4, `docs/mockup.html` lines 1074-1208)
   - Build generic `DataTable` component
   - Header row with column titles
   - Dynamic row generation from data
   - Hover states
   - Cell styling (emphasis, monospace, secondary text)

2. **Implement Roadmap Projects table** ✅
   - Project name with color dot
   - Engineering/Science/Total estimates
   - Engineering/Science/Total allocations (with status badges)
   - Start date and launch date
   - Notes field (truncated)

3. **Add capacity badge logic** ✅ (Reference: `docs/component-reference.md` section 2)
   - Calculate allocated vs estimated for each project
   - Apply success/warning/error styling based on thresholds:
     - Success: within 5% (±0.5 weeks per 10 weeks)
     - Warning: 5-25% off
     - Error: >25% off

4. **Build quarter summary section** ✅ (Reference: `docs/mockup.html` lines 1210-1229)
   - Total capacity calculation
   - Total allocated calculation
   - Utilization percentage
   - Capacity breakdown by role

5. **Add search/filter functionality** ✅
   - Search input component
   - Filter projects by name
   - "Add Roadmap Project" button (placeholder for now)

### Acceptance Criteria
- [x] Application builds successfully
- [x] Roadmap view displays table with sample data
- [x] Status badges show correct colors based on allocation
- [x] Quarter summary shows accurate capacity calculations
- [x] Search filters projects by name
- [x] Table rows have hover states
- [x] All typography and spacing match design spec

### Manual Testing
**Run:** `dx serve`
1. Switch to Roadmap view ✅
2. Verify all table columns display correctly ✅
3. Check status badge colors (success/warning/error) ✅
4. Verify quarter summary calculations ✅
5. Test search functionality ✅
6. Hover over table rows to verify hover state ✅

### Documentation Updates
- [x] Document capacity calculation formulas in code comments

### Implementation Notes
- Created generic `DataTable`, `TableRow`, `TableCell`, `ProjectName` components (src/components/ui/data_table.rs)
- Added capacity calculation methods to Plan model: `calculate_roadmap_allocated_weeks()`, `calculate_total_capacity()`, `calculate_total_allocated()`
- Implemented `get_capacity_status()` function to determine badge type based on allocation variance
- Full RoadmapView with search, table, and quarter summary (src/components/views/roadmap_view.rs)
- Added comprehensive CSS for data tables and quarter summary section (assets/styling/main.css)

---

## Phase 4: Allocation Grid (Read-Only Display) ✅ COMPLETE
**Goal:** Build the allocation grid with read-only cell display

**Note:** Grid uses timeline convention - time flows horizontally (weeks across top), people vertically (engineers on left)

### Tasks
1. **Create grid structure** ✅ (Reference: `docs/ui-design.md` section 5.3 & 7.3)
   - Grid container with dynamic columns based on week count
   - Column headers: Sprint numbers, week dates, week progress (3 rows, sticky on vertical scroll)
   - Row headers: Engineer names, roles, capacity indicators (fixed width 180px, sticky on horizontal scroll)
   - Sprint separators (VERTICAL dashed lines every 2 weeks)

2. **Implement GridCell component** ✅ (Reference: `docs/ui-design.md` section 5.3)
   - Cell dimensions: 150px width × 120px height (increased from 120px × 96px for better readability)
   - Empty cell state (dashed border, "+" icon)
   - Single week allocation (project name, percentage, rounded corners)
   - Multi-week allocation (connected cells with rounded edges, duration badge on last cell)
   - Split allocation cell (horizontal layout: project name left, percentage badge right, stacked vertically)
   - Oncall cell (purple with diagonal stripes, phone icon)
   - Unallocated cell (red tint with dashed lines)
   - Before-start-date cell (hash overlay, warning icon)

3. **Implement multi-week project connections** ⚠️ (Deferred to Phase 6: Editing)
   - Detect consecutive weeks with same project for an engineer
   - First cell: rounded left edge, square right
   - Middle cells: square all edges, subtle connecting lines
   - Last cell: square left edge, rounded right, duration badge (e.g., "3w")
   - Hover any cell highlights entire connected series
   - Slightly darker background for visual cohesion
   - NOTE: Component structure supports this, but automatic detection will be added in editing phase

4. **Render grid from state data** ✅
   - Generate weeks for Q1 2025 (13 weeks) as columns
   - Map allocations to grid cells
   - Apply project colors from enum
   - Calculate sprint numbers and groupings

5. **Add row header capacity indicators** ✅ (Reference: `docs/ui-design.md` section 5.3)
   - Show allocated vs capacity for each engineer (e.g., "11.5 / 12 weeks")
   - Mini progress bar (80px width, 4px height)
   - Color code based on utilization:
     - Success: within 0.5 weeks
     - Warning: 0.5-1 week off
     - Error: >1 week off

6. **Implement scroll behavior** ✅
   - Horizontal scroll for many weeks (>13)
   - Vertical scroll for many engineers (>6)
   - Column headers sticky on vertical scroll
   - Row headers sticky on horizontal scroll

### Acceptance Criteria
- [x] Application builds successfully
- [x] Allocation grid displays with sample data (weeks horizontal, engineers vertical)
- [x] All cell variants render correctly (empty, single-week, split, oncall, before-start)
- [ ] Multi-week projects show as connected cells with duration badge (deferred to Phase 6)
- [x] Split cells show horizontal layout (project name left, percentage badge right, stacked top/bottom)
- [x] Cell dimensions are 150px × 120px for optimal text display
- [x] Project colors apply correctly
- [x] Sprint numbers and week dates calculate correctly
- [x] VERTICAL sprint separators appear every 2 weeks
- [x] Engineer capacity indicators show correct colors and progress bars
- [x] Horizontal scroll works for many weeks
- [x] Sticky headers work correctly (columns on vertical scroll, rows on horizontal scroll)

### Manual Testing
**Run:** `dx serve`
1. Switch to Allocation view ✅
2. Verify grid structure: ✅
   - Weeks flow horizontally across top
   - Engineers listed vertically on left
   - Sprint numbers span 2 week columns
3. Check grid cell variants: ✅
   - Empty cell with "+" icon
   - Single week allocated cell with project name
   - Split cell with horizontal layout (project name left, percentage right, stacked top/bottom)
   - Oncall cell with purple styling
   - Before-start cell with hash pattern
4. Verify VERTICAL sprint separators appear every 2 weeks ✅
5. Check engineer capacity colors and progress bars in row headers ✅
6. Test horizontal scroll for many weeks ✅
7. Verify sticky headers work (scroll both directions) ✅

### Documentation Updates
- [x] Document grid cell state logic in code comments
- [x] Document week/sprint calculation logic
- [x] Create `docs/adrs/ADR-003-grid-layout.md` - Document CSS Grid choice, timeline orientation (horizontal weeks), sticky header approach, alternatives considered
- [x] Create `docs/grid-architecture.md` - Document grid rendering pipeline, cell state machine, multi-week detection strategy, performance considerations

### Implementation Notes
- Created `GridCell` component with variants: Empty, SingleWeek, MultiWeek, Split, Oncall (src/components/ui/grid_cell.rs)
- Implemented `AllocationView` with full grid rendering (src/components/views/allocation_view.rs)
- Added comprehensive CSS for all grid elements, sticky headers, and cell variants (assets/styling/main.css)
- Multi-week project detection deferred to Phase 6 (editing) - component structure supports it
- Grid uses CSS Grid with dynamic column calculation based on week count
- Sticky headers implemented with z-index layering (corner: 3, column headers: 2, row headers: 1)
- Capacity indicators use color-coded text and progress bars
- **UI Refinements (Post-Phase 4):**
  - Increased cell dimensions from 120px × 96px to 150px × 120px for better readability (maintaining 1.25:1 ratio)
  - Optimized split cell layout: horizontal design with project name on left, percentage badge on right, two sections stacked vertically
  - Improved text rendering with 3-line clamp and left-aligned text in split cells
  - Adjusted grid template columns from 120px to 150px per week

---

## Phase 5: Technical Projects View
**Goal:** Implement the Technical Projects view with side panel

### Tasks
1. **Create side panel component** (Reference: `docs/component-reference.md` section 9, `docs/mockup.html` lines 1234-1270)
   - Filter section with checkboxes (All, On Track, At Risk, No Link)
   - Sort section with radio buttons (Roadmap, Status, Allocation)
   - Collapsible functionality
   - 320px width with border

2. **Build Technical Projects table**
   - Technical project name with color dot
   - Linked roadmap project
   - Estimated weeks
   - Allocated weeks (with status badge)
   - Status text
   - Grid layout with side panel

3. **Implement filter logic**
   - Filter by status (on track, at risk)
   - Filter by roadmap link presence
   - "All Projects" checkbox

4. **Implement sort logic**
   - Sort by roadmap project
   - Sort by status
   - Sort by allocation

5. **Add search functionality**
   - Search technical projects by name
   - "Add Technical Project" button (placeholder)

### Acceptance Criteria
- [x] Application builds successfully
- [x] Technical view displays with side panel and table
- [x] Side panel filters work correctly
- [x] Sort options reorder table
- [x] Search filters technical projects
- [x] Table shows correct data from state
- [x] Status badges match allocation status
- [x] Layout matches design spec (320px panel, flex layout)

### Manual Testing
**Run:** `dx serve`
1. Switch to Technical view ✅
2. Verify side panel displays on left ✅
3. Test each filter option: ✅
   - All Projects
   - On Track
   - At Risk
   - No Roadmap Link
4. Test each sort option ✅
5. Test search functionality ✅
6. Verify table data matches state ✅
7. Check status badge colors ✅

### Documentation Updates
- [x] Document filter/sort logic in code comments

### Implementation Notes
- Created side panel component with filter and sort sections (src/components/views/technical_view.rs)
- Implemented all filter options using HashSet for multi-select functionality
- Implemented sorting with three sort modes (Roadmap, Status, Allocation)
- Added search functionality with real-time filtering
- Created comprehensive CSS for side panel, buttons, and search input (assets/styling/main.css)
- Added BadgeType::Info variant for "Not Started" status
- **UI Improvements:**
  - Enhanced search bar visibility with better contrast and hover/focus states
  - Added proper button styling with hover and active states
  - Created view-header layout with proper spacing
  - Changed error status text from "Over Allocated" to "Critical" for accuracy
  - Search input: 40px height, `--bg-secondary` background, subtle border with hover effect
  - Primary button: Blue with hover lift effect and shadow transitions

---

## Phase 6: Interactive Allocation Grid (Paintbrush Mode)
**Goal:** Add interactive allocation editing with paintbrush mode

### Tasks
1. **Implement paintbrush mode toggle** (Reference: `docs/component-reference.md` section 7, `docs/ui-design.md` section 6.1)
   - Toggle button (OFF/ON states)
   - Project selector dropdown
   - Cursor style change when active
   - Visual feedback (glow effect when ON)

2. **Add project selector dropdown** (Reference: `docs/ui-design.md` section 5.4)
   - List all technical projects
   - Show project color dot
   - Show allocated weeks count
   - Oncall option at bottom
   - Search within dropdown

3. **Implement click-to-allocate**
   - Click empty cell → assign selected project at 100%
   - Visual feedback (success glow animation)
   - Update state
   - Recalculate capacity

4. **Implement click-and-drag painting**
   - Mouse down → start paint
   - Mouse move → highlight cells in drag path
   - Mouse up → commit allocations
   - Drag path visual feedback

5. **Add keyboard shortcuts** (Reference: `docs/ui-design.md` section 6.3)
   - Esc: Exit paintbrush mode
   - Arrow keys: Navigate grid
   - Tab/Shift+Tab: Move between cells

6. **Implement cell validation**
   - Prevent allocation to already-full cells
   - Show error feedback (shake animation, red border)
   - Validate against project start dates

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Paintbrush toggle switches states correctly
- [ ] Project selector appears when paintbrush ON
- [ ] Clicking empty cell allocates selected project
- [ ] Drag painting works across multiple cells
- [ ] Esc key exits paintbrush mode
- [ ] Invalid allocations show error feedback
- [ ] State updates reflect in capacity indicators
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Switch to Allocation view
2. Click "Paintbrush Mode: OFF" → should turn ON with blue styling
3. Select a project from dropdown
4. Click an empty cell → should allocate project
5. Click and drag across multiple cells → should paint all
6. Try clicking a full cell → should show error
7. Press Esc → should exit paintbrush mode
8. Verify capacity indicators update after allocation

### Documentation Updates
- [ ] Document paintbrush mode interaction patterns
- [ ] Document state mutation patterns in code
- [ ] Create `docs/adrs/ADR-004-paintbrush-interaction.md` - Document paintbrush mode design, click-and-drag vs alternatives, project selector UX decisions
- [ ] Create `docs/interaction-patterns.md` - Document all interaction modes (paintbrush, context menu, drag-drop), keyboard shortcuts, user feedback patterns

---

## Phase 7: Context Menu & Split Allocation
**Goal:** Add right-click context menu and split allocation dialog

### Tasks
1. **Implement context menu** (Reference: `docs/ui-design.md` section 5.4)
   - Right-click on grid cell → show menu
   - Menu items:
     - Assign Project...
     - Split Allocation...
     - Clear Assignment
     - Copy Assignment (Cmd+C)
     - Paste Assignment (Cmd+V)
   - Position menu near cursor
   - Close on click outside

2. **Build split allocation modal** (Reference: `docs/ui-design.md` section 5.4)
   - Modal overlay with backdrop blur
   - Project A selector with percentage slider
   - Visual preview bar showing VERTICAL split (matches cell design)
   - Project B selector with auto-calculated percentage
   - Validation: total must equal 100%
   - Apply/Cancel buttons

3. **Implement copy/paste allocation**
   - Store allocation in clipboard state
   - Keyboard shortcuts (Cmd/Ctrl+C, Cmd/Ctrl+V)
   - Visual feedback for paste

4. **Add clear assignment functionality**
   - Remove allocation from cell
   - Update state and capacity
   - Visual feedback (fade out animation)

5. **Implement delete/backspace key**
   - Delete key clears selected cell
   - Works on focused cell

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Right-click opens context menu near cursor
- [ ] Split allocation modal opens and functions correctly
- [ ] Percentage slider updates both projects (total = 100%)
- [ ] Split cells display correctly in grid
- [ ] Copy/paste works with keyboard shortcuts
- [ ] Clear assignment removes allocation
- [ ] Delete/Backspace key clears cell
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Right-click on an allocated cell → verify menu appears
2. Click "Split Allocation..."
   - Select two projects
   - Adjust slider
   - Verify preview updates
   - Click Apply
   - Verify split cell appears in grid
3. Test copy/paste:
   - Right-click cell → Copy Assignment
   - Right-click empty cell → Paste Assignment
4. Test clear:
   - Right-click cell → Clear Assignment
   - Verify cell becomes empty
5. Test keyboard delete on focused cell

### Documentation Updates
- [ ] Add code comments for modal state management
- [ ] Document keyboard shortcuts in code comments
- [ ] Update `docs/interaction-patterns.md` with context menu and split allocation workflows

---

## Phase 8: Tooltip System
**Goal:** Add comprehensive tooltip system for grid cells and other elements

### Tasks
1. **Create tooltip component** (Reference: `docs/component-reference.md` section 8, `docs/ui-design.md` section 6.4)
   - Positioned absolutely near target element
   - Backdrop blur effect
   - Border with emphasis color
   - Shadow
   - Arrow pointer to target
   - Max width 280px

2. **Implement hover delay system**
   - Show tooltip after 300ms hover
   - Hide after 100ms mouse leave
   - Cancel on mouse out before delay

3. **Add tooltip content for grid cells** (Reference: `docs/ui-design.md` section 6.4)
   - Project name
   - Linked roadmap project (clickable)
   - Allocation percentage
   - Project progress (X / Y weeks)
   - Status indicator
   - Hint text ("Click to edit • Right-click for options")

4. **Add tooltip for engineer headers**
   - Engineer name and role
   - Q1 capacity and utilization
   - Current projects list with allocated weeks
   - Oncall weeks
   - Unallocated weeks

5. **Add tooltip for project table rows**
   - Project details
   - Estimates
   - Allocated breakdown by engineer
   - Start and launch dates

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Tooltips appear after 300ms hover
- [ ] Tooltips disappear after 100ms mouse leave
- [ ] Tooltip positions correctly near target
- [ ] Grid cell tooltips show all required info
- [ ] Engineer header tooltips show capacity breakdown
- [ ] Project tooltips show assignment details
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Hover over allocated grid cell
   - Wait 300ms → tooltip should appear
   - Verify all info displays correctly
   - Move mouse away → tooltip should disappear
2. Hover over engineer header
   - Verify capacity breakdown
   - Check current projects list
3. Hover over project in table
   - Verify project details
4. Test tooltip positioning near screen edges

### Documentation Updates
- [ ] Document tooltip content templates

---

## Phase 9: File Operations (Save/Load)
**Goal:** Implement JSON-based file persistence

### Tasks
1. **Create file menu dropdown** (Reference: `docs/ui-design.md` section 9.1)
   - File menu button (☰ icon)
   - Dropdown with options:
     - New Plan (Cmd+N)
     - Open Plan... (Cmd+O)
     - Save Plan (Cmd+S)
     - Save Plan As... (Cmd+Shift+S)
     - Export to CSV
     - Export to Excel
     - Import from CSV
     - Preferences...
     - About

2. **Implement JSON serialization** (Reference: `docs/ui-design.md` section 9.2)
   - Serialize plan state to JSON
   - Include version field for future compatibility
   - Save quarter config, engineers, projects, allocations

3. **Implement save functionality**
   - Save current plan to file
   - File dialog (platform-specific)
   - Save As with file name input
   - Auto-save on changes (optional)

4. **Implement load functionality**
   - Open file dialog
   - Load and parse JSON
   - Validate file format
   - Update application state
   - Handle errors gracefully

5. **Add keyboard shortcuts**
   - Cmd/Ctrl+N: New plan (confirm if unsaved changes)
   - Cmd/Ctrl+O: Open plan
   - Cmd/Ctrl+S: Save plan
   - Cmd/Ctrl+Shift+S: Save as

6. **Implement "New Plan" dialog**
   - Confirm if unsaved changes
   - Reset state to defaults
   - Allow setting quarter and year

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] File menu dropdown displays all options
- [ ] Save creates valid JSON file
- [ ] Load reads JSON and updates state correctly
- [ ] Keyboard shortcuts work
- [ ] New plan confirms if unsaved changes
- [ ] File format validation catches errors
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Make changes to allocation grid
2. Click File menu → Save Plan As...
   - Choose location and save
   - Open file in editor → verify JSON structure
3. Make more changes
4. Click File menu → Open Plan...
   - Select saved file
   - Verify state restores correctly
5. Test keyboard shortcuts:
   - Cmd/Ctrl+S: Save
   - Cmd/Ctrl+O: Open
   - Cmd/Ctrl+N: New (should confirm)
6. Test error handling:
   - Try opening invalid JSON file
   - Verify error message displays

### Documentation Updates
- [ ] Create `docs/adrs/ADR-005-file-format.md` - Document JSON choice vs alternatives (YAML, TOML), schema versioning strategy, migration approach
- [ ] Create `docs/file-format.md` - Document complete JSON schema, field descriptions, versioning, example files
- [ ] Document keyboard shortcuts in code comments

---

## Phase 10: CSV Import/Export
**Goal:** Add CSV import and export functionality

### Tasks
1. **Implement CSV export for roadmap projects**
   - Export columns: Project Name, Eng Est, Sci Est, Total Est, Eng Alloc, Sci Alloc, Total Alloc, Start Date, Launch Date, Notes
   - File dialog to save location
   - CSV formatting with proper escaping

2. **Implement CSV export for allocation grid**
   - Export engineer allocation matrix
   - Rows: weeks, Columns: engineers
   - Cell values: project names or percentages

3. **Implement CSV import for projects**
   - Parse CSV file
   - Validate headers
   - Map to project data structure
   - Handle errors (missing fields, invalid data)
   - Preview before import

4. **Add export to Excel (optional)**
   - If feasible, use a library to export XLSX
   - Otherwise, enhanced CSV with recommended Excel settings

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Export roadmap projects to CSV creates valid file
- [ ] Export allocation grid to CSV creates readable matrix
- [ ] Import CSV validates and creates projects
- [ ] Import shows preview before applying
- [ ] Import handles errors gracefully
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. File menu → Export to CSV
   - Choose "Roadmap Projects"
   - Save file
   - Open in Excel/Sheets → verify formatting
2. File menu → Export to CSV
   - Choose "Allocation Grid"
   - Save file
   - Open in Excel/Sheets → verify matrix layout
3. File menu → Import from CSV
   - Select valid project CSV
   - Verify preview
   - Import
   - Check projects appear in tables
4. Test error handling:
   - Import CSV with missing headers
   - Import CSV with invalid data
   - Verify error messages

### Documentation Updates
- [ ] Update `docs/file-format.md` with CSV format specifications (roadmap projects format, allocation grid format, import requirements, validation rules)

---

## Phase 11: Preferences & Settings
**Goal:** Implement user preferences and configuration

### Tasks
1. **Create preferences modal** (Reference: `docs/ui-design.md` section 9.3)
   - General settings:
     - Sprint start day dropdown (Monday-Sunday)
     - Sprint length (1-4 weeks)
     - Default capacity per engineer
   - Theme settings:
     - Dark mode (default)
     - Light mode toggle (placeholder for future)
   - Grid settings:
     - Cell width slider
     - Show sprint separators toggle
     - Highlight weekends toggle
   - Notification settings:
     - Warn on over-allocation toggle
     - Show capacity alerts toggle

2. **Implement preferences persistence**
   - Save preferences to local storage (web) or config file (desktop)
   - Load on app startup
   - Apply settings to UI

3. **Add quarter configuration**
   - Quarter selector updates quarter setting
   - Start date calculation from quarter
   - Number of weeks in quarter

4. **Implement sprint start day logic**
   - Recalculate week dates when start day changes
   - Update grid display

5. **Add capacity default setting**
   - Use default when adding new engineers
   - Allow override per engineer

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Preferences modal opens from File menu
- [ ] All preference settings are functional
- [ ] Preferences persist across app restarts
- [ ] Sprint start day recalculates dates correctly
- [ ] Default capacity applies to new engineers
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. File menu → Preferences
2. Test each setting:
   - Change sprint start day → verify grid updates
   - Change sprint length → verify separators update
   - Adjust default capacity
   - Toggle grid settings
3. Close and reopen app
   - Verify preferences persist
4. Add new engineer
   - Verify default capacity applies

### Documentation Updates
- [ ] Create `docs/adrs/ADR-006-preferences-storage.md` - Document storage strategy (local storage for web, config file for desktop), format choice, default values approach
- [ ] Create `docs/preferences.md` - Document all preference settings, defaults, validation rules, storage location per platform
- [ ] Document preferences format in code comments

---

## Phase 12: Undo/Redo System
**Goal:** Add undo/redo functionality for allocations

### Tasks
1. **Implement command pattern for state mutations**
   - Create `Command` trait
   - Commands:
     - AllocateCommand
     - ClearCommand
     - SplitCommand
     - PasteCommand
   - Each command has `execute()` and `undo()`

2. **Add undo/redo stack**
   - History stack for executed commands
   - Redo stack for undone commands
   - Max history size (e.g., 50 operations)

3. **Implement keyboard shortcuts**
   - Cmd/Ctrl+Z: Undo
   - Cmd/Ctrl+Shift+Z: Redo
   - Update menu items (enabled/disabled based on stack)

4. **Add undo/redo buttons (optional)**
   - Toolbar buttons for undo/redo
   - Disabled state when stack is empty
   - Show tooltip with last action

5. **Handle state transitions**
   - Clear redo stack on new action
   - Limit history size
   - Optimize for performance

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Undo reverses last allocation change
- [ ] Redo re-applies undone change
- [ ] Keyboard shortcuts work (Cmd/Ctrl+Z, Cmd/Ctrl+Shift+Z)
- [ ] History limit prevents memory issues
- [ ] All tests pass including undo/redo edge cases

### Manual Testing
**Run:** `dx serve`
1. Make allocation changes:
   - Allocate project to cell
   - Clear cell
   - Split allocation
2. Press Cmd/Ctrl+Z multiple times
   - Verify each action undoes correctly
3. Press Cmd/Ctrl+Shift+Z
   - Verify redo works
4. Make new change after undo
   - Verify redo stack clears
5. Make >50 changes
   - Verify old history is discarded

### Documentation Updates
- [ ] Create `docs/adrs/ADR-007-undo-redo-pattern.md` - Document command pattern choice, alternatives considered (memento pattern), memory management strategy, history limit decisions
- [ ] Document command pattern implementation in code comments

---

## Phase 13: Validation & Alerts
**Goal:** Add comprehensive validation and user feedback

### Tasks
1. **Implement over-allocation warnings**
   - Detect when engineer is over-allocated
   - Show warning icon in capacity indicator
   - Toast notification when allocation causes over-allocation
   - Option to proceed or cancel

2. **Add under-allocation alerts**
   - Detect unallocated weeks for engineers
   - Show info notification in capacity summary
   - Highlight unallocated cells in grid

3. **Implement project deadline validation**
   - Warn when allocating before project start date
   - Alert if project won't complete by launch date based on current allocation
   - Show critical path issues

4. **Add save validation**
   - Check for incomplete data before save
   - Warn if projects have zero allocation
   - Confirm if saving plan with errors

5. **Create notification/toast system**
   - Success messages (green)
   - Warning messages (orange)
   - Error messages (red)
   - Info messages (blue)
   - Auto-dismiss after timeout
   - Dismiss on click

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Over-allocation warnings appear correctly
- [ ] Under-allocation info displays
- [ ] Project deadline warnings show
- [ ] Toast notifications display and auto-dismiss
- [ ] Save validation catches incomplete data
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Over-allocate an engineer
   - Verify warning appears
   - Check capacity indicator shows error color
2. Leave engineer weeks unallocated
   - Verify info notification
   - Check unallocated cells are highlighted
3. Allocate before project start date
   - Verify warning icon on cell
   - Check tooltip shows warning
4. Try to save incomplete plan
   - Verify validation dialog
5. Test toast notifications:
   - Success: save plan
   - Warning: over-allocation
   - Error: invalid import
   - Info: under-allocation

### Documentation Updates
- [ ] Document validation rules in `docs/validation.md`

---

## Phase 14: Animations & Polish
**Goal:** Add micro-interactions and polish UI

### Tasks
1. **Implement animations** (Reference: `docs/ui-design.md` section 6.5)
   - Button press: scale 0.98, 150ms
   - Cell selection: border grow + color shift
   - Project assignment: success glow (300ms fade)
   - Drag start: lift shadow + scale 1.02
   - Dropdown open: fade + slide (250ms)
   - Modal open: backdrop fade + modal scale spring

2. **Add loading states**
   - Skeleton screens for initial load
   - Loading spinner for file operations
   - Progress bar for large imports

3. **Improve hover states**
   - Smooth transitions on all interactive elements
   - Consistent timing (150ms)
   - Visual feedback for clickable elements

4. **Add drag & drop for cells** (Reference: `docs/ui-design.md` section 6.2)
   - Click cell border to select
   - Drag to another cell to move
   - Drag to empty space to delete
   - Visual feedback: lifted shadow, semi-transparent
   - Drop target validation: green/red border

5. **Implement capacity dashboard visualization** (Reference: `docs/ui-design.md` section 8.1)
   - Animated progress bars
   - Hover shimmer effect on bars
   - Smooth color transitions

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] All animations use correct timing functions
- [ ] Loading states appear for async operations
- [ ] Drag & drop works smoothly
- [ ] Hover states are consistent across UI
- [ ] Capacity bars animate smoothly
- [ ] Performance remains good (60fps)
- [ ] All tests pass

### Manual Testing
**Run:** `dx serve`
1. Test all button press animations
2. Allocate projects and watch success glow
3. Test drag & drop:
   - Drag cell to new location
   - Drag to invalid target (should show red border)
   - Drop on valid target
4. Open/close modals and dropdowns
   - Verify smooth animations
5. Hover over interactive elements
   - Check transition smoothness
6. Load large plan file
   - Verify loading state appears

### Documentation Updates
- [ ] Document animation timing in code comments

---

## Phase 15: Testing & Accessibility
**Goal:** Comprehensive testing and accessibility improvements

### Tasks
1. **Write unit tests**
   - Data model tests (validation, calculations)
   - Utility function tests (date helpers, capacity calcs)
   - State management tests

2. **Write integration tests**
   - View switching
   - Allocation workflow (paintbrush → allocate → save)
   - File operations (save → load → verify)
   - Undo/redo scenarios

3. **Add accessibility features** (Reference: `docs/ui-design.md` section 11)
   - ARIA labels for all interactive elements
   - Screen reader announcements for state changes
   - Focus indicators (2px outline, primary-50)
   - Keyboard navigation for grid
   - Skip links for large grids

4. **Implement keyboard navigation** (Reference: `docs/ui-design.md` section 6.3)
   - Tab/Shift+Tab: move between cells
   - Arrow keys: navigate grid
   - Enter: open project selector
   - Cmd/Ctrl+1/2/3: switch views
   - All other documented shortcuts

5. **Color contrast validation**
   - Verify all text meets WCAG AA
   - Test with colorblind simulators
   - Ensure pattern overlays (not just color) distinguish states

6. **Performance optimization**
   - Profile and optimize render performance
   - Virtualize grid rows if >20 weeks
   - Memoize expensive calculations
   - Debounce search/filter operations

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] All unit tests pass
- [ ] Integration tests cover main workflows
- [ ] ARIA labels on all interactive elements
- [ ] Full keyboard navigation works
- [ ] Color contrast meets WCAG AA
- [ ] Performance remains smooth with large data (100+ weeks, 20+ engineers)
- [ ] Screen reader can navigate and understand UI

### Manual Testing
**Run:** `dx serve`
1. Test with keyboard only (no mouse):
   - Navigate entire app
   - Switch views
   - Allocate projects
   - Open modals
   - Save/load files
2. Test with screen reader (VoiceOver/NVDA)
   - Verify announcements are clear
   - Check grid navigation
3. Test with colorblind simulator
   - Verify status is clear without color
4. Load large plan (50+ weeks, 15+ engineers)
   - Verify smooth scrolling and interaction

### Documentation Updates
- [ ] Add testing guide in `docs/testing.md`
- [ ] Add keyboard shortcuts reference in `docs/keyboard-shortcuts.md`

---

## Phase 16: Final Polish & Documentation
**Goal:** Final refinements, comprehensive documentation, and release preparation

### Tasks
1. **Final UI polish**
   - Review all spacing matches design spec
   - Verify all colors match design tokens
   - Check typography consistency
   - Fix any layout issues on different screen sizes

2. **Cross-platform testing**
   - Test web build (dx build --platform web)
   - Test desktop build (dx build --platform desktop)
   - Test on macOS, Windows, Linux

3. **Performance optimization**
   - Bundle size optimization
   - Asset compression
   - Code splitting if needed
   - Minimize CSS

4. **Comprehensive documentation**
   - Complete user guide in README
   - Architecture documentation
   - API documentation for components
   - Contributing guide
   - Troubleshooting section
   - Review and finalize all ADRs in `docs/adrs/`
   - Create `docs/adrs/README.md` index of all architectural decisions
   - Ensure all technical documentation is complete and cross-referenced

5. **Create example data files**
   - Sample Q1 2025 plan
   - Sample Q2 2025 plan
   - Tutorial walkthrough plan

6. **Record demo video**
   - Show all major features
   - Walkthrough typical workflow
   - Add to README

7. **Prepare release**
   - Version numbering (1.0.0)
   - Release notes
   - Build distributables
   - Test installation process

### Acceptance Criteria
- [ ] All builds compile successfully across platforms
- [ ] All tests pass
- [ ] Documentation is complete and accurate
- [ ] Example files load correctly
- [ ] Demo video shows all features
- [ ] No known critical bugs
- [ ] Performance is acceptable on target hardware
- [ ] Ready for release

### Manual Testing
**Run:** `dx serve` and `dx build --release`
1. Complete full workflow from scratch:
   - Create new plan
   - Add engineers
   - Add roadmap projects
   - Add technical projects
   - Allocate using paintbrush mode
   - Split allocations
   - Verify capacity indicators
   - Save plan
   - Load plan
   - Export CSV
   - Import CSV
2. Test on different platforms
3. Follow user guide step-by-step to verify accuracy

### Documentation Updates
- [ ] Finalize README with all sections
- [ ] Create CHANGELOG.md
- [ ] Write release announcement
- [ ] Create GitHub releases page

---

## Post-Release Enhancements (Future Phases)

These features can be added in future releases:

### Phase 17: Advanced Features (Future)
- Light mode theme
- Multiple team support
- Project dependencies and Gantt view
- Real-time collaboration
- Integration with JIRA/GitHub
- Advanced reporting and analytics
- Export to PDF with visualizations
- Custom project color picker
- Templates for common project types

### Phase 18: Mobile Support (Future)
- Responsive design for tablet (768-1023px)
- Mobile-optimized views
- Touch gestures for allocation
- Simplified mobile interface

---

## Development Guidelines

### Before Starting Each Phase:
1. Review the relevant sections in `docs/ui-design.md` and `docs/component-reference.md`
2. Check the `docs/mockup.html` for visual reference
3. Create a feature branch: `git checkout -b phase-N-description`
4. Read through all tasks in the phase

### During Development:
1. Build frequently: `dx build` should always succeed
2. Test manually after completing each task
3. Write tests as you go (don't defer to end)
4. Keep commits atomic and well-described
5. Update documentation alongside code changes

### After Completing Each Phase:
1. Run full test suite: `cargo test`
2. Build release: `dx build --release`
3. Complete all manual testing steps
4. Check off all acceptance criteria
5. Update documentation
6. Request review (when working with team)
7. Merge to main
8. Create a git tag: `git tag phase-N-complete`

### Communication:
After each phase, I will run `dx serve` and test the application. I will provide feedback on:
- Any bugs or issues found
- UI/UX suggestions
- Performance concerns
- Questions about next phase

**We will not proceed to the next phase until the current phase is complete and approved.**

---

## Success Metrics

The project is successful when:
- ✅ All 16 phases are complete
- ✅ Application builds without errors on web and desktop
- ✅ All tests pass
- ✅ Can create, save, and load a full quarterly plan
- ✅ Allocation grid is fully interactive
- ✅ Validation and warnings work correctly
- ✅ Keyboard navigation and accessibility features work
- ✅ Documentation is complete and accurate
- ✅ Performance is smooth with realistic data sizes
- ✅ UI matches design specification

---

## Estimated Timeline

Each phase is designed to be a focused unit of work:
- **Phases 1-5**: Foundation (core views, basic functionality)
- **Phases 6-10**: Interactivity (editing, file operations)
- **Phases 11-13**: Advanced features (preferences, undo, validation)
- **Phases 14-16**: Polish (animations, testing, documentation)

**Estimated total**: 16 phases, each representing a logical milestone

---

## References

- **UI Design Spec**: `docs/ui-design.md` - Complete design system
- **Component Reference**: `docs/component-reference.md` - Implementation examples
- **Visual Mockup**: `docs/mockup.html` - Working HTML reference
- **Dioxus Docs**: https://dioxuslabs.com/learn/0.7/

---

**Let's build something great! 🚀**
