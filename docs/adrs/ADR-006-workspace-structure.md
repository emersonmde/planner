# ADR-006: Dioxus Workspace Structure

**Status:** Accepted
**Date:** 2025-11-25
**Context:** Milestone 14 - Workspace Restructure

## Decision

Organize the Dioxus project as a Cargo workspace with two crates:
- `planner-core`: Platform-independent models and utilities
- `planner-app`: Dioxus UI application (platform-specific)

## Context

### Problem

Single-crate Dioxus applications face CI/testing challenges:

1. **Platform Dependencies**: `dioxus-desktop` pulls in platform-specific dependencies (GTK on Linux, WebKit, etc.) that conflict with each other or require specific system libraries
2. **Expensive CI**: Tests must run on macOS runners (~10x cost of Linux) because Linux desktop dependencies conflict
3. **No Separation**: Platform-independent logic (models, date calculations) is mixed with UI code

### Alternatives Considered

#### Option A: Default Dioxus Workspace (dx new --template workspace)

```
packages/
├── api/      # Pure Rust - server functions
├── ui/       # Shared Dioxus components
├── web/      # Web entry point + views
├── desktop/  # Desktop entry point + views (duplicated)
└── mobile/   # Mobile entry point + views (duplicated)
```

**Pros:**
- Each platform has its own binary crate
- No feature flags needed
- Clean per-platform customization
- Official Dioxus recommendation

**Cons:**
- Views duplicated across platforms (3x maintenance)
- Assets duplicated per platform
- 5 crates for a single app
- Designed for fullstack SSR apps (api crate)

#### Option B: Two-Crate Workspace (Chosen)

```
crates/
├── planner-core/   # Models, utils (no dioxus deps)
└── planner-app/    # Single app with feature flags
```

**Pros:**
- No code duplication
- Single entry point
- Core tests run on Linux CI (cheap, fast)
- Simpler: just 2 crates
- Assets in one place

**Cons:**
- Feature flags for platform switching
- Requires `-p planner-app` flag for dx commands
- Less flexibility for per-platform customization

#### Option C: Three-Crate Workspace

```
crates/
├── planner-core/   # Models, utils
├── planner-ui/     # Shared Dioxus components
└── planner-app/    # Entry point only
```

**Pros:**
- Maximum separation
- UI components testable separately

**Cons:**
- More complexity for minimal benefit
- UI components rarely need separate testing

## Decision Rationale

**Option B** is best for apps where:
1. Web and desktop share 95%+ of code
2. Platform-specific code is minimal (just I/O operations)
3. You're not using fullstack/SSR features
4. You want core business logic testable on cheap Linux CI

The default Dioxus workspace (Option A) is better for:
1. Apps that diverge significantly per platform
2. Fullstack apps with server functions
3. Teams that want maximum per-platform flexibility

## Implementation

### Directory Structure

```
planner/
├── Cargo.toml                    # [workspace] members = ["crates/*"]
├── Dioxus.toml                   # Web/desktop config
├── crates/
│   ├── planner-core/
│   │   ├── Cargo.toml            # chrono, serde, uuid (no dioxus)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models/           # Data structures
│   │       └── utils/            # Date helpers, calculations
│   └── planner-app/
│       ├── Cargo.toml            # dioxus, planner-core
│       ├── assets/               # CSS, images (moved here)
│       └── src/
│           ├── main.rs
│           ├── components/
│           ├── state.rs
│           ├── storage/
│           └── plan_io.rs        # Platform-specific I/O
```

### Cargo.toml (Workspace Root)

```toml
[workspace]
resolver = "2"
members = ["crates/planner-core", "crates/planner-app"]

[workspace.dependencies]
dioxus = { version = "0.7" }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### planner-core/Cargo.toml

```toml
[package]
name = "planner-core"

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
# NO dioxus dependency - keeps this testable anywhere
```

### planner-app/Cargo.toml

```toml
[package]
name = "planner-app"

[dependencies]
planner-core = { path = "../planner-core" }
dioxus = { version = "0.7" }
# ... platform-specific deps with cfg()

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
```

### Import Pattern

```rust
// In planner-app code, use planner_core for models/utils
use planner_core::models::{PlanState, TeamMember};
use planner_core::utils::generate_quarter_weeks;
```

## CI Configuration

The key benefit is splitting tests by platform dependency:

```yaml
jobs:
  test-core:
    runs-on: ubuntu-latest  # Cheap!
    steps:
      - run: cargo test -p planner-core --verbose

  test-app:
    runs-on: macos-latest  # Expensive, but necessary
    steps:
      - run: cargo test -p planner-app --features desktop --verbose

  build-web:
    runs-on: ubuntu-latest
    steps:
      - run: dx bundle -p planner-app --release
```

## CLI Commands

Dioxus CLI requires explicit package specification from workspace root:

```bash
# Development
dx serve -p planner-app                    # web
dx serve -p planner-app --platform desktop # desktop

# Build
dx build -p planner-app --release
dx bundle -p planner-app --release

# Testing
cargo test -p planner-core    # Linux CI - fast, cheap
cargo test -p planner-app --features desktop  # macOS only
```

**Note:** The `sub_package` Dioxus.toml option does not work in dx 0.7. The `-p` flag is required.

## Consequences

### Positive
- Core business logic tests run on Linux CI (21 tests, ~2s)
- Clear separation of concerns
- Faster CI feedback loop
- Lower CI costs
- Easier to reason about dependencies

### Negative
- Must remember `-p planner-app` for dx commands
- Assets moved into app crate (not at workspace root)
- Slightly more complex import paths

### Neutral
- Same total lines of code
- Same functionality
- No runtime performance impact

## When to Use This Pattern

**Use two-crate workspace when:**
- Your app is 90%+ shared code across platforms
- Platform-specific code is isolated (I/O, storage)
- You want cheap Linux CI for core logic
- You're building a client-only app (no SSR)

**Use default Dioxus workspace when:**
- Platforms need significantly different UIs
- You're building fullstack with server functions
- You want completely separate platform binaries
- Per-platform asset customization is important

## References

- [Dioxus GitHub Discussion #3304](https://github.com/DioxusLabs/dioxus/discussions/3304) - Using dx from workspace root
- [Dioxus CLI Documentation](https://dioxuslabs.com/learn/0.7/CLI)
- Cargo Workspaces: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
