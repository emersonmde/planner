# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Planner is a Dioxus 0.7 web/desktop application for engineering managers to plan quarterly resource allocation. The app features three main views (Roadmap, Technical Projects, Allocation Grid) with an interactive weekly allocation grid, capacity tracking, and localStorage persistence.

**Status:** Milestone 13 Complete - Preparing for 1.0. The app features a "Plan Menu" (Notion/Linear-style) for file operations: Open, Save, Copy/Paste to clipboard, with keyboard shortcuts (⌘O, ⌘S). Viewing mode displays imported plans with unsaved changes detection. Two-signal architecture (Preferences + PlanState) with localStorage persistence. Full CRUD operations for all entities. See `docs/roadmap.md` for the v1.0 roadmap (3 remaining milestones: M14-16).

## Development Commands

### Building & Running
```bash
# Run web version with hot reload
dx serve

# Run desktop version
dx serve --platform desktop

# Build for production (web)
dx build --release

# Build for specific platform
dx build --platform desktop --release
```

### Testing & Linting
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy (web target)
cargo clippy --target wasm32-unknown-unknown --features web -- -D warnings

# Run clippy (desktop target)
cargo clippy --features desktop --all-targets -- -D warnings

# Run tests
cargo test --verbose --features desktop

# Check for security vulnerabilities
cargo audit

# Generate documentation
cargo doc --no-deps
```

### Pre-commit Hook

The repository includes a pre-commit hook that runs all quality checks before allowing commits:

```bash
# The hook runs automatically before each commit:
# - cargo fmt --check
# - cargo clippy (web and desktop)
# - cargo test
# - cargo doc
# - cargo audit
# - dx bundle --release
```

**Required tools:**
- `cargo-audit`: Install with `cargo install cargo-audit --locked`
- `dioxus-cli`: Install with `cargo install dioxus-cli --locked`

### Continuous Integration

The project uses GitHub Actions for CI/CD (`.github/workflows/ci.yml`):
- **Format/Lint**: Runs on Linux (cheapest) - fmt check, clippy for web target
- **Tests**: Run on macOS (avoids Linux desktop dependency issues)
- **Security Audit**: cargo audit on Linux
- **Web Build**: Linux, deploys to GitHub Pages on main branch
- **Desktop Builds**: macOS (required), Windows (optional, continue-on-error)
- **Deploy**: Only after web build + macOS build + audit pass

**Platform Notes:**
- Tests must run on macOS due to dioxus-desktop Linux dependency conflicts
- Linux desktop removed from CI (ashpd async-std/tokio conflict)
- Windows build is optional and doesn't block deployment

### Project-Specific Notes
- The project uses Cargo features for platform targeting: `web` (default), `desktop`, `mobile`
- CSS files must be loaded via `asset!()` macro, NOT via `@import` in CSS
- Both `theme.css` and `main.css` must be explicitly linked in `main.rs`

## Architecture

### Component Organization

```
src/
├── components/
│   ├── layout/      # TopNav (with sub-components: PlanMenu, ViewTabs, CapacityIndicator)
│   ├── ui/          # Reusable UI primitives (Button, Badge, Input, Modals)
│   └── views/       # Main view components (RoadmapView, TechnicalView, AllocationView)
├── models/          # Data structures (Plan, TeamMember, Projects, Allocations, PlanExport)
├── plan_io.rs       # Platform-specific file I/O (download, clipboard, file reading)
├── state.rs         # App state management (signals, context, ViewingSession)
├── storage.rs       # localStorage persistence (preferences, plan state)
└── utils/           # Helper functions (date calculations, capacity tracking)
```

### Design System

The application uses a comprehensive design token system defined in `assets/styling/theme.css`:

- **Color System**: Based on Apple's dark mode guidelines
  - Backgrounds: Primary (#1c1c1e), Secondary (#2c2c2e), Tertiary (#3a3a3c)
  - Semantic: Primary (blue #0A84FF), Success (green #32D74B), Warning (orange #FF9F0A), Error (red #FF453A)
  - Project Colors: 9-color palette for visual differentiation
- **Typography**: System font stack with 6 size levels
- **Spacing**: 8px base unit, all spacing uses multiples of 4px/8px
- **Transitions**: Three timing functions (quick 150ms, base 250ms, slow 400ms)

### State Management

**Current (M12.5+)**: Two-signal architecture for improved reactivity and persistence
- `use_preferences()`: Team roster, sprint config, default capacity (persisted to localStorage)
- `use_plan_state()`: Projects, allocations, quarter config (persisted to localStorage)
- `PlanExport`: Self-contained format for sharing and future multi-team aggregation (M13)

Both signals auto-save to localStorage on change via `use_effect`. State updates trigger reactive UI updates via Dioxus signals.

### Component Patterns

1. **UI Components** (Button, Badge, Input):
   - Use `#[component]` macro
   - Props include variants and optional event handlers
   - Event handlers use `#[props(default)]` for optional onclick
   - Pass closures with `move |evt|` syntax, not `Some(|_|)`

