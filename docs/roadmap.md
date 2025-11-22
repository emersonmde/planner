# Quarterly Planner - Development Roadmap

This roadmap focuses on upcoming development phases. Completed phases are documented in git history and implementation notes.

## Progress Status

**Current Status:** Phase 6.6 In Progress (6.6 of 17 phases)

- ✅ Phases 1-6 Complete (Foundation, Views, Paintbrush Mode)
- ✅ **Phase 6.5**: Grid Layout Rotation (Vertical Scrolling UX) - COMPLETE
- 🔄 **Phase 6.6**: UI/UX Refinements (Floating FAB, Visual Polish) - IN PROGRESS
- ⏸️ **Phase 7**: Context Menu & Advanced Interactions - PARTIALLY COMPLETE
- 📋 **Phases 8-17**: Planned

**Build Status:** ✅ Compiles successfully

---

## Phase 6.5: Grid Layout Rotation (Vertical Scrolling UX) ✅ COMPLETE
**Goal:** Rotate grid to vertical-scrolling layout for better UX

**Rationale:** More weeks (13) than engineers (4-8) means vertical scrolling is more natural than horizontal scrolling. Better use of screen space on wide monitors.

### Design Analysis

**Current Layout (Horizontal Timeline):**
- **Columns**: Weeks (13+ cells wide) → horizontal scroll required
- **Rows**: Engineers (4-8 cells tall) → minimal vertical scroll
- **Headers**: Week dates across top (sticky on vertical scroll), Engineer names on left (sticky on horizontal scroll)
- **Sprint Separators**: Vertical dashed lines every 2 weeks

**New Layout (Vertical Timeline):**
- **Columns**: Engineers (4-8 cells wide) → minimal horizontal scroll
- **Rows**: Weeks (13+ cells tall) → vertical scroll (more natural)
- **Headers**: Engineer names + capacity across top (sticky on vertical scroll), Week dates on left (sticky on horizontal scroll)
- **Sprint Separators**: Horizontal dashed lines every 2 weeks

### Tasks

1. **Update grid structure** (src/components/views/allocation_view.rs)
   - Change `grid-template-columns` from `180px repeat({weeks}, 150px)` to `120px repeat({engineers}, 180px)`
   - Swap iteration order: outer loop = weeks, inner loop = engineers
   - Update allocation map lookup to use (engineer_id, week_start) correctly

2. **Redesign column headers** (engineer columns)
   - Engineer name + role badge
   - Capacity indicator (allocated / capacity weeks)
   - Mini progress bar (80px width, 4px height)
   - Vertical layout stacking these elements
   - Sticky on vertical scroll (z-index: 2)

3. **Redesign row headers** (week rows)
   - Week date (e.g., "Jan 6")
   - Week progress (e.g., "Wk 1/13")
   - Sprint label (e.g., "Sprint 1") on first week of sprint
   - Width: 120px (narrower than engineer headers were)
   - Sticky on horizontal scroll (z-index: 1)

4. **Update sprint separators**
   - Change from VERTICAL borders (border-left) to HORIZONTAL borders (border-top)
   - Apply to row headers and cells
   - Still every 2 weeks (check if week.is_sprint_start())
   - CSS class: `.grid-week-row.sprint-separator`

5. **Update grid cell dimensions**
   - Maintain aspect ratio and readability
   - Now: 180px width (engineer column), 120px height (week row)
   - Text layout remains the same

6. **Update CSS** (assets/styling/main.css)
   - `.grid-header-corner`: Update z-index layering
   - `.grid-engineer-header`: New class for engineer column headers (replaces `.grid-week-header`)
   - `.grid-week-row-header`: New class for week row headers (replaces `.grid-row-header`)
   - `.sprint-separator`: Change from `border-left` to `border-top`
   - Update sticky positioning classes
   - Adjust grid container scrolling behavior

7. **Update helper functions**
   - No changes needed to `calculate_cell_variant()` - still works the same
   - Allocation map lookup stays (engineer_id, week_start) - no change needed

