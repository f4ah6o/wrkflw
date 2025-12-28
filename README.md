# WRKFLW

[![Crates.io](https://img.shields.io/crates/v/wrkflw)](https://crates.io/crates/wrkflw)
[![Rust Version](https://img.shields.io/badge/rust-1.67%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/wrkflw)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/bahdotsh/wrkflw/build.yml?branch=main)](https://github.com/bahdotsh/wrkflw/actions/workflows/build.yml)

A CLI tool for validating and executing GitHub Actions workflows and GitLab CI/CD pipelines locally—before pushing to remote.

![WRKFLW Demo](demo.gif)

## Overview

WRKFLW enables local development and testing of CI/CD workflows without pushing changes to GitHub or GitLab. It parses workflow files, validates syntax, and executes jobs in dependency order using Docker containers or native emulation.

**Key benefits:**

- Fast feedback loop—test workflows locally without remote commits
- Catch configuration errors before they reach CI/CD
- Debug workflow failures interactively with container inspection
- Support for both GitHub Actions and GitLab CI/CD

## Installation

### Via Cargo (Recommended)

```bash
cargo install wrkflw
```

### From Source

```bash
git clone https://github.com/bahdotsh/wrkflw.git
cd wrkflw
cargo build --release
# Binary at target/release/wrkflw
```

## Quick Start

```bash
# Navigate to your project
cd your-project

# Launch TUI (auto-detects .github/workflows)
wrkflw

# Or validate workflows
wrkflw validate

# Or run a specific workflow
wrkflw run .github/workflows/ci.yml
```

## Usage

### Validation

Validate workflow syntax and structure:

```bash
# Validate all workflows in .github/workflows
wrkflw validate

# Validate specific file
wrkflw validate .github/workflows/ci.yml

# Validate with verbose output
wrkflw validate --verbose path/to/workflow.yml

# Validate GitLab CI pipelines
wrkflw validate .gitlab-ci.yml --gitlab

# Output validation results as JSON (for CI/IDE integration)
wrkflw validate --json .github/workflows/ci.yml
```

**Exit codes:** `0` (success), `1` (validation failed), `2` (usage error)

### Execution

Run workflows locally:

```bash
# Run with Docker (default)
wrkflw run .github/workflows/ci.yml

# Run in secure emulation mode (sandboxed, no containers)
wrkflw run --runtime secure-emulation .github/workflows/ci.yml

# Run with verbose output
wrkflw run --verbose .github/workflows/ci.yml

# Preserve failed containers for debugging
wrkflw run --preserve-containers-on-failure .github/workflows/ci.yml

# Output execution progress as newline-delimited JSON
wrkflw run --json-output .github/workflows/ci.yml
```

### TUI Interface

```bash
# Open TUI (default: .github/workflows)
wrkflw tui

# Open with custom path
wrkflw tui path/to/workflows

# Open in specific runtime mode
wrkflw tui --runtime emulation
```

**TUI Keybinds:**

| Key | Action |
|-----|--------|
| `Tab` / `1-4` | Switch tabs |
| `↑↓` / `j/k` | Navigate |
| `Space` | Toggle selection |
| `Enter` | Run / View details |
| `r` | Run selected |
| `e` | Cycle runtime mode |
| `q` | Quit |

### Remote Triggering

Trigger workflows on GitHub/GitLab:

```bash
# GitHub workflow (requires GITHUB_TOKEN)
export GITHUB_TOKEN=ghp_your_token
wrkflw trigger workflow-name --branch main --input key=value

# GitLab pipeline (requires GITLAB_TOKEN)
export GITLAB_TOKEN=glpat_your_token
wrkflw trigger-gitlab --branch main --variable key=value
```

## Runtime Modes

| Mode | Isolation | Use Case |
|------|-----------|----------|
| **Docker** | Containers | Closest to CI environment; supports all action types |
| **Secure Emulation** | Sandboxed processes | Local development; safe for untrusted workflows |
| **Emulation** | None (⚠️ unsafe) | Legacy; not recommended |

### Docker Mode

```bash
wrkflw run --runtime docker .github/workflows/ci.yml
```

- Full GitHub Actions compatibility
- Supports Docker container actions
- Supports service containers

### Secure Emulation Mode

```bash
wrkflw run --runtime secure-emulation .github/workflows/ci.yml
```

- No container runtime required
- Command filtering blocks dangerous operations (`rm -rf /`, `sudo`, etc.)
- Resource limits (CPU, memory, execution time)
- Ideal for local development

### Emulation Mode (Legacy)

```bash
wrkflw run --runtime emulation .github/workflows/ci.yml
```

- No container runtime required
- No security protections—**use only with trusted workflows**
- Does not support Docker container actions

## Features

### Supported

- ✅ Workflow validation with proper exit codes
- ✅ Job dependency resolution (`needs` keyword)
- ✅ Parallel job execution
- ✅ Matrix builds
- ✅ Environment variables and GitHub context
- ✅ Docker container actions (Docker mode only)
- ✅ JavaScript actions
- ✅ Composite actions (including nested)
- ✅ Local actions
- ✅ Reusable workflows (caller jobs via `jobs.<id>.uses`)
- ✅ `actions/checkout` native handling
- ✅ Environment files (`GITHUB_OUTPUT`, `GITHUB_ENV`, `GITHUB_PATH`, `GITHUB_STEP_SUMMARY`)
- ✅ Remote workflow triggering
- ✅ GitLab CI/CD pipeline validation and triggering
- ✅ VSCode extension with real-time diagnostics

### Not Supported

- ❌ GitHub secrets (use environment variables instead)
- ❌ Actions cache (`actions/cache`)
- ❌ Artifact upload/download
- ❌ Windows/macOS runners (Linux only)
- ❌ Service containers in emulation mode
- ❌ Job/step timeouts enforcement
- ❌ Concurrency limits
- ❌ Event triggers other than `workflow_dispatch`

## System Requirements

- **Rust**: 1.67+ (for building from source)
- **Docker**: Optional but recommended (for Docker mode)

## Examples

### Validate a Workflow

```bash
$ wrkflw validate .github/workflows/ci.yml
Validating 1 workflow file(s)...
✅ Valid: .github/workflows/ci.yml

Summary: 1 valid, 0 invalid
```

### Run a Workflow

```bash
$ wrkflw run .github/workflows/ci.yml

Executing workflow: .github/workflows/ci.yml
============================================================
Runtime: Docker
------------------------------------------------------------

✅ Job succeeded: build

------------------------------------------------------------
  ✅ Checkout code
  ✅ Set up Rust
  ✅ Build
  ✅ Run tests

✅ Workflow completed successfully!
```

### Reusable Workflow

```yaml
jobs:
  call-shared:
    uses: ./.github/workflows/shared.yml
    with:
      config: production
    secrets:
      token: ${{ secrets.MY_TOKEN }}
```

## Environment Files

WRKFLW supports GitHub's special environment files:

```bash
# Step outputs
echo "result=value" >> "$GITHUB_OUTPUT"

# Environment variables
echo "VAR=value" >> "$GITHUB_ENV"

# PATH modification
echo "/path/to/bin" >> "$GITHUB_PATH"

# Step summary (Markdown)
echo "## Summary" >> "$GITHUB_STEP_SUMMARY"
```

## Debugging Failed Containers

Preserve failed containers for inspection:

```bash
wrkflw run --preserve-containers-on-failure .github/workflows/build.yml
```

When a job fails, WRKFLW keeps the container running:

```
Preserving container abc123 for debugging (exit code: 1).
Use 'docker exec -it abc123 bash' to inspect.
```

## VSCode Extension

WRKFLW includes a VSCode extension for real-time workflow validation.

### Features

* Real-time workflow diagnostics as you type
* Code completion for GitHub Actions syntax
* Hover information for workflow properties
* Custom language support for `.github/workflows/*.yml`

### Installation

The extension is available in `vscode-wrkflw/`. To build and install:

```bash
cd vscode-wrkflw
pnpm install
pnpm build
# Install in VSCode via Install from VSIX
```

### Commands

* `WRKFLW: Validate Current Workflow` - Validate the currently open workflow file

## WebAssembly

WRKFLW provides WebAssembly bindings for browser-based workflow validation.

### Available Functions

* `expandMatrix(matrix_json)` - Expand matrix configurations
* `formatCombinationName(job_name, combination_json)` - Format matrix job names
* `validateWorkflow(workflow_json)` - Validate workflow structure

### Usage Example

```javascript
import { expandMatrix, validateWorkflow } from 'wrkflw-wasm';

const matrix = {
  parameters: {
    os: ["ubuntu-latest", "windows-latest"],
    node: [14, 16, 18]
  }
};
const combinations = expandMatrix(JSON.stringify(matrix));
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT License](LICENSE)