2. **View Components**:
   - Each view is a separate component in `components/views/`
   - Views use `.view` CSS class with `.active` modifier for visibility
   - State-driven view switching via enum in TopNav

3. **CSS Integration**:
   - Load CSS via `asset!()` macro: `const THEME_CSS: Asset = asset!("/assets/styling/theme.css");`
   - Link in document head: `document::Link { rel: "stylesheet", href: THEME_CSS }`
   - Never use `@import` in CSS files

## Key Documentation

### Getting Started
- **`docs/roadmap.md`**: v1.0 roadmap with M14-16 remaining, acceptance criteria, and post-1.0 vision
- **`docs/ui-design.md`**: Comprehensive UI/UX specification with design tokens, color system, component specs
- **`docs/component-reference.md`**: Implementation examples for each component with HTML/CSS/Dioxus notes
- **`docs/mockup.html`**: Working HTML reference implementation (open in browser to see visual design)

### Architecture & Technical Guides
- **`docs/state-management.md`**: Practical guide for working with Dioxus Signals, common patterns, computed values
- **`docs/grid-architecture.md`**: Technical implementation of allocation grid (rendering pipeline, cell state machine, sticky headers)

### Architecture Decision Records (ADRs)
ADRs document major design decisions and their rationale. Located in `docs/adrs/`:

- **`ADR-001-design-system-structure.md`**: Why CSS design tokens, module organization, dark-mode-first approach
- **`ADR-002-state-management.md`**: Why Dioxus Signals over Redux/multiple signals, alternatives considered
- **`ADR-003-grid-layout.md`**: Why horizontal timeline, CSS Grid choice, sticky header strategy, performance considerations
- **`ADR-004-dark-mode-design-principles.md`**: Dark mode color system, contrast ratios, semantic color usage
- **`ADR-005-state-persistence.md`**: Two-signal architecture, three-tier localStorage persistence, self-contained export format

### Future Documentation
These may be created in upcoming milestones as needed:
- **M15**: Testing documentation (if needed for core tests)
- **M16**: User guide and release notes

## Milestone-Based Development

The project follows a structured milestone approach toward v1.0 release. **Never skip ahead to future milestones.**

**Before marking any milestone complete**, follow the **Milestone Completion Checklist** in `docs/roadmap.md`:
1. Run `.githooks/pre-commit` and fix all errors/warnings
2. Review all changes as a Sr Rust Engineer (code structure, naming, complexity, performance)
3. Build verification (web + desktop targets)
4. Manual testing
5. Documentation updates

**Current Status:** Milestone 13 Complete - Preparing for 1.0
**Next Milestone:** Milestone 14 - Workspace Restructure & CI

**v1.0 Goals** (M14-M16):
- M14: Cargo workspace for proper CI testing and platform separation
- M15: Core unit tests for models and utilities
- M16: Release preparation (version, README, empty states, release artifacts)

**1.0 Success Criteria:**
- Visual allocation grid with paintbrush mode and keyboard shortcuts
- Full CRUD operations for roadmap projects, technical projects, and team members
- Two-signal state architecture with localStorage persistence
- Self-contained plan export/import for sharing and versioning
- Workspace restructure for proper CI/testing
- Core unit tests (models, utils)
- macOS desktop release