### Acceptance Criteria
- [x] Application builds successfully
- [x] Grid displays with engineers as columns, weeks as rows
- [x] Vertical scrolling works smoothly for many weeks
- [x] Column headers (engineers) stick to top on vertical scroll
- [x] Row headers (weeks) stick to left on horizontal scroll
- [x] Sprint separators appear as HORIZONTAL lines every 2 weeks
- [x] Capacity indicators display correctly in engineer column headers
- [x] Cell allocation lookups work correctly
- [x] All cell variants render properly
- [x] Paintbrush mode works with new layout

### Manual Testing
**Run:** `dx serve`
1. Switch to Allocation view
2. Verify grid orientation:
   - Engineers listed horizontally across top
   - Weeks listed vertically down left side
   - Sprint numbers span 2 week rows
3. Test vertical scrolling:
   - Scroll down through weeks
   - Verify engineer headers stay visible (sticky top)
4. Test horizontal scrolling (if many engineers):
   - Scroll right through engineers
   - Verify week headers stay visible (sticky left)
5. Check capacity indicators in engineer column headers
6. Verify HORIZONTAL sprint separators every 2 weeks
7. Test paintbrush mode with new layout
8. Verify cell hover and click interactions work

### Documentation Updates
- [ ] Update `docs/adrs/ADR-003-grid-layout.md` with rotation decision and rationale
- [ ] Update `docs/ui-design.md` section 7.3 with new grid orientation
- [ ] Document layout rotation in code comments

---

## Phase 6.6: UI/UX Refinements (Floating FAB, Visual Polish) 🔄 IN PROGRESS
**Goal:** Replace horizontal paintbrush bar with floating action button, improve visual hierarchy and polish

**Status:** Core FAB and panel components complete. Old paintbrush controls still need removal.

**Rationale:**
- Current horizontal paintbrush bar takes 80-100px vertical space (~15-20% of viewport)
- Visually blends with grid headers causing hierarchy confusion
- User feedback: "paintbrush header attaches to table header making it kind of confusing"
- Floating action pattern is more modern, space-efficient, and app-like

### Design Analysis

**Current Issues:**
1. **Space inefficiency**: Paintbrush controls permanently visible when enabled
2. **Visual hierarchy**: No clear separation between mode controls and grid data
3. **Workflow friction**: Toggle → Select → Paint requires 3+ interactions
4. **Limited discoverability**: Small dropdown doesn't use available screen space

**Solution: Floating Action Button (FAB) + Slide-out Panel**
- FAB fixed in bottom-right (64×64px, always accessible)
- Click FAB → Project selector panel slides from right (340px wide)
- Panel shows: search, all projects with colors + allocated weeks, Oncall option
- Unified UI for both "Paintbrush Mode" and "Assign Project" (context menu)
- Panel only appears when needed (reclaims 80-100px vertical space)

### Tasks - Priority 1: Critical (Implement First)

#### 1. **Create Floating Action Button Component** ✅ COMPLETE
**Files:** src/components/ui/floating_fab.rs (new), src/components/ui/mod.rs

- [x] Create `FloatingFab` component
- [x] Position: fixed, bottom: 32px, right: 32px
- [x] Size: 56×56px circle (adjusted from 64px for better proportions)
- [x] Background: var(--bg-overlay) with subtle primary blue hints (design system compliant)
- [x] Hover: scale(1.05), box-shadow increase
- [x] Active state: background changes to selected project color via color-mix
- [x] Click handler: toggle project panel visibility
- [x] Z-index: 1000
- [x] Icon: 🎨 emoji (updated from 🖌️)

**CSS:**
```css
.floating-paintbrush-fab {
  position: fixed;
  bottom: 32px;
  right: 32px;
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--primary-50);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
  transition: all 0.3s ease;
  z-index: 1000;
}

.floating-paintbrush-fab.active {
  background: var(--project-color);
  animation: pulse 2s infinite;
}
```

