// utils crate

use std::path::Path;

pub fn is_workflow_file(path: &Path) -> bool {
    // First, check for GitLab CI files by name
    if let Some(file_name) = path.file_name() {
        let file_name_str = file_name.to_string_lossy().to_lowercase();
        if file_name_str == ".gitlab-ci.yml" || file_name_str.ends_with("gitlab-ci.yml") {
            return true;
        }
    }

    // Then check for GitHub Actions workflows
    if let Some(ext) = path.extension() {
        if ext == "yml" || ext == "yaml" {
            // Check if the file is in a .github/workflows directory
            if let Some(parent) = path.parent() {
                return parent.ends_with(".github/workflows") || parent.ends_with("workflows");
            } else {
                // Check if filename contains workflow indicators
                let filename = path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_lowercase())
                    .unwrap_or_default();

                return filename.contains("workflow")
                    || filename.contains("action")
                    || filename.contains("ci")
                    || filename.contains("cd");
            }
        }
    }
    false
}

/// Module for safely handling file descriptor redirection
///
/// On Unix systems (Linux, macOS), this module provides true file descriptor
/// redirection by duplicating stderr and redirecting it to /dev/null.
///
/// On Windows systems, the redirection functionality is limited due to platform
/// differences in file descriptor handling. The functions will execute without
/// error but stderr may not be fully suppressed.
pub mod fd {
    use std::io::Result;

    /// Represents a redirected stderr that can be restored
    pub struct RedirectedStderr {
        #[cfg(unix)]
        original_fd: Option<std::os::unix::io::RawFd>,
        #[cfg(unix)]
        null_fd: Option<std::os::unix::io::RawFd>,
        #[cfg(windows)]
        _phantom: std::marker::PhantomData<()>,
    }

    #[cfg(unix)]
    mod unix_impl {
        use super::*;
        use nix::fcntl::{open, OFlag};
        use nix::sys::stat::Mode;
        use nix::unistd::{close, dup, dup2};
        use std::io;
        use std::os::unix::io::RawFd;
        use std::path::Path;

        /// Standard file descriptors
        const STDERR_FILENO: RawFd = 2;

        impl RedirectedStderr {
            /// Creates a new RedirectedStderr that redirects stderr to /dev/null
            pub fn to_null() -> Result<Self> {
                // Duplicate the current stderr fd
                let stderr_backup = match dup(STDERR_FILENO) {
                    Ok(fd) => fd,
                    Err(e) => return Err(io::Error::other(e)),
                };

                // Open /dev/null
                let null_fd = match open(Path::new("/dev/null"), OFlag::O_WRONLY, Mode::empty()) {
                    Ok(fd) => fd,
                    Err(e) => {
                        let _ = close(stderr_backup); // Clean up on error
                        return Err(io::Error::other(e));
                    }
                };

                // Redirect stderr to /dev/null
                if let Err(e) = dup2(null_fd, STDERR_FILENO) {
                    let _ = close(stderr_backup); // Clean up on error
                    let _ = close(null_fd);
                    return Err(io::Error::other(e));
                }

                Ok(RedirectedStderr {
                    original_fd: Some(stderr_backup),
                    null_fd: Some(null_fd),
                })
            }
        }

        impl Drop for RedirectedStderr {
            /// Automatically restores stderr when the RedirectedStderr is dropped
            fn drop(&mut self) {
                if let Some(orig_fd) = self.original_fd.take() {
                    // Restore the original stderr
                    let _ = dup2(orig_fd, STDERR_FILENO);
                    let _ = close(orig_fd);
                }

                // Close the null fd
                if let Some(null_fd) = self.null_fd.take() {
                    let _ = close(null_fd);
                }
            }
        }
    }

    #[cfg(windows)]
    mod windows_impl {
        use super::*;

        impl RedirectedStderr {
            /// Creates a new RedirectedStderr that redirects stderr to NUL on Windows
            pub fn to_null() -> Result<Self> {
                // On Windows, we can't easily redirect stderr at the file descriptor level
                // like we can on Unix systems. This is a simplified implementation that
                // doesn't actually redirect but provides the same interface.
                // The actual stderr suppression will need to be handled differently on Windows.
                Ok(RedirectedStderr {
                    _phantom: std::marker::PhantomData,
                })
            }
        }

        impl Drop for RedirectedStderr {
            /// No-op drop implementation for Windows
            fn drop(&mut self) {
                // Nothing to restore on Windows in this simplified implementation
            }
        }
    }

