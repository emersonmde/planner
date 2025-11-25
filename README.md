# Quarterly Planner

A quarterly planning tool for engineering managers, built with Rust and Dioxus 0.7.

## Overview

Quarterly Planner helps engineering managers plan and visualize team member allocation across a quarter. It provides three coordinated views:

- **Roadmap View**: High-level product initiatives with engineering/science estimates
- **Technical View**: Detailed implementation projects linked to roadmap items
- **Allocation Grid**: Interactive weekly allocation matrix with paintbrush mode

## Features

### Core Functionality
- Interactive allocation grid with paintbrush mode for rapid weekly planning
- Context menu and keyboard shortcuts (Cmd+C/V copy/paste, Delete, `?` for help)
- Split allocation support (assign multiple projects to same week)
- Real-time capacity tracking and utilization calculations
- Auto-updating project dates based on allocations
- Hover tooltips with project and capacity details

### Data Management
- Full CRUD operations for roadmap projects, technical projects, and team members
- Color-coded projects with 9-color palette
- Sprint boundary visualization
- Settings modal for plan and sprint configuration

### Import/Export
- Plan Menu (Notion/Linear-style) for file operations
- Open/Save plans as JSON files (Cmd+O, Cmd+S)
- Copy/Paste plans to clipboard for easy sharing
- Self-contained exports include team snapshot for portability
- Viewing mode indicator when viewing imported plans
- Unsaved changes detection (orange dot indicator)

### Architecture
- Two-signal reactive architecture (preferences + plan state)
- localStorage persistence for team configuration and plan state
- Cargo workspace with separate core library (testable on any platform)
- 45 unit tests covering models, utilities, and business logic

## Quick Start

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli --locked

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Running

```bash
# Web (hot reload enabled)
dx serve -p planner-app

# Desktop (macOS)
dx serve -p planner-app --platform desktop

# Production build
dx build -p planner-app --release
```

## Development

### Project Structure

```
planner/
├── Cargo.toml              # Workspace root
├── Dioxus.toml             # Dioxus configuration
├── crates/
│   ├── planner-core/       # Platform-independent models & utils
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/     # Data structures (Plan, Projects, Allocations)
│   │       └── utils/      # Date helpers, capacity calculations
│   └── planner-app/        # Dioxus UI application
│       ├── assets/         # CSS files (theme.css, main.css)
│       └── src/
│           ├── main.rs
│           ├── components/ # UI components (layout, views, ui primitives)
│           ├── state.rs    # App state management
│           ├── storage.rs  # localStorage persistence
│           └── plan_io.rs  # Platform-specific file I/O
├── docs/                   # Documentation
└── .github/workflows/      # CI configuration
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -p planner-core
cargo clippy -p planner-app --target wasm32-unknown-unknown --features web

# Run tests (core library only - no platform deps)
cargo test -p planner-core

# All checks (via pre-commit hook)
.githooks/pre-commit
```

### CI/CD

The project uses GitHub Actions:
- **Core tests**: Run on Linux (fast, cheap)
- **App builds**: Run on macOS (required for desktop)
- **Web deployment**: Automatic to GitHub Pages on main branch

## Documentation

- **[Roadmap](docs/roadmap.md)**: Development plan and milestones
- **[UI Design](docs/ui-design.md)**: Design tokens, component specifications
- **[State Management](docs/state-management.md)**: Dioxus Signals patterns
- **[Grid Architecture](docs/grid-architecture.md)**: Allocation grid implementation
- **[ADRs](docs/adrs/)**: Architecture Decision Records

## Tech Stack

- **Rust** (stable)
- **Dioxus 0.7** (reactive UI framework)
- **WebAssembly** (primary web target)
- **Desktop** support via Dioxus desktop feature (macOS)

## License

MIT - See [LICENSE](LICENSE) for details.
