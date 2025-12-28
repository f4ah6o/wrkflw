# AGENTS.md

This file provides guidance for AI agents (Claude, OpenAI, etc.) working with the WRKFLW codebase.

## Project Overview

WRKFLW is a CLI tool for validating and executing GitHub Actions workflows and GitLab CI/CD pipelines locally. It is written in Rust and provides:

* Local workflow validation before pushing to remote
* Local workflow execution using Docker or sandboxed emulation
* VSCode extension for real-time editor diagnostics
* WebAssembly bindings for browser-based validation

## Architecture

### Workspace Structure

```
wrkflw/                              # Main CLI binary
├── crates/
│   ├── wrkflw/                     # Main binary entry point
│   ├── executor/                   # Workflow execution engine
│   ├── runtime/                    # Docker and emulation runtimes
│   ├── parser/                     # YAML parsing
│   ├── evaluator/                  # Expression evaluation
│   ├── validators/                 # Structural validation
│   ├── models/                     # Shared data structures
│   ├── matrix/                     # Matrix build expansion
│   ├── wasm/                       # WebAssembly bindings
│   ├── github/                     # GitHub API integration
│   ├── gitlab/                     # GitLab API integration
│   ├── ui/                         # Terminal UI (ratatui)
│   ├── logging/                    # Logging utilities
│   ├── secrets/                    # Secrets management
│   └── utils/                      # Shared utilities
└── vscode-wrkflw/                  # VSCode extension (TypeScript)
```

### Key Dependencies

* `clap` - CLI argument parsing
* `ratatui` - Terminal UI framework
* `bollard` - Docker API client
* `serde` / `serde_yaml` - Serialization
* `wasm-bindgen` - WebAssembly bindings
* `thiserror` - Error handling

## Build & Development

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p <crate-name>    # Build specific crate

# Test
cargo test                     # Run all tests
cargo test -p <crate-name>     # Test specific crate

# Lint & Format (run before committing)
cargo clippy                   # Lint check
cargo fmt                      # Format code

# Run locally
cargo run                              # Launch TUI
cargo run -- validate                  # Validate workflows
cargo run -- run path/to/workflow.yml  # Execute a workflow
cargo run -- validate --json           # JSON output
cargo run -- run --json-output ...     # NDJSON streaming
```

## VSCode Extension

Located in `vscode-wrkflw/`:

```bash
cd vscode-wrkflw
pnpm install
pnpm build          # Build extension
pnpm test           # Run tests
```

## Code Conventions

* Use `thiserror` for custom error types
* Use `?` operator for error propagation
* Avoid `.unwrap()` in production code
* Run `cargo clippy` and `cargo fmt` before committing
* Follow Rust API guidelines for public APIs

## Important Constraints

* **No Podman support** - Removed in commit a532a3b
* **Runtime modes**: Docker, Emulation, SecureEmulation (3 modes, not 4)
* **Linux only** - Windows/macOS runners are not supported
* **JSON output flags**: `--json` (validate), `--json-output` (run)

## Common Tasks

### Adding a New CLI Flag

1. Edit `crates/wrkflw/src/main.rs`
2. Add the argument to the appropriate `Commands` enum variant
3. Handle the new flag in the command processing logic

### Adding a New Validator

1. Create validation function in `crates/validators/src/lib.rs`
2. Call from appropriate validation context
3. Add tests in `tests/` directory

### Modifying Workflow Execution

1. Core execution is in `crates/executor/src/engine.rs`
2. Runtime-specific code is in `crates/runtime/src/`
3. Job scheduling logic is in `crates/executor/src/scheduler.rs`

## Testing

* Integration tests are in `tests/`
* Fixtures are in `tests/fixtures/` and `tests/workflows/`
* Unit tests are co-located with source code in `src/` directories

## Documentation Updates

When modifying features:

1. Update `README.md` (user-facing changes)
2. Update `README.ja.md` (Japanese translation)
3. Update `CLAUDE.md` (architecture/developer changes)
4. Update this file (`AGENTS.md`) if workflow/conventions change
