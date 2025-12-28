# WRKFLW

[![Crates.io](https://img.shields.io/crates/v/wrkflw)](https://crates.io/crates/wrkflw)
[![Rust Version](https://img.shields.io/badge/rust-1.67%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/wrkflw)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/bahdotsh/wrkflw/build.yml?branch=main)](https://github.com/bahdotsh/wrkflw/actions/workflows/build.yml)
[![Downloads](https://img.shields.io/crates/d/wrkflw)](https://crates.io/crates/wrkflw)

WRKFLW is a powerful command-line tool for validating and executing GitHub Actions workflows locally, without requiring a full GitHub environment. It helps developers test their workflows directly on their machines before pushing changes to GitHub.

![WRKFLW Demo](demo.gif)

## Features

- **TUI Interface**: A full-featured terminal user interface for managing and monitoring workflow executions
- **Validate Workflow Files**: Check for syntax errors and common mistakes in GitHub Actions workflow files with proper exit codes for CI/CD integration
- **Execute Workflows Locally**: Run workflows directly on your machine using Docker containers
- **Multiple Container Runtimes**: Support for Docker and emulation mode for flexibility
- **Job Dependency Resolution**: Automatically determines the correct execution order based on job dependencies
- **Container Integration**: Execute workflow steps in isolated containers with proper environment setup
- **GitHub Context**: Provides GitHub-like environment variables and workflow commands
- **Action Support**: Supports various GitHub Actions types:
  - Docker container actions
  - JavaScript actions
  - Composite actions
  - Local actions
- **Special Action Handling**: Native handling for commonly used actions like `actions/checkout`
- **Reusable Workflows (Caller Jobs)**: Execute jobs that call reusable workflows via `jobs.<id>.uses` (local path or `owner/repo/path@ref`)
- **Output Capturing**: View logs, step outputs, and execution details
- **Parallel Job Execution**: Runs independent jobs in parallel for faster workflow execution
- **Trigger Workflows Remotely**: Manually trigger workflow runs on GitHub or GitLab

## Requirements

### Container Runtime (Optional)

WRKFLW supports multiple container runtimes for isolated execution:

- **Docker**: The default container runtime. Install from [docker.com](https://docker.com)
- **Emulation**: No container runtime required. Executes commands directly on the host system

## Installation

The recommended way to install `wrkflw` is using Rust's package manager, Cargo:

### Using Cargo Install (Recommended)
```bash
cargo install wrkflw
```

### From Source

Clone the repository and build using Cargo:

```bash
git clone https://github.com/bahdotsh/wrkflw.git
cd wrkflw
cargo build --release
```

The compiled binary will be available at `target/release/wrkflw`.

## Usage

The simplest way to use WRKFLW is to navigate to your project's root directory and run:

```bash
wrkflw
```

This will automatically detect and load all workflows from `.github/workflows` directory into the TUI interface.

WRKFLW also provides three main command modes:

### Validating Workflow Files

```bash
# Validate all workflow files in the default location (.github/workflows)
wrkflw validate

# Validate a specific workflow file
wrkflw validate path/to/workflow.yml

# Validate workflows in a specific directory
wrkflw validate path/to/workflows

# Validate multiple files and/or directories (GitHub and GitLab are auto-detected)
wrkflw validate path/to/flow-1.yml path/to/flow-2.yml path/to/workflows

# Force GitLab parsing for all provided paths
wrkflw validate --gitlab .gitlab-ci.yml other.gitlab-ci.yml

# Validate with verbose output
wrkflw validate --verbose path/to/workflow.yml

# Validate GitLab CI pipelines
wrkflw validate .gitlab-ci.yml --gitlab

# Disable exit codes for custom error handling (default: enabled)
wrkflw validate --no-exit-code path/to/workflow.yml
```

#### Exit Codes for CI/CD Integration

By default, `wrkflw validate` sets the exit code to `1` when validation fails, making it perfect for CI/CD pipelines and scripts:

```bash
# In CI/CD scripts - validation failure will cause the script to exit
if ! wrkflw validate; then
    echo "❌ Workflow validation failed!"
    exit 1
fi
echo "✅ All workflows are valid!"

# For custom error handling, disable exit codes
wrkflw validate --no-exit-code
if [ $? -eq 0 ]; then
    echo "Validation completed (check output for details)"
fi
```

**Exit Code Behavior:**
- `0`: All validations passed successfully
- `1`: One or more validation failures detected
- `2`: Command usage error (invalid arguments, file not found, etc.)

### Running Workflows in CLI Mode

```bash
# Run a workflow with Docker (default)
wrkflw run .github/workflows/ci.yml

# Run a workflow in emulation mode (without containers)
wrkflw run --runtime emulation .github/workflows/ci.yml

# Run with verbose output
wrkflw run --verbose .github/workflows/ci.yml

# Preserve failed containers for debugging
wrkflw run --preserve-containers-on-failure .github/workflows/ci.yml
```

### Using the TUI Interface

```bash
# Open TUI with workflows from the default directory
wrkflw tui

# Open TUI with a specific directory of workflows
wrkflw tui path/to/workflows

# Open TUI with a specific workflow pre-selected
wrkflw tui path/to/workflow.yml

# Open TUI in emulation mode
wrkflw tui --runtime emulation
```

### Triggering Workflows Remotely

```bash
# Trigger a workflow remotely on GitHub
wrkflw trigger workflow-name --branch main --input key1=value1 --input key2=value2

# Trigger a pipeline remotely on GitLab
wrkflw trigger-gitlab --branch main --variable key1=value1 --variable key2=value2
```

## TUI Controls

The terminal user interface provides an interactive way to manage workflows:

- **Tab / 1-4**: Switch between tabs (Workflows, Execution, Logs, Help)
- **Up/Down or j/k**: Navigate lists
- **Space**: Toggle workflow selection
- **Enter**: Run selected workflow / View job details
- **r**: Run all selected workflows
- **a**: Select all workflows
- **n**: Deselect all workflows
- **e**: Cycle through runtime modes (Docker → Secure Emulation → Emulation)
- **v**: Toggle between Execution and Validation mode
- **Esc**: Back / Exit detailed view
- **q**: Quit application

## Examples

### Validating a Workflow

```bash
$ wrkflw validate .github/workflows/rust.yml
Validating GitHub workflow file: .github/workflows/rust.yml... Validating 1 workflow file(s)...
✅ Valid: .github/workflows/rust.yml

Summary: 1 valid, 0 invalid

$ echo $?
0

# Example with validation failure
$ wrkflw validate .github/workflows/invalid.yml
Validating GitHub workflow file: .github/workflows/invalid.yml... Validating 1 workflow file(s)...
❌ Invalid: .github/workflows/invalid.yml
   1. Job 'test' is missing 'runs-on' field
   2. Job 'test' is missing 'steps' section

Summary: 0 valid, 1 invalid

$ echo $?
1
```

### Running a Workflow

```bash
$ wrkflw run .github/workflows/rust.yml

Executing workflow: .github/workflows/rust.yml
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

### Quick TUI Startup

```bash
# Navigate to project root and run wrkflw
$ cd my-project
$ wrkflw

# This will automatically load .github/workflows files into the TUI
```

## System Requirements

- Rust 1.67 or later
- Container Runtime (optional, for container-based execution):
  - **Docker**: Traditional container runtime
  - **None**: Emulation mode runs workflows using local system tools

## How It Works

WRKFLW parses your GitHub Actions workflow files and executes each job and step in the correct order. For container mode (Docker), it creates containers that closely match GitHub's runner environments. The workflow execution process:

1. **Parsing**: Reads and validates the workflow YAML structure
2. **Dependency Resolution**: Creates an execution plan based on job dependencies
3. **Environment Setup**: Prepares GitHub-like environment variables and context
4. **Execution**: Runs each job and step either in containers (Docker) or through local emulation
5. **Monitoring**: Tracks progress and captures outputs in the TUI or command line

## Advanced Features

### GitHub Environment Files Support

WRKFLW supports GitHub's environment files and special commands:

- `GITHUB_OUTPUT`: For storing step outputs (`echo "result=value" >> $GITHUB_OUTPUT`)
- `GITHUB_ENV`: For setting environment variables (`echo "VAR=value" >> $GITHUB_ENV`)
- `GITHUB_PATH`: For modifying the PATH (`echo "/path/to/dir" >> $GITHUB_PATH`)
- `GITHUB_STEP_SUMMARY`: For creating step summaries (`echo "# Summary" >> $GITHUB_STEP_SUMMARY`)

### Composite Actions

WRKFLW supports composite actions, which are actions made up of multiple steps. This includes:

- Local composite actions referenced with `./path/to/action`
- Remote composite actions from GitHub repositories
- Nested composite actions (composite actions that use other actions)

### Container Cleanup

WRKFLW automatically cleans up any containers created during workflow execution (Docker), even if the process is interrupted with Ctrl+C.

For debugging failed workflows, you can preserve containers that fail by using the `--preserve-containers-on-failure` flag:

```bash
# Preserve failed containers for debugging
wrkflw run --preserve-containers-on-failure .github/workflows/build.yml

# Also available in TUI mode
wrkflw tui --preserve-containers-on-failure
```

When a container fails with this flag enabled, WRKFLW will:
- Keep the failed container running instead of removing it
- Log the container ID and provide inspection instructions
- Show a message like: `Preserving container abc123 for debugging (exit code: 1). Use 'docker exec -it abc123 bash' to inspect.`

This allows you to inspect the exact state of the container when the failure occurred, examine files, check environment variables, and debug issues more effectively.

## Limitations

### Supported Features
- ✅ Basic workflow syntax and validation (all YAML syntax checks, required fields, and structure) with proper exit codes for CI/CD integration
- ✅ Job dependency resolution and parallel execution (all jobs with correct 'needs' relationships are executed in the right order, and independent jobs run in parallel)
- ✅ Matrix builds (supported for reasonable matrix sizes; very large matrices may be slow or resource-intensive)
- ✅ Environment variables and GitHub context (all standard GitHub Actions environment variables and context objects are emulated)
- ✅ Container actions (all actions that use containers are supported in Docker mode)
- ✅ JavaScript actions (all actions that use JavaScript are supported)
- ✅ Composite actions (all composite actions, including nested and local composite actions, are supported)
- ✅ Local actions (actions referenced with local paths are supported)
- ✅ Special handling for common actions (e.g., `actions/checkout` is natively supported)
- ✅ Reusable workflows (caller): Jobs that use `jobs.<id>.uses` to call local or remote workflows are executed; inputs and secrets are propagated to the called workflow
- ✅ Workflow triggering via `workflow_dispatch` (manual triggering of workflows is supported)
- ✅ GitLab pipeline triggering (manual triggering of GitLab pipelines is supported)
- ✅ Environment files (`GITHUB_OUTPUT`, `GITHUB_ENV`, `GITHUB_PATH`, `GITHUB_STEP_SUMMARY` are fully supported)
- ✅ TUI interface for workflow management and monitoring
- ✅ CLI interface for validation, execution, and remote triggering
- ✅ Output capturing (logs, step outputs, and execution details are available in both TUI and CLI)
- ✅ Container cleanup (all containers created by wrkflw are automatically cleaned up, even on interruption)

### Limited or Unsupported Features (Explicit List)
- ❌ GitHub secrets and permissions: Only basic environment variables are supported. GitHub's encrypted secrets and fine-grained permissions are NOT available.
- ❌ GitHub Actions cache: Caching functionality (e.g., `actions/cache`) is NOT supported in emulation mode and only partially supported in Docker mode (no persistent cache between runs).
- ❌ GitHub API integrations: Only basic workflow triggering is supported. Features like workflow status reporting, artifact upload/download, and API-based job control are NOT available.
- ❌ GitHub-specific environment variables: Some advanced or dynamic environment variables (e.g., those set by GitHub runners or by the GitHub API) are emulated with static or best-effort values, but not all are fully functional.
- ❌ Large/complex matrix builds: Very large matrices (hundreds or thousands of job combinations) may not be practical due to performance and resource limits.
- ❌ Network-isolated actions: Actions that require strict network isolation or custom network configuration may not work out-of-the-box and may require manual container runtime configuration.
- ❌ Some event triggers: Only `workflow_dispatch` (manual trigger) is fully supported. Other triggers (e.g., `push`, `pull_request`, `schedule`, `release`, etc.) are NOT supported.
- ❌ GitHub runner-specific features: Features that depend on the exact GitHub-hosted runner environment (e.g., pre-installed tools, runner labels, or hardware) are NOT guaranteed to match. Only a best-effort emulation is provided.
- ❌ Windows and macOS runners: Only Linux-based runners are fully supported. Windows and macOS jobs are NOT supported.
- ❌ Service containers: Service containers (e.g., databases defined in `services:`) are only supported in Docker mode. In emulation mode, they are NOT supported.
- ❌ Artifacts: Uploading and downloading artifacts between jobs/steps is NOT supported.
- ❌ Job/step timeouts: Custom timeouts for jobs and steps are NOT enforced.
- ❌ Job/step concurrency and cancellation: Features like `concurrency` and job cancellation are NOT supported.
- ❌ Expressions and advanced YAML features: Most common expressions are supported, but some advanced or edge-case expressions may not be fully implemented.
- ⚠️ Reusable workflows (limits):
  - Outputs from called workflows are not propagated back to the caller (`needs.<id>.outputs.*` not supported)
  - `secrets: inherit` is not special-cased; provide a mapping to pass secrets
  - Remote calls clone public repos via HTTPS; private repos require preconfigured access (not yet implemented)
  - Deeply nested reusable calls work but lack cycle detection beyond regular job dependency checks

## Reusable Workflows

WRKFLW supports executing reusable workflow caller jobs.

### Syntax

```yaml
jobs:
  call-local:
    uses: ./.github/workflows/shared.yml

  call-remote:
    uses: my-org/my-repo/.github/workflows/shared.yml@v1
    with:
      foo: bar
    secrets:
      token: ${{ secrets.MY_TOKEN }}
```

### Behavior
- Local references are resolved relative to the current working directory.
- Remote references are shallow-cloned at the specified `@ref` into a temporary directory.
- `with:` entries are exposed to the called workflow as environment variables `INPUT_<KEY>`.
- `secrets:` mapping entries are exposed as environment variables `SECRET_<KEY>`.
- The called workflow executes according to its own `jobs`/`needs`; a summary of its job results is reported as a single result for the caller job.

### Current limitations
- Outputs from called workflows are not surfaced back to the caller.
- `secrets: inherit` is not supported; specify an explicit mapping.
- Private repositories for remote `uses:` are not yet supported.

### Runtime Mode Differences
- **Docker Mode**: Provides the closest match to GitHub's environment, including support for Docker container actions, service containers, and Linux-based jobs. Some advanced container configurations may still require manual setup.
- **🔒 Secure Emulation Mode**: Runs workflows on the local system with comprehensive sandboxing for security. **Recommended for local development**:
  - Command validation and filtering (blocks dangerous commands like `rm -rf /`, `sudo`, etc.)
  - Resource limits (CPU, memory, execution time)
  - Filesystem access controls
  - Process monitoring and limits
  - Safe for running untrusted workflows locally
- **⚠️ Emulation Mode (Legacy)**: Runs workflows using local system tools without sandboxing. **Not recommended - use Secure Emulation instead**:
  - Only supports local and JavaScript actions (no Docker container actions)
  - No support for service containers
  - No caching support
  - **No security protections - can execute harmful commands**
  - Some actions may require adaptation to work locally

### Best Practices
- **Use Secure Emulation mode for local development** - provides safety without container overhead
- Test workflows in multiple runtime modes to ensure compatibility
- **Use Docker mode for production** - provides maximum isolation and reproducibility
- Keep matrix builds reasonably sized for better performance
- Use environment variables instead of GitHub secrets when possible
- Consider using local actions for complex custom functionality
- **Review security warnings** - pay attention to blocked commands in secure emulation mode
- **Start with secure mode** - only fall back to legacy emulation if necessary

## Roadmap

The following roadmap outlines our planned approach to implementing currently unsupported or partially supported features in WRKFLW. Progress and priorities may change based on user feedback and community contributions.

### 1. Secrets and Permissions
- **Goal:** Support encrypted secrets and fine-grained permissions similar to GitHub Actions.
- **Plan:** 
  - Implement secure secret storage and injection for workflow steps.
  - Add support for reading secrets from environment variables, files, or secret managers.
  - Investigate permission scoping for jobs and steps.

### 2. GitHub Actions Cache
- **Goal:** Enable persistent caching between workflow runs, especially for dependencies.
- **Plan:** 
  - Implement a local cache directory for Docker mode.
  - Add support for `actions/cache` in both Docker and emulation modes.
  - Investigate cross-run cache persistence.

### 3. GitHub API Integrations
- **Goal:** Support artifact upload/download, workflow/job status reporting, and other API-based features.
- **Plan:** 
  - Add artifact upload/download endpoints.
  - Implement status reporting to GitHub via the API.
  - Add support for job/step annotations and logs upload.

### 4. Advanced Environment Variables
- **Goal:** Emulate all dynamic GitHub-provided environment variables.
- **Plan:** 
  - Audit missing variables and add dynamic computation where possible.
  - Provide a compatibility table in the documentation.

### 5. Large/Complex Matrix Builds
- **Goal:** Improve performance and resource management for large matrices.
- **Plan:** 
  - Optimize matrix expansion and job scheduling.
  - Add resource limits and warnings for very large matrices.

### 6. Network-Isolated Actions
- **Goal:** Support custom network configurations and strict isolation for actions.
- **Plan:**
  - Add advanced container network configuration options for Docker.
  - Document best practices for network isolation.

### 7. Event Triggers
- **Goal:** Support additional triggers (`push`, `pull_request`, `schedule`, etc.).
- **Plan:** 
  - Implement event simulation for common triggers.
  - Allow users to specify event payloads for local runs.

### 8. Windows and macOS Runners
- **Goal:** Add support for non-Linux runners.
- **Plan:** 
  - Investigate cross-platform containerization and emulation.
  - Add documentation for platform-specific limitations.

### 9. Service Containers in Emulation Mode
- **Goal:** Support service containers (e.g., databases) in emulation mode.
- **Plan:** 
  - Implement local service startup and teardown scripts.
  - Provide configuration for common services.

### 10. Artifacts, Timeouts, Concurrency, and Expressions
- **Goal:** Support artifact handling, job/step timeouts, concurrency, and advanced YAML expressions.
- **Plan:** 
  - Add artifact storage and retrieval.
  - Enforce timeouts and concurrency limits.
  - Expand expression parser for advanced use cases.

---

**Want to help?** Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Remote Workflow Triggering

WRKFLW allows you to manually trigger workflow runs on GitHub through both the command-line interface (CLI) and the terminal user interface (TUI).

### Requirements:

1. You need a GitHub token with workflow permissions. Set it in the `GITHUB_TOKEN` environment variable:
   ```bash
   export GITHUB_TOKEN=ghp_your_token_here
   ```

2. The workflow must have the `workflow_dispatch` trigger defined in your workflow YAML:
   ```yaml
   on:
     workflow_dispatch:
       inputs:
         name:
           description: 'Person to greet'
           default: 'World'
           required: true
         debug:
           description: 'Enable debug mode'
           required: false
           type: boolean
           default: false
   ```

### Triggering from CLI:

```bash
# Trigger a workflow using the default branch
wrkflw trigger workflow-name

# Trigger a workflow on a specific branch
wrkflw trigger workflow-name --branch feature-branch

# Trigger with input parameters
wrkflw trigger workflow-name --branch main --input name=Alice --input debug=true
```

After triggering, WRKFLW will provide feedback including the URL to view the triggered workflow on GitHub.

### Triggering from TUI:

1. Launch the TUI interface:
   ```bash
   wrkflw tui
   ```

2. Navigate to the "Workflows" tab (use `Tab` key or press `1`).

3. Use the arrow keys (`↑`/`↓`) or `j`/`k` to select the desired workflow.

4. Press `t` to trigger the selected workflow.

5. If the workflow is successfully triggered, you'll see a notification in the UI.

6. You can monitor the triggered workflow's execution on GitHub using the provided URL.

### Verifying Triggered Workflows:

To verify that your workflow was triggered:

1. Visit your GitHub repository in a web browser.
2. Navigate to the "Actions" tab.
3. Look for your workflow in the list of workflow runs.
4. Click on it to view the details of the run.