#### 2. **Create Floating Project Panel Component** ✅ COMPLETE
**Files:** src/components/ui/floating_project_panel.rs (new), src/components/ui/mod.rs

- [x] Create `FloatingProjectPanel` component
- [x] Position: fixed, top: 80px, right: 32px
- [x] Size: 340px width, max-height: calc(100vh - 120px)
- [x] Background: var(--bg-secondary), border, rounded corners (16px)
- [x] Box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6)
- [x] Backdrop-filter: blur(20px) for glassmorphism effect
- [x] Slide-in animation from right (slide-in-right 0.3s)
- [x] Z-index: 999 (below FAB)
- [x] Contains:
  - Search input at top
  - Scrollable project list (all projects)
  - Each project shows: color dot, name, allocated weeks
  - Selected project highlighted with primary color
  - Click project → selects project for paintbrush mode
  - Clear option to deselect
- [x] Click outside → closes panel
- [x] **Added:** Collapse/expand functionality (›› / ‹‹ buttons) for narrow windows
- [x] **Fixed:** Clear button border displays completely
- [x] **Fixed:** Collapse button properly centered between search and edge

**CSS:**
```css
.floating-project-panel {
  position: fixed;
  top: 80px;
  right: 32px;
  width: 340px;
  max-height: calc(100vh - 120px);
  background: var(--bg-secondary);
  border: 1px solid var(--border-emphasis);
  border-radius: 16px;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(20px);
  overflow: hidden;
  z-index: 999;
  animation: slide-in-right 0.3s ease-out;
}

@keyframes slide-in-right {
  from { transform: translateX(100%); opacity: 0; }
  to { transform: translateX(0); opacity: 1; }
}
```

#### 3. **Remove Horizontal Paintbrush Controls** ✅ COMPLETE
**Files:** src/components/views/paintbrush.rs, assets/styling/main.css

- [x] Remove `PaintbrushControls` component
- [x] Remove `ProjectSelector` component
- [x] Keep paintbrush mode state logic (paintbrush_active, selected_project)
- [x] Keep allocation logic (allocate_project_to_cell function)
- [x] Remove horizontal controls container CSS (378 lines removed)
- [x] Clean up unused imports

#### 4. **Integrate FAB + Panel into Allocation View**
**Files:** src/components/views/allocation_view.rs

- [ ] Add `panel_visible` signal (controls panel visibility)
- [ ] Import FloatingFab and FloatingProjectPanel components
- [ ] Render FloatingFab at bottom of RSX (always visible)
- [ ] Render FloatingProjectPanel conditionally (when panel_visible)
- [ ] Wire up FAB click → toggle panel_visible
- [ ] Wire up project selection:
  - From panel → enters paintbrush mode with selected project
  - From context menu "Assign Project" → opens panel, selection assigns to cell
- [ ] Add panel close handlers (click outside, Esc key)
- [ ] Update paintbrush mode to show selected project color in FAB

#### 5. **Unified Project Selection Flow**
**Files:** src/components/views/allocation_view.rs

**Two modes:**
1. **Paintbrush Mode** (FAB click):
   - Click FAB → panel opens
   - Click project → paintbrush_active = true, selected_project = project
   - Panel stays open, FAB shows project color
   - Click cells to paint
   - Esc or FAB click → exit paintbrush mode, close panel

2. **Direct Assignment** (Context menu):
   - Right-click cell → "Assign Project"
   - Panel opens with `assign_mode = true`
   - Click project → assigns to cell immediately, closes panel
   - No paintbrush mode activation

- [ ] Add `assign_mode` signal (differentiates paintbrush vs direct assign)
- [ ] Update panel project click handler to check assign_mode
- [ ] If assign_mode: create allocation, close panel
- [ ] If paintbrush mode: set selected_project, keep panel open

### Code Quality Improvements ✅ COMPLETE

