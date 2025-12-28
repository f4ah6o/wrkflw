use wrkflw_models::ValidationResult;

pub fn validate_action_reference(
    action_ref: &str,
    job_name: &str,
    step_idx: usize,
    result: &mut ValidationResult,
) {
    // Check if it's a local action (starts with ./)
    let is_local_action = action_ref.starts_with("./");

    // For non-local actions, enforce standard format
    if !is_local_action && !action_ref.contains('/') && !action_ref.contains('.') {
        result.add_issue(format!(
            "Job '{}', step {}: Invalid action reference format '{}'",
            job_name,
            step_idx + 1,
            action_ref
        ));
        return;
    }

    // Check for version tag or commit SHA, but only for non-local actions
    if !is_local_action && action_ref.contains('@') {
        let parts: Vec<&str> = action_ref.split('@').collect();
        if parts.len() != 2 || parts[1].is_empty() {
            result.add_issue(format!(
                "Job '{}', step {}: Action '{}' has invalid version/ref format",
                job_name,
                step_idx + 1,
                action_ref
            ));
        }
    } else if !is_local_action {
        // Missing version tag is not recommended for non-local actions
        result.add_issue(format!(
            "Job '{}', step {}: Action '{}' is missing version tag (@v2, @main, etc.)",
            job_name,
            step_idx + 1,
            action_ref
        ));
    }

    // For local actions, verify the path exists
    if is_local_action {
        let action_path = std::path::Path::new(action_ref);
        if !action_path.exists() {
            // We can't reliably check this during validation since the working directory
            // might not be the repository root, but we'll add a warning
            result.add_issue(format!(
                "Job '{}', step {}: Local action path '{}' may not exist at runtime",
                job_name,
                step_idx + 1,
                action_ref
            ));
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    /// Generate a valid action reference with version tag
    fn arb_valid_action_ref() -> impl Strategy<Value = String> {
        (
            "[a-zA-Z][a-zA-Z0-9_-]{2,20}",  // owner
            "[a-zA-Z][a-zA-Z0-9_-]{2,20}",  // repo
            "(v[0-9]+|main|master|[a-f0-9]{40})",  // version/ref
        )
            .prop_map(|(owner, repo, version)| format!("{}/{}@{}", owner, repo, version))
    }

    /// Generate an action reference without version tag
    fn arb_action_without_version() -> impl Strategy<Value = String> {
        (
            "[a-zA-Z][a-zA-Z0-9_-]{2,20}",  // owner
            "[a-zA-Z][a-zA-Z0-9_-]{2,20}",  // repo
        )
            .prop_map(|(owner, repo)| format!("{}/{}", owner, repo))
    }

    /// Generate a local action reference
    fn arb_local_action_ref() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_/-]{1,30}".prop_map(|path| format!("./{}", path))
    }

    /// Generate an invalid action reference (no slash, no dot)
    fn arb_invalid_action_ref() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_-]{5,20}".prop_filter("Must not contain slash or dot", |s| {
            !s.contains('/') && !s.contains('.')
        })
    }

    /// Generate a job name
    fn arb_job_name() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_-]{2,15}"
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Valid action references with version should be accepted (only warning-free if path exists for local)
        #[test]
        fn prop_valid_action_ref_accepted(
            action_ref in arb_valid_action_ref(),
            job_name in arb_job_name(),
            step_idx in 0usize..10
        ) {
            let mut result = ValidationResult::new();
            validate_action_reference(&action_ref, &job_name, step_idx, &mut result);
            // Valid action refs should not have format errors
            let has_format_error = result.issues.iter().any(|i| i.contains("Invalid action reference format"));
            prop_assert!(!has_format_error, "Valid action ref should not have format errors");
        }

        /// Action references without version should produce a warning
        #[test]
        fn prop_action_without_version_has_warning(
            action_ref in arb_action_without_version(),
            job_name in arb_job_name(),
            step_idx in 0usize..10
        ) {
            let mut result = ValidationResult::new();
            validate_action_reference(&action_ref, &job_name, step_idx, &mut result);
            let has_version_warning = result.issues.iter().any(|i| i.contains("missing version tag"));
            prop_assert!(has_version_warning, "Action without version should have warning");
        }

        /// Local action references should start with ./
        #[test]
        fn prop_local_action_starts_with_dot_slash(
            action_ref in arb_local_action_ref(),
            job_name in arb_job_name(),
            step_idx in 0usize..10
        ) {
            let mut result = ValidationResult::new();
            validate_action_reference(&action_ref, &job_name, step_idx, &mut result);
            // Local actions shouldn't have format errors (but may have path warnings)
            let has_format_error = result.issues.iter().any(|i| i.contains("Invalid action reference format"));
            prop_assert!(!has_format_error, "Local action should not have format errors");
        }

        /// Invalid action references (no slash, no dot) should produce errors
        #[test]
        fn prop_invalid_action_ref_has_error(
            action_ref in arb_invalid_action_ref(),
            job_name in arb_job_name(),
            step_idx in 0usize..10
        ) {
            let mut result = ValidationResult::new();
            validate_action_reference(&action_ref, &job_name, step_idx, &mut result);
            prop_assert!(
                !result.is_valid,
                "Invalid action ref '{}' should produce error",
                action_ref
            );
        }

        /// Action with empty version after @ should be invalid
        #[test]
        fn prop_empty_version_invalid(
            owner in "[a-zA-Z][a-zA-Z0-9_-]{2,10}",
            repo in "[a-zA-Z][a-zA-Z0-9_-]{2,10}",
            job_name in arb_job_name(),
            step_idx in 0usize..10
        ) {
            let action_ref = format!("{}/{}@", owner, repo);
            let mut result = ValidationResult::new();
            validate_action_reference(&action_ref, &job_name, step_idx, &mut result);
            let has_version_error = result.issues.iter().any(|i| i.contains("invalid version/ref format"));
            prop_assert!(has_version_error, "Empty version should produce error");
        }
    }

    #[test]
    fn test_well_known_actions() {
        // Test common actions
        let actions = [
            ("actions/checkout@v4", true),
            ("actions/setup-node@v3", true),
            ("docker/build-push-action@v5", true),
        ];

        for (action, should_be_valid) in actions {
            let mut result = ValidationResult::new();
            validate_action_reference(action, "test-job", 0, &mut result);
            let has_format_error = result.issues.iter().any(|i| i.contains("Invalid"));
            assert_eq!(
                !has_format_error,
                should_be_valid,
                "Action {} validation failed",
                action
            );
        }
    }
}
