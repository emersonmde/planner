# Quarterly Planner - Development Roadmap

## Progress Status

**Current Status:** Milestone 15 Complete - Core Tests Done

- ✅ Milestones 1-15 Complete (core functionality + workspace + tests)
- 📋 **Milestone 16**: Final 1.0 release preparation

**Build Status:** ✅ Compiles successfully (web + macOS desktop)
**Test Status:** ✅ 45 tests passing in planner-core

---

## Product Vision & 1.0 Goals

**Core Purpose:** Enable engineering managers to plan quarterly resource allocation through an intuitive, interactive interface with three coordinated views (Roadmap, Technical Projects, Allocation Grid).

**1.0 Success Criteria:**
- ✅ Visual allocation grid with paintbrush mode and keyboard shortcuts
- ✅ Full CRUD operations for roadmap projects, technical projects, and team members
- ✅ Two-signal state architecture (persistent preferences + exportable plan state)
- ✅ Self-contained plan export/import for sharing and versioning
- ✅ Settings modal with plan and sprint configuration
- ✅ Workspace restructure for proper CI/testing
- 🎯 Core unit tests (models, utils)
- 🎯 macOS desktop release

**Post-1.0 Vision:**
- Multi-team aggregation (Sr Manager view across multiple teams)
- Windows/Linux desktop releases
- Cloud sync and real-time collaboration

---

## Completed Milestones (1-13) - Summary

### M1-4: Foundation & Core Views
- Dioxus 0.7 setup, design system (theme.css), component library
- Three views: Roadmap, Technical Projects, Allocation Grid
- View switching via TopNav, CSS design tokens

### M5-6: Interactive Allocation Grid
- Side panel with filters, sorting, and search
- Paintbrush mode for rapid weekly allocation
- Rotated grid: engineers as columns, weeks as rows
- Floating FAB for paintbrush mode with slide-out project selector

### M7-8: Interactions & Polish
- Right-click context menu (Assign, Split, Clear)
- Keyboard shortcuts: Copy/Paste (Cmd+C/V), Delete, keybindings help (`?`)
- Split allocation modal with live preview
- Hover tooltips for grid cells and engineer headers
- Auto-update project dates based on allocations

### M9: State Architecture
- **Two-signal architecture**: `use_preferences()` + `use_plan_state()`
- localStorage persistence for team preferences
- PlanExport model for self-contained export format
- Schema versioning for future migration support

### M10-12: CRUD Operations
- Full CRUD for roadmap projects, technical projects, team members
- Modal-based editing pattern with inline validation
- Cascade behavior on delete (removes related allocations)
- Color inheritance from linked roadmap projects

### M12.5: Settings & Configuration
- Settings modal with plan configuration (name, start date, weeks)
- Sprint configuration (anchor date, length)
- Storage management (load sample data, clear preferences)

### M13: Plan Import/Export
- "Plan Title as Menu" pattern (like Notion/Linear)
- Plan Menu dropdown: Open Plan, Save Plan, Copy/Paste to clipboard
- Viewing mode indicator when viewing imported plan
- Unsaved changes indicator (orange dot)
- Keyboard shortcuts: Cmd+O (Open), Cmd+S (Save)
- Self-contained PlanExport includes team snapshot for portability

---

## Milestone 14: Workspace Restructure & CI ✅

**Goal:** Convert to cargo workspace for proper CI testing and platform separation

**Status:** ✅ Complete
**Completed:** 2025-11-25

### Context

The current single-crate structure causes CI issues:
- Linux CI can't test because dioxus-desktop pulls in platform-specific deps
- Tests run on macOS only, which is expensive
- No clear separation between platform-independent and platform-specific code

### Solution: Cargo Workspace

```
planner/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── planner-core/       # Platform-independent: models, utils
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── planner-app/        # Dioxus UI: components, state, storage
│       ├── Cargo.toml
│       └── src/main.rs
├── .github/workflows/ci.yml
└── docs/
```

### Tasks

#### 14.1: Create Workspace Structure
- Create `crates/planner-core/` with models and utils
- Create `crates/planner-app/` with UI code
- Update root `Cargo.toml` as workspace
- Move `src/models/` and `src/utils/` to core crate
- Update imports in app crate to use `planner_core::`

#### 14.2: Update CI
- Core tests run on Linux (cheap, fast)
- App builds run on macOS (native desktop target)
- Web build on Linux for GH Pages deployment
- Windows build optional (continue-on-error)

#### 14.3: Update Pre-commit
- Run core tests
- Run clippy on both crates
- Build web and desktop

