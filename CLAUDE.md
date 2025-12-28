# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WRKFLW is a CLI tool for validating and executing GitHub Actions workflows and GitLab CI/CD pipelines locally. It supports Docker, Podman, and emulation mode for container execution.

## Build & Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build
cargo build -p <crate-name>    # Build specific crate

# Test
cargo test                     # Run all tests
cargo test --lib               # Unit tests only
cargo test -p <crate-name>     # Test specific crate
cargo test test_name           # Run specific test

# Lint & Format
cargo clippy                   # Lint check
cargo fmt                      # Format code

# Run locally
cargo run                              # Launch TUI (default)
cargo run -- validate                  # Validate workflows in .github/workflows
cargo run -- run path/to/workflow.yml  # Execute a workflow
cargo run -- --verbose run ...         # Verbose output
cargo run -- --debug run ...           # Debug output
```

## Architecture

Rust workspace with 14 crates in `crates/`:

```
wrkflw (main binary)
├── executor     # Workflow execution engine, job scheduling
├── runtime      # Container/emulation runtime (docker, podman, sandbox)
├── parser       # YAML parsing for GitHub/GitLab workflows
├── evaluator    # Workflow validation and expression evaluation
├── validators   # Structural validation rules
├── models       # Data structures (ValidationResult, GitLab Pipeline)
├── matrix       # Matrix build expansion
├── github       # GitHub API integration (trigger workflows)
├── gitlab       # GitLab API integration (trigger pipelines)
├── ui           # TUI interface (ratatui-based)
├── logging      # Logging utilities
├── secrets      # Secrets management
└── utils        # Shared utilities
```

### Key Types

* `RuntimeType` (executor): Docker, Podman, Emulation, SecureEmulation
* `ExecutionConfig` (executor): Runtime settings for workflow execution
* `JobResult`, `StepResult` (executor): Execution results
* `ValidationResult` (models): Validation outcome with issues list

### Execution Flow

1. CLI parses args → `Commands` enum in `main.rs`
2. `wrkflw_parser` parses YAML workflow/pipeline
3. `wrkflw_executor::execute_workflow()` orchestrates execution
4. Jobs run via `runtime` module (container or emulation)
5. Automatic cleanup on Ctrl+C via signal handler

## Testing

Integration tests are in `tests/`:
* `matrix_test.rs` - Matrix expansion
* `reusable_workflow_test.rs` - Reusable workflow validation
* `cleanup_test.rs` - Docker resource cleanup

Test fixtures in `tests/fixtures/` and `tests/workflows/`.

## Conventions

* Use `thiserror` for custom error types
* Use `?` operator for error propagation
* Avoid `.unwrap()` in production code
* Run `cargo clippy` and `cargo fmt` before committing
