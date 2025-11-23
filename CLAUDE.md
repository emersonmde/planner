# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Quarterly Planner is a Dioxus 0.7 web/desktop application for engineering managers to plan quarterly resource allocation. The app features three main views (Roadmap, Technical Projects, Allocation Grid) with an interactive weekly allocation grid, capacity tracking, and file-based persistence.

**Status:** Milestone 9 Complete (State Architecture Refactor). The app now uses a two-signal architecture (Preferences + PlanState) for better reactivity and localStorage persistence. Includes interactive allocation grid, paintbrush mode, tooltips, and self-contained export format ready for M13. See `docs/roadmap.md` for the complete v1.0 roadmap (17 milestones).

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

The project uses GitHub Actions for CI/CD:
- **On push/PR**: Runs tests, fmt, clippy, and cargo audit
- **On push to main**: Builds WASM bundle and deploys to GitHub Pages
- **Location**: `.github/workflows/ci.yml`

All CI checks mirror the pre-commit hook to ensure consistency.

### Project-Specific Notes
- The project uses Cargo features for platform targeting: `web` (default), `desktop`, `mobile`
- CSS files must be loaded via `asset!()` macro, NOT via `@import` in CSS
- Both `theme.css` and `main.css` must be explicitly linked in `main.rs`

## Architecture

### Component Organization

```
src/
├── components/
│   ├── layout/      # TopNav, future: SidePanel
│   ├── ui/          # Reusable UI primitives (Button, Badge, Input)
│   └── views/       # Main view components (RoadmapView, TechnicalView, AllocationView)
├── models/          # Data structures (Plan, TeamMember, Projects, Allocations)
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

**Current (M1-8)**: Single-signal architecture with `use_plan_state()` hook
**Planned (M9)**: Two-signal architecture for improved reactivity and persistence
- `use_preferences()`: Team roster, sprint config (persisted to localStorage)
- `use_plan_state()`: Projects, allocations (exported/imported per quarter)
- `PlanExport`: Self-contained format for sharing and future multi-team aggregation

State updates trigger reactive UI updates via Dioxus signals.

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
- **`docs/roadmap.md`**: Complete v1.0 roadmap with 17 milestones, acceptance criteria, and post-1.0 vision
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

### Future Documentation
These will be created in upcoming milestones:
- **M9**: `docs/adrs/ADR-004-state-persistence.md` - State architecture, persistence strategy, and self-contained export format
- **M13**: `docs/file-format.md` - PlanExport JSON schema and import/export specifications
- **M14**: `docs/validation.md` - Validation rules, error messages, and user feedback patterns
- **M16**: `docs/testing.md` - Testing strategy, coverage goals, and accessibility testing
- **M17**: `docs/user-guide.md` - End-user documentation with workflows and keyboard shortcuts

## Milestone-Based Development

The project follows a structured milestone approach toward v1.0 release. **Never skip ahead to future milestones.** Each milestone must:
1. Build successfully (`dx build`)
2. Meet all acceptance criteria
3. Be manually tested
4. Have documentation updated

**Current Status:** Milestone 9 Complete (State Architecture Refactor)
**Next Milestone:** Milestone 10 - Roadmap Projects Management (CRUD Operations)

**v1.0 Goals** (M9-M17):
- Two-signal state architecture (preferences + plan_state)
- Full CRUD operations for roadmap projects, technical projects, and team members
- Self-contained plan export/import for sharing and versioning
- Undo/redo system for all operations
- Comprehensive validation and user feedback
- Accessibility compliance (WCAG AA)
- Production-ready polish and documentation

**Post-1.0 Vision:**
- Multi-team aggregation (Sr Manager view across multiple teams)
- Cloud sync and real-time collaboration
- Advanced reporting and analytics

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

**Current (M8)**: Single `Plan` struct with all data
**Planned (M9)**: Split into `Preferences` (localStorage) and `PlanState` (exported/imported)
**Export Format (M9+)**: `PlanExport` - self-contained JSON including team snapshot for portability

File format: See `docs/roadmap.md` Milestone 9 for data model refactor details

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