**Performance Optimizations:**
- [x] Fixed search filter inefficiency - cached `to_lowercase()` to avoid redundant allocations
- [x] Removed duplicate color resolution logic across components
- [x] Added `TechnicalProject::get_color(&Plan)` helper method for DRY code

**Validation & Correctness:**
- [x] Added project existence validation in `allocate_project_to_cell()`
- [x] Function now correctly returns `false` when project doesn't exist
- [x] Removed unused `ProjectColor` imports

**Files Modified:**
- src/models/plan.rs - Added `get_color()` helper
- src/components/ui/floating_project_panel.rs - Performance & refactoring
- src/components/views/paintbrush.rs - Performance, validation & refactoring

### Tasks - Priority 2: Important (Do Soon)

#### 6. **Improve Sprint Separator Visibility**
**Files:** assets/styling/main.css

**Issue:** Current 2px dashed border is too subtle, hard to see sprint boundaries

- [ ] Change `.grid-week-row-header.sprint-separator` border-top from 2px dashed to 3px dashed
- [ ] Use var(--border-emphasis) instead of var(--border-default) for higher contrast
- [ ] Add subtle background color to sprint start rows: background: var(--bg-tertiary)
- [ ] Update `.grid-cell.sprint-separator` similarly
- [ ] Test visibility with different monitor brightness levels

```css
.grid-week-row-header.sprint-separator {
  border-top: 3px dashed var(--border-emphasis);
  background: var(--bg-tertiary);
  padding-top: calc(var(--space-sm) + 4px);
}

.grid-cell.sprint-separator {
  border-top: 3px dashed var(--border-emphasis);
  background: color-mix(in srgb, var(--bg-tertiary) 50%, transparent);
}
```

#### 7. **Add Row/Column Hover Highlights**
**Files:** assets/styling/main.css

**Goal:** Help users understand which engineer + week they're interacting with

- [ ] Add box-shadow on cell hover that highlights entire row and column
- [ ] Use rgba(10, 132, 255, 0.05) for subtle blue tint
- [ ] Apply to both empty and allocated cells
- [ ] Ensure doesn't conflict with paintbrush mode cursor
- [ ] Test with different cell variants (allocated, split, oncall)

```css
.grid-cell:hover {
  box-shadow:
    0 0 0 2px var(--primary-50),
    -2000px 0 0 0 rgba(10, 132, 255, 0.05),
    0 -2000px 2000px 0 rgba(10, 132, 255, 0.05);
  z-index: 1;
}
```

#### 8. **Increase Capacity Text Readability**
**Files:** assets/styling/main.css

**Issue:** Capacity text (4.0 / 12 w) is small, progress bar is subtle

- [ ] Increase `.capacity-text` font-size from 11-12px to 13px
- [ ] Increase font-weight to 600 (semi-bold)
- [ ] Increase `.capacity-bar` height from 4px to 6px
- [ ] Increase `.capacity-bar` width from 80px to 100px
- [ ] Add subtle background tint to over-allocated engineer headers

```css
.capacity-text {
  font-size: 13px;
  font-weight: 600;
}

.capacity-bar {
  width: 100px;
  height: 6px;
}

.grid-engineer-header.over-allocated {
  background: rgba(255, 69, 58, 0.05);
}
```

#### 9. **Paintbrush Mode Visual Feedback**
**Files:** assets/styling/main.css

**Goal:** Clear indication when paintbrush mode is active

- [ ] Add glow effect to FAB when paintbrush mode is active
- [ ] Show selected project color in FAB background
- [ ] Add pulse animation to FAB (2s infinite)
- [ ] Keep crosshair cursor on grid cells (already done)
- [ ] Add subtle border to panel when in paintbrush mode

```css
.floating-paintbrush-fab.active {
  background: var(--project-color);
  box-shadow: 0 0 0 4px rgba(var(--project-color-rgb), 0.2);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4); }
  50% { box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4), 0 0 0 4px var(--primary-50); }
}
```

