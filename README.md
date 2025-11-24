# Quarterly Planner

A quarterly planning tool for engineering managers, built with Rust and Dioxus 0.7.

## Overview

Quarterly Planner helps engineering managers plan and visualize team member allocation across a quarter. It provides three coordinated views:

- **Roadmap View**: High-level product initiatives with engineering/science estimates
- **Technical View**: Detailed implementation projects with status tracking and filtering
- **Allocation Grid**: Interactive weekly allocation matrix with paintbrush mode

### Current Features

- Interactive allocation grid with paintbrush mode for rapid weekly planning
- Context menu and keyboard shortcuts (copy/paste, split allocation)
- Real-time capacity tracking and utilization calculations
- Auto-updating project dates based on allocations
- Hover tooltips with project and capacity details
- Full CRUD operations for roadmap projects with modal-based editing
- Color-coded projects with 9-color palette
- Sprint boundary visualization
- Two-signal reactive architecture (preferences + plan state)
- localStorage persistence for team configuration

## Architecture

**Tech Stack:**
- Rust (stable)
- Dioxus 0.7 (reactive UI framework)
- WebAssembly (primary target)
- Desktop support via Dioxus desktop feature

**State Management:**
- Two independent signals: `use_preferences()` and `use_plan_state()`
- Preferences (team roster, sprint config) persist to localStorage
- Plan state (projects, allocations) designed for export/import
- See [docs/adrs/ADR-004-state-persistence.md](docs/adrs/ADR-004-state-persistence.md)

**Design System:**
- Dark-mode-first CSS design tokens
- 8px spacing system
- Apple-inspired color palette
- See [docs/ui-design.md](docs/ui-design.md)

## Development

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli --locked

# Install cargo-audit (for pre-commit hook)
cargo install cargo-audit --locked

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Running

```bash
# Web (hot reload enabled)
dx serve

# Desktop
dx serve --platform desktop

# Production build
dx build --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy (web target)
cargo clippy --target wasm32-unknown-unknown --features web -- -D warnings

# Run clippy (desktop target)
cargo clippy --features desktop --all-targets -- -D warnings

# Run tests
cargo test --verbose --features desktop

# Security audit
cargo audit

# All checks (via pre-commit hook)
.githooks/pre-commit
```

### Project Structure

```
planner/
├── src/
│   ├── components/
│   │   ├── layout/          # TopNav, layout components
│   │   ├── ui/              # Reusable UI primitives (Button, Badge, Input, ColorPicker)
│   │   └── views/           # Main views (RoadmapView, TechnicalView, AllocationView)
│   ├── models/              # Data structures (Plan, Projects, Allocations, Preferences)
│   ├── utils/               # Helpers (date calculations, capacity tracking)
│   └── main.rs
├── assets/
│   └── styling/             # CSS files (theme.css, main.css)
├── docs/
│   ├── roadmap.md           # Development roadmap and milestones
│   ├── ui-design.md         # Design system specification
│   ├── component-reference.md
│   ├── state-management.md
│   ├── grid-architecture.md
│   └── adrs/                # Architecture Decision Records
└── .githooks/               # Git hooks for quality checks
```

## Documentation

- **[UI Design](docs/ui-design.md)**: Design tokens, component specifications
- **[Component Reference](docs/component-reference.md)**: Implementation examples
- **[State Management](docs/state-management.md)**: Dioxus Signals patterns
- **[Grid Architecture](docs/grid-architecture.md)**: Allocation grid implementation
- **[ADRs](docs/adrs/)**: Architecture Decision Records
- **[Roadmap](docs/roadmap.md)**: Development plan and milestones

## License

MIT - See [LICENSE](LICENSE) for details.