**Post-1.0 Vision:**
- Multi-team aggregation (Sr Manager view across multiple teams)
- Windows/Linux desktop releases
- Cloud sync and real-time collaboration

When implementing new milestones:
1. Read the milestone details in `docs/roadmap.md`
2. Review context, design decisions, and dependencies
3. Reference `docs/ui-design.md` for design specifications
4. Check `docs/component-reference.md` for implementation examples
5. Ensure the build passes before moving on

## Data Model

The application manages:
- **TeamMember**: id, name, role (Eng/Sci), capacity (weeks) - stored in Preferences (M9+)
- **RoadmapProject**: id, name, eng/sci estimates, dates, notes, color - stored in PlanState
- **TechnicalProject**: id, name, roadmap link, estimated weeks, dates - stored in PlanState
- **Allocation**: week_start, team_member_id, assignments (project_id + percentage) - stored in PlanState
- **ProjectColor**: enum for visual differentiation (9 colors)

**Current Architecture (M12.5+)**:
- `Preferences`: Team members, sprint config, default capacity - persisted to localStorage
- `PlanState`: Projects, allocations, quarter config (name, start date, weeks) - persisted to localStorage
- `PlanExport` (M13): Self-contained JSON including team snapshot for portability and sharing

See `docs/roadmap.md` for data model details and M13 import/export specifications.

## Dioxus 0.7 Patterns

This project uses **Dioxus 0.7**, which has breaking changes from 0.5/0.6. Key patterns:

### Components
- Use `#[component]` macro (required for all components)
- Component functions must start with capital letter or contain underscore
- Props must be owned values (`String`, not `&str`), must implement `PartialEq` and `Clone`
- Components re-render when props change or reactive state updates

```rust
#[component]
fn MyComponent(name: String, count: i32) -> Element {
    rsx! { "Hello {name}, count: {count}" }
}
```

### State Management
- **`use_signal`** for local state (replaces old `use_state`)
- Call signal like function to clone value: `count()`
- Use `.read()` for reference, `.write()` for mutable reference
- Use `.with_mut()` to mutate: `count.with_mut(|c| *c += 1)`
- **`use_memo`** for derived/computed values
- **`use_context_provider`/`use_context`** for shared state down component tree

```rust
let mut count = use_signal(|| 0);
let doubled = use_memo(move || count() * 2);
```

### RSX Syntax
- Attributes: `class: "container"`
- Inline styles: `color: "red"`
- Conditional attributes: `width: if condition { "100%" }`
- Prefer `for` loops over iterators: `for i in 0..5 { div { "{i}" } }`
- Iterators must be wrapped in braces: `{items.iter().map(...)}`
- Conditionals can contain elements directly: `if show { div { "content" } }`

### Assets
- Always use `asset!()` macro: `asset!("/assets/image.png")`
- Paths start with `/` and are relative to project root
- Use `document::Link` for stylesheets (not `document::Stylesheet`)

### Event Handlers
- Use `move |evt|` syntax
- Access event data: `e.value()`, `e.key()`, etc.
- Example: `oninput: move |e| value.set(e.value())`

### Async/Resources
- **`use_resource`** for async operations (network requests)
- Returns `None` while loading, `Some(value)` when ready
- Automatically re-runs when dependent signals change

```rust
let dog = use_resource(move || async move {
    fetch_dog().await
});

match dog() {
    Some(data) => rsx! { Dog { data } },
    None => rsx! { "Loading..." },
}
```

## Common Pitfalls

1. **CSS Loading**: Must use `asset!()` macro for each CSS file separately, not `@import`
2. **Event Handlers**: Use `move |evt|` directly in props, not wrapped in `Some()`
3. **Component Props**: Optional handlers need `#[props(default)]` attribute
4. **View Switching**: Uses CSS classes (`.view.active`), not conditional rendering
5. **Design Tokens**: Always use CSS variables (e.g., `var(--primary-50)`) not hardcoded colors
6. **Dioxus 0.7 Breaking Changes**: `cx`, `Scope`, and `use_state` are gone - use `use_signal` instead
7. **Signal Reading**: Call as function `count()` to clone, or use `.read()`/`.write()` for references