### Tasks - Priority 3: Polish (Nice to Have)

#### 10. **Empty Cell Hover Refinements**
**Files:** assets/styling/main.css

- [ ] Hide "+" icon by default (opacity: 0)
- [ ] Show "+" only on hover (opacity: 1)
- [ ] In paintbrush mode, show ghost preview of selected project on hover
- [ ] Add subtle scale animation on hover

```css
.grid-cell-empty .empty-icon {
  opacity: 0;
  transition: opacity 0.2s;
}

.grid-cell-empty:hover .empty-icon {
  opacity: 1;
}
```

#### 11. **Cell Transition Animations**
**Files:** assets/styling/main.css

- [ ] Add smooth transitions for cell background color changes
- [ ] Add scale animation when cell is allocated
- [ ] Add fade-in for newly created allocations
- [ ] Keep existing success-pulse animation for paintbrush allocations

#### 12. **Panel Search Functionality**
**Files:** src/components/ui/floating_project_panel.rs

- [ ] Add search input at top of panel
- [ ] Filter projects by name as user types
- [ ] Highlight matching text in project names
- [ ] Show "No results" message when no matches
- [ ] Clear search on Esc key

### Acceptance Criteria
- [x] Application builds successfully
- [x] Floating FAB appears in bottom-right corner
- [x] FAB click opens project selector panel from right side
- [x] Panel shows all projects with color dots and allocated weeks
- [x] Panel includes search functionality
- [x] Click project in panel selects it for paintbrush mode
- [x] FAB shows selected project color when paintbrush mode active
- [x] Panel includes collapse/expand functionality for narrow windows
- [x] Clear button displays complete border
- [x] Collapse button properly centered
- [x] Click outside panel closes it
- [x] All existing grid functionality still works
- [x] Pre-commit checks pass (fmt, clippy, tests, audit)
- [ ] Horizontal paintbrush bar completely removed (80-100px space reclaimed) - **TODO**
- [ ] Context menu "Assign Project" opens same panel - **TODO**
- [ ] Sprint separators clearly visible (3px dashed, higher contrast) - **TODO**
- [ ] Row/column hover highlights work correctly - **TODO**
- [ ] Capacity text is readable (13px, semi-bold) - **TODO**
- [ ] Over-allocated engineers have subtle red background tint - **TODO**
- [ ] Paintbrush mode has clear visual feedback (FAB glow, pulse) - **TODO**
- [ ] Esc key closes panel and exits paintbrush mode - **TODO**

### Manual Testing
**Run:** `dx serve`

1. **FAB + Panel**:
   - Verify FAB appears in bottom-right corner
   - Click FAB → panel slides in from right
   - Verify panel shows all projects with colors
   - Search for project → verify filtering works
   - Click outside panel → verify it closes

2. **Paintbrush Mode via FAB**:
   - Click FAB → panel opens
   - Click "ML Pipeline Optimization" → verify paintbrush mode activates
   - Verify FAB background changes to blue (project color)
   - Verify FAB has pulse animation
   - Click cells → verify allocation works
   - Press Esc → verify paintbrush mode exits, panel closes

3. **Direct Assignment via Context Menu**:
   - Right-click empty cell → "Assign Project"
   - Verify panel opens
   - Click "Auth Service Refactor" → verify cell assigned immediately
   - Verify panel closes after assignment
   - Verify paintbrush mode NOT activated

4. **Visual Polish**:
   - Hover over cells → verify row/column highlights appear
   - Check sprint separators → verify 3px dashed lines visible
   - Check engineer headers → verify capacity text readable
   - Check over-allocated engineer (Dave Roberts) → verify subtle red tint
   - Hover empty cell → verify "+" appears on hover only

5. **Space Reclamation**:
   - Compare grid visible area before/after (should gain ~80-100px)
   - Verify more weeks visible without scrolling
   - Verify no horizontal controls bar above grid