### Acceptance Criteria
- [x] Workspace compiles successfully
- [x] `cargo test -p planner-core` runs on Linux CI
- [x] `cargo build -p planner-app --features desktop` works on macOS
- [x] Web deployment to GH Pages works
- [x] Pre-commit hook passes locally

---

## Milestone 15: Core Tests ✅

**Goal:** Add unit tests for platform-independent code

**Status:** ✅ Complete
**Completed:** 2025-11-25

### Context

With workspace restructure complete, we can test `planner-core` on any platform. Focus on testing the core logic that handles data integrity.

### Tasks

#### 15.1: Model Tests ✅
- Plan calculations (allocated weeks, capacity utilization)
- PlanExport serialization/deserialization round-trip
- Validation logic (team member, project, allocation)
- Schema version handling (backward compatibility)

#### 15.2: Utility Tests ✅
- Date helpers (week calculations, sprint boundaries, quarter dates)
- Capacity calculations

#### 15.3: CI Integration ✅
- Run tests in CI on Linux
- Coverage reporting (optional)

### Test Coverage Summary (45 tests)

| Module | Tests | Coverage |
|--------|-------|----------|
| plan.rs | 16 | Allocation methods, capacity calculations, get_capacity_status |
| plan_state.rs | 10 | PlanState lookups, allocation calculations, date updates |
| plan_export.rs | 6 | Export/import round-trip, validation |
| preferences.rs | 7 | Creation, defaults, validation, schema versioning |
| date_helpers.rs | 6 | Week calculations, sprint boundaries, quarter dates |

### Acceptance Criteria
- [x] 80%+ test coverage on `planner-core`
- [x] All tests pass in CI
- [x] Export/import round-trip tested
- [x] Edge cases covered (empty data, boundary dates)

---

## Milestone 16: Release Preparation

**Goal:** Prepare and publish v1.0 release

**Status:** 📋 Not Started
**Estimated Effort:** 1 day

### Tasks

#### 16.1: Version & Documentation
- Update `Cargo.toml` version to 1.0.0
- Update README with current features and screenshots
- Ensure all docs reflect current state

#### 16.2: Empty States
- No team members: "Add your first team member to get started"
- No projects: "Create a roadmap project to begin planning"
- No allocations: "Use paintbrush mode to allocate work"

#### 16.3: Release Artifacts
- Tag git commit: `v1.0.0`
- Build web bundle and verify GH Pages deployment
- Build macOS desktop binary
- Create GitHub release with changelog

### Acceptance Criteria
- [ ] Version 1.0.0 in Cargo.toml
- [ ] README is up-to-date
- [ ] Empty states provide helpful guidance
- [ ] GH Pages deployment working
- [ ] GitHub release published with macOS binary

---

## Post-1.0 Roadmap

### v1.1 - Quality of Life
- Light mode theme
- Batch operations (assign project to multiple weeks)
- Advanced search/filter in views
- Undo/redo system (if user demand)

### v1.2 - Platform Expansion
- Windows desktop release
- Linux desktop release (if ashpd issues resolved)
- Export to Google Sheets / Excel

### v2.0 - Multi-Team Aggregation
Sr Managers can load multiple team plans and view organization-level capacity.

**Already supported by 1.0 architecture:**
- Self-contained PlanExport includes `team_name` and full roster
- No data model changes needed
- Pure client-side aggregation

---

## Milestone Completion Checklist

Before marking any milestone as complete:

1. **Pre-commit Hook**: Run `.githooks/pre-commit` - all checks must pass
2. **Code Review**: Review changes for quality, naming, error handling
3. **Build Verification**: Web and desktop targets compile
4. **Manual Testing**: Test new functionality in browser/desktop
5. **Documentation**: Update roadmap and CLAUDE.md

---

## Development Guidelines

### State Management (Two-Signal Architecture)

```rust
// Read preferences (team config, persisted to localStorage)
let preferences = use_preferences();
let team = preferences().team_members;

// Read plan state (projects, allocations, exportable)
let plan_state = use_plan_state();
let projects = plan_state().roadmap_projects;

// Mutate
plan_state.write().allocations.push(new_allocation);
preferences.write().team_members.push(new_member);
```

### Project Structure (After M14)

```
planner/
├── Cargo.toml             # Workspace root
├── Dioxus.toml            # Web/desktop config (use `dx serve -p planner-app`)
├── crates/
│   ├── planner-core/      # Models, utils (no platform deps)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/
│   │       └── utils/
│   └── planner-app/       # Dioxus UI (platform-specific)
│       ├── assets/        # CSS, images
│       └── src/
│           ├── main.rs
│           ├── components/
│           ├── state.rs
│           ├── storage/
│           └── plan_io.rs
├── docs/                  # Documentation
└── .github/workflows/     # CI configuration
```