    /// Run a function with stderr redirected to /dev/null (Unix) or suppressed (Windows), then restore stderr
    ///
    /// # Platform Support
    /// - **Unix (Linux, macOS)**: Fully supported - stderr is redirected to /dev/null
    /// - **Windows**: Limited support - function executes but stderr may be visible
    ///
    /// # Example
    /// ```
    /// use wrkflw_utils::fd::with_stderr_to_null;
    ///
    /// let result = with_stderr_to_null(|| {
    ///     eprintln!("This will be hidden on Unix");
    ///     42
    /// }).unwrap();
    /// assert_eq!(result, 42);
    /// ```
    pub fn with_stderr_to_null<F, T>(f: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        #[cfg(unix)]
        {
            let _redirected = RedirectedStderr::to_null()?;
            Ok(f())
        }
        #[cfg(windows)]
        {
            // On Windows, we can't easily redirect stderr at the FD level,
            // so we just run the function without redirection.
            // This means stderr won't be suppressed on Windows, but the function will work.
            Ok(f())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    mod proptest_tests {
        use super::*;
        use proptest::prelude::*;

        /// Generate a valid GitHub workflow path
        fn arb_github_workflow_path() -> impl Strategy<Value = PathBuf> {
            "[a-zA-Z][a-zA-Z0-9_-]{1,20}\\.(yml|yaml)"
                .prop_map(|name| PathBuf::from(format!(".github/workflows/{}", name)))
        }

        /// Generate a GitLab CI path
        fn arb_gitlab_ci_path() -> impl Strategy<Value = PathBuf> {
            Just(PathBuf::from(".gitlab-ci.yml"))
        }

        /// Generate a non-workflow YAML file
        fn arb_non_workflow_yaml() -> impl Strategy<Value = PathBuf> {
            "[a-zA-Z][a-zA-Z0-9_-]{1,20}\\.(yml|yaml)"
                .prop_map(|name| PathBuf::from(format!("config/{}", name)))
        }

        /// Generate a non-YAML file
        fn arb_non_yaml_file() -> impl Strategy<Value = PathBuf> {
            "[a-zA-Z][a-zA-Z0-9_-]{1,20}\\.(txt|json|toml|rs)"
                .prop_map(PathBuf::from)
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(256))]

            /// GitHub workflow paths should be detected
            #[test]
            fn prop_github_workflow_detected(path in arb_github_workflow_path()) {
                prop_assert!(
                    is_workflow_file(&path),
                    "GitHub workflow path should be detected: {:?}",
                    path
                );
            }

            /// GitLab CI paths should be detected
            #[test]
            fn prop_gitlab_ci_detected(path in arb_gitlab_ci_path()) {
                prop_assert!(
                    is_workflow_file(&path),
                    "GitLab CI path should be detected: {:?}",
                    path
                );
            }

            /// Non-workflow YAML files should not be detected
            #[test]
            fn prop_non_workflow_yaml_not_detected(path in arb_non_workflow_yaml()) {
                prop_assert!(
                    !is_workflow_file(&path),
                    "Non-workflow YAML should not be detected: {:?}",
                    path
                );
            }

            /// Non-YAML files should not be detected
            #[test]
            fn prop_non_yaml_not_detected(path in arb_non_yaml_file()) {
                prop_assert!(
                    !is_workflow_file(&path),
                    "Non-YAML file should not be detected: {:?}",
                    path
                );
            }

            /// with_stderr_to_null should return the correct value
            #[test]
            fn prop_with_stderr_returns_value(value in any::<i32>()) {
                let result = fd::with_stderr_to_null(|| value);
                prop_assert!(result.is_ok());
                prop_assert_eq!(result.unwrap(), value);
            }
        }
    }

    #[test]
    fn test_fd_redirection() {
        // This test will write to stderr, which should be redirected on Unix
        // On Windows, it will just run normally without redirection
        let result = fd::with_stderr_to_null(|| {
            // This would normally appear in stderr (suppressed on Unix, visible on Windows)
            eprintln!("This should be redirected to /dev/null on Unix");
            // Return a test value to verify the function passes through the result
            42
        });

        // The function should succeed and return our test value on both platforms
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_is_workflow_file() {
        // GitHub workflow files
        assert!(is_workflow_file(Path::new(".github/workflows/ci.yml")));
        assert!(is_workflow_file(Path::new(".github/workflows/test.yaml")));

        // GitLab CI
        assert!(is_workflow_file(Path::new(".gitlab-ci.yml")));

        // Non-workflow files
        assert!(!is_workflow_file(Path::new("config.yml")));
        assert!(!is_workflow_file(Path::new("src/main.rs")));
        assert!(!is_workflow_file(Path::new("package.json")));
    }
}