### Documentation Updates
- [ ] Create `docs/adrs/ADR-004-floating-fab-pattern.md` documenting FAB decision
- [ ] Update `docs/ui-design.md` section 6 with FAB specifications
- [ ] Update `docs/component-reference.md` with FloatingFab and FloatingProjectPanel
- [ ] Document unified project selection flow (paintbrush vs direct assign)

---

## Phase 7: Context Menu & Advanced Interactions ⏸️ PARTIALLY COMPLETE
**Goal:** Add right-click context menu, assignment modals, and improved keyboard interactions

### Current Status
**Completed:**
- ✅ `ContextMenu` component created (src/components/ui/context_menu.rs)
- ✅ `SplitAllocationModal` component created (src/components/ui/split_modal.rs)
- ✅ `AssignProjectModal` component created (src/components/ui/assign_project_modal.rs)
- ✅ Basic keyboard shortcuts (Esc, Delete)
- ✅ Context menu action handlers partially implemented

**Remaining:**
- ❌ Complete AssignProjectModal integration and handlers
- ❌ Update copy/paste to work with hover + keyboard only (no context menu)
- ❌ Add keybindings help overlay/tooltip
- ❌ Fix split modal text overflow in project dropdowns
- ❌ Improve split preview visual clarity (color-coded sections)
- ❌ Add "Edit Split" option for existing split cells
- ❌ Wire up cell hover to set `focused_cell` for keyboard ops
- ❌ Add comprehensive CSS for all modals
- ❌ Render modals in allocation_view.rs
- ❌ Test all interactions

### Tasks

#### 1. Complete AssignProjectModal Integration
**Files:** src/components/views/allocation_view.rs, src/components/ui/assign_project_modal.rs

- [x] Create AssignProjectModal component with project list
- [ ] Add modal state (assign_modal_visible, assign_project_id)
- [ ] Wire up "Assign Project" context menu action to open modal
- [ ] Implement project selection handler
- [ ] Implement oncall selection handler
- [ ] Implement apply handler (create Allocation with selected project)
- [ ] Implement cancel handler
- [ ] Render AssignProjectModal in allocation_view RSX

#### 2. Improve Copy/Paste UX (Hover-Based)
**Files:** src/components/views/allocation_view.rs

**Current Issue:** Copy/paste requires context menu, but should work with hover + keyboard only

- [ ] Add `focused_cell` signal to track hovered cell
- [ ] Add `onmouseenter` handler to grid cells to set `focused_cell`
- [ ] Add `onmouseleave` handler to clear `focused_cell` (debounced)
- [ ] Update Cmd/Ctrl+C handler to use `focused_cell` instead of `context_menu_cell`
- [ ] Update Cmd/Ctrl+V handler to use `focused_cell` instead of `context_menu_cell`
- [ ] Update Delete/Backspace handler to use `focused_cell`
- [ ] Remove copy/paste from context menu (keep only: Assign, Split/Edit Split, Clear)
- [ ] Add visual indicator for focused cell (subtle border or glow)

#### 3. Add Keybindings Help Overlay
**Files:** src/components/ui/keybindings_overlay.rs (new), src/components/views/allocation_view.rs

- [ ] Create KeybindingsOverlay component
- [ ] Design overlay layout (floating panel, 2-column grid)
- [ ] Add all keyboard shortcuts with descriptions:
  - **General:** Esc (close modals/exit paintbrush), ? (toggle this help)
  - **Allocation:** Cmd/Ctrl+C (copy hovered cell), Cmd/Ctrl+V (paste to hovered cell)
  - **Editing:** Delete/Backspace (clear hovered cell)
  - **Paintbrush:** Esc (exit mode), Click (allocate), Drag (paint multiple)
- [ ] Add toggle state (show/hide with `?` key)
- [ ] Add keyboard shortcut to toggle (? key)
- [ ] Position in bottom-right corner with slide-in animation
- [ ] Add semi-transparent backdrop
- [ ] Style with design tokens

#### 4. Fix Split Modal Text Overflow
**Files:** assets/styling/main.css

**Issue:** Project names in dropdowns can overflow their containers

- [ ] Add text-overflow CSS to `.project-dropdown option`
- [ ] Set max-width on dropdown
- [ ] Add ellipsis for long project names
- [ ] Ensure percentage displays don't get cut off
- [ ] Add tooltip on hover showing full project name (optional)

#### 5. Improve Split Preview Visual Clarity
**Files:** assets/styling/main.css, src/components/ui/split_modal.rs

**Issue:** Preview bar doesn't clearly show which color belongs to which project

- [ ] Update `.split-preview-project1` and `.split-preview-project2` backgrounds
- [ ] Use actual project colors from selected projects (pass as props)
- [ ] Add project names inside preview sections
- [ ] Increase opacity from 0.3 to 0.5-0.6
- [ ] Add border between the two sections
- [ ] Consider vertical stripes pattern to differentiate from solid cells

#### 6. Add "Edit Split" Context Menu Option
**Files:** src/components/ui/context_menu.rs, src/components/views/allocation_view.rs

- [x] Add `EditSplit` to `MenuAction` enum
- [x] Update ContextMenu to show "Edit Split Allocation..." when cell is already split
- [ ] Pass `is_split` prop to ContextMenu (check if allocation.assignments.len() == 2)
- [ ] Handle EditSplit action (pre-fill modal with current split percentages)
- [ ] Update context menu render in allocation_view.rs

#### 7. Wire Up Cell Hover for Focused State
**Files:** src/components/views/allocation_view.rs

- [ ] Add `onmouseenter` to grid cell wrapper div
- [ ] Set `focused_cell` to (engineer_id, week_start)
- [ ] Add `onmouseleave` to clear `focused_cell` after 100ms delay
- [ ] Add CSS class for focused cell visual feedback
- [ ] Ensure paintbrush mode doesn't conflict with focus state

#### 8. Add Modal CSS
**Files:** assets/styling/main.css

- [ ] Add `.assign-project-modal` specific styles
- [ ] Add `.assign-project-list` grid layout
- [ ] Add `.assign-project-option` button styles
- [ ] Add selected state styles
- [ ] Ensure modal widths are appropriate (400-500px)
- [ ] Add animations (fade-in, scale)
- [ ] Ensure backdrop blur works correctly

#### 9. Test All Interactions
- [ ] Test AssignProjectModal:
  - Right-click → Assign Project
  - Select project from list
  - Select Oncall
  - Apply assignment
  - Cancel
- [ ] Test SplitAllocationModal:
  - Right-click → Split Allocation
  - Select two different projects
  - Adjust slider (0-100%)
  - Verify preview updates with colors
  - Apply split
  - Edit existing split (right-click split cell → Edit Split)
- [ ] Test Copy/Paste:
  - Hover cell, press Cmd/Ctrl+C
  - Hover different cell, press Cmd/Ctrl+V
  - Verify clipboard works across page
- [ ] Test Delete:
  - Hover cell, press Delete/Backspace
  - Verify cell clears
- [ ] Test Keybindings Help:
  - Press ? to toggle overlay
  - Verify all shortcuts listed
  - Press Esc or ? again to close
- [ ] Test on both Mac (Cmd) and Windows/Linux (Ctrl)

### Acceptance Criteria
- [ ] Application builds successfully
- [ ] Right-click opens context menu near cursor
- [ ] AssignProjectModal opens and assigns projects correctly
- [ ] SplitAllocationModal opens and creates/edits splits correctly
- [ ] Percentage slider updates both projects (total = 100%)
- [ ] Split preview shows color-coded sections with project names
- [ ] Copy/paste works with hover + keyboard only
- [ ] Delete/Backspace clears hovered cell
- [ ] Keybindings overlay shows all shortcuts
- [ ] Project names don't overflow in modals
- [ ] "Edit Split" option appears for split cells
- [ ] All keyboard shortcuts work on Mac (Cmd) and Windows/Linux (Ctrl)
- [ ] Visual feedback for focused/hovered cell
- [ ] All tests pass

### Documentation Updates
- [ ] Document keyboard shortcuts in `docs/keyboard-shortcuts.md`
- [ ] Update `docs/interaction-patterns.md` with context menu and modal workflows
- [ ] Add comments explaining hover-based copy/paste UX decision

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
   - Show tooltip after 500ms hover
   - Hide immediately on mouse leave
   - Cancel on mouse out before delay

3. **Add tooltip content for grid cells**
   - Project name
   - Linked roadmap project (clickable)
   - Allocation percentage
   - Project progress (X / Y weeks)
   - Status indicator
   - Hint text ("Cmd+C to copy • Delete to clear")

4. **Add tooltip for engineer headers**
   - Engineer name and role
   - Q1 capacity and utilization
   - Current projects list with allocated weeks
   - Oncall weeks
   - Unallocated weeks

---

## Phase 9: File Operations (Save/Load)
**Goal:** Implement JSON-based file persistence

### Tasks
1. **Create file menu dropdown**
   - File menu button (☰ icon)
   - Dropdown with options
   - Keyboard shortcuts (Cmd+S, Cmd+O, etc.)

2. **Implement JSON serialization**
   - Serialize plan state to JSON
   - Include version field
   - Save quarter config, engineers, projects, allocations

3. **Implement save functionality**
   - Save current plan to file
   - File dialog (platform-specific)
   - Save As with file name input

4. **Implement load functionality**
   - Open file dialog
   - Load and parse JSON
   - Validate file format
   - Update application state

---

## Phase 10: CSV Import/Export
**Goal:** Add CSV import and export functionality

### Tasks
1. **Implement CSV export for roadmap projects**
2. **Implement CSV export for allocation grid**
3. **Implement CSV import for projects**
4. **Add export to Excel (optional)**

---

## Phase 11: Preferences & Settings
**Goal:** Implement user preferences and configuration

### Tasks
1. **Create preferences modal**
2. **Implement preferences persistence**
3. **Add quarter configuration**
4. **Implement sprint start day logic**
5. **Add capacity default setting**

---

## Phase 12: Undo/Redo System
**Goal:** Add undo/redo functionality for allocations

### Tasks
1. **Implement command pattern for state mutations**
2. **Add undo/redo stack**
3. **Implement keyboard shortcuts (Cmd/Ctrl+Z, Cmd/Ctrl+Shift+Z)**
4. **Handle state transitions**

---

## Phase 13: Validation & Alerts
**Goal:** Add comprehensive validation and user feedback

### Tasks
1. **Implement over-allocation warnings**
2. **Add under-allocation alerts**
3. **Implement project deadline validation**
4. **Add save validation**
5. **Create notification/toast system**

---

## Phase 14: Animations & Polish
**Goal:** Add micro-interactions and polish UI

### Tasks
1. **Implement animations**
2. **Add loading states**
3. **Improve hover states**
4. **Add drag & drop for cells**
5. **Implement capacity dashboard visualization**

---

## Phase 15: Testing & Accessibility
**Goal:** Comprehensive testing and accessibility improvements

### Tasks
1. **Write unit tests**
2. **Write integration tests**
3. **Add accessibility features (ARIA labels, keyboard nav)**
4. **Color contrast validation**
5. **Performance optimization**

---

## Phase 16: Final Polish & Documentation
**Goal:** Final refinements and release preparation

### Tasks
1. **Final UI polish**
2. **Cross-platform testing**
3. **Performance optimization**
4. **Comprehensive documentation**
5. **Create example data files**
6. **Record demo video**
7. **Prepare release (v1.0.0)**

---

## Phase 17: Post-Release Enhancements (Future)
- Light mode theme
- Multiple team support
- Project dependencies and Gantt view
- Real-time collaboration
- Integration with JIRA/GitHub
- Advanced reporting and analytics

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
