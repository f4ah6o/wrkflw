use serde_yaml::Value;
use wrkflw_models::ValidationResult;

pub fn validate_triggers(on: &Value, result: &mut ValidationResult) {
    let valid_events = vec![
        "branch_protection_rule",
        "check_run",
        "check_suite",
        "create",
        "delete",
        "deployment",
        "deployment_status",
        "discussion",
        "discussion_comment",
        "fork",
        "gollum",
        "issue_comment", // Covers comments on PRs that are not part of a diff
        "issues",
        "label",
        "merge_group",
        "milestone",
        "page_build",
        "public",
        "pull_request",
        "pull_request_review",
        "pull_request_review_comment",
        "pull_request_target",
        "push",
        "registry_package",
        "release",
        "repository_dispatch",
        "schedule",
        "status",
        "watch",
        "workflow_call",
        "workflow_dispatch",
        "workflow_run",
    ];

    match on {
        Value::String(event) => {
            if !valid_events.contains(&event.as_str()) {
                result.add_issue(format!("Unknown trigger event: '{}'", event));
            }
        }
        Value::Sequence(events) => {
            for event in events {
                if let Some(event_str) = event.as_str() {
                    if !valid_events.contains(&event_str) {
                        result.add_issue(format!("Unknown trigger event: '{}'", event_str));
                    }
                }
            }
        }
        Value::Mapping(event_map) => {
            for (event, _) in event_map {
                if let Some(event_str) = event.as_str() {
                    if !valid_events.contains(&event_str) {
                        result.add_issue(format!("Unknown trigger event: '{}'", event_str));
                    }
                }
            }

            // Check schedule syntax if present
            if let Some(Value::Sequence(schedules)) =
                event_map.get(Value::String("schedule".to_string()))
            {
                for schedule in schedules {
                    if let Some(schedule_map) = schedule.as_mapping() {
                        if let Some(Value::String(cron)) =
                            schedule_map.get(Value::String("cron".to_string()))
                        {
                            validate_cron_syntax(cron, result);
                        } else {
                            result.add_issue("Schedule is missing 'cron' expression".to_string());
                        }
                    }
                }
            }
        }
        _ => {
            result.add_issue("'on' section has invalid format".to_string());
        }
    }
}

pub(crate) fn validate_cron_syntax(cron: &str, result: &mut ValidationResult) {
    // Basic validation of cron syntax
    let parts: Vec<&str> = cron.split_whitespace().collect();
    if parts.len() != 5 {
        result.add_issue(format!(
            "Invalid cron syntax '{}': should have 5 components",
            cron
        ));
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    // Valid GitHub Actions trigger events
    const VALID_EVENTS: &[&str] = &[
        "branch_protection_rule",
        "check_run",
        "check_suite",
        "create",
        "delete",
        "deployment",
        "deployment_status",
        "discussion",
        "discussion_comment",
        "fork",
        "gollum",
        "issue_comment",
        "issues",
        "label",
        "merge_group",
        "milestone",
        "page_build",
        "public",
        "pull_request",
        "pull_request_review",
        "pull_request_review_comment",
        "pull_request_target",
        "push",
        "registry_package",
        "release",
        "repository_dispatch",
        "schedule",
        "status",
        "watch",
        "workflow_call",
        "workflow_dispatch",
        "workflow_run",
    ];

    /// Generate a valid trigger event name
    fn arb_valid_event() -> impl Strategy<Value = String> {
        proptest::sample::select(VALID_EVENTS).prop_map(|s| s.to_string())
    }

    /// Generate an invalid trigger event name
    fn arb_invalid_event() -> impl Strategy<Value = String> {
        "[a-z_]{5,20}"
            .prop_filter("Must not be valid event", |s| !VALID_EVENTS.contains(&s.as_str()))
    }

    /// Generate a valid 5-component cron expression
    fn arb_valid_cron() -> impl Strategy<Value = String> {
        (
            "(\\*|[0-5]?[0-9])",                    // minute
            "(\\*|[01]?[0-9]|2[0-3])",              // hour
            "(\\*|[1-9]|[12][0-9]|3[01])",          // day of month
            "(\\*|[1-9]|1[0-2])",                   // month
            "(\\*|[0-6])",                          // day of week
        )
            .prop_map(|(min, hour, dom, month, dow)| {
                format!("{} {} {} {} {}", min, hour, dom, month, dow)
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Valid events as string should not produce issues
        #[test]
        fn prop_valid_event_string_no_issues(event in arb_valid_event()) {
            let mut result = ValidationResult::new();
            let value = Value::String(event);
            validate_triggers(&value, &mut result);
            prop_assert!(result.is_valid, "Valid event should not produce issues");
        }

        /// Invalid events as string should produce issues
        #[test]
        fn prop_invalid_event_string_has_issues(event in arb_invalid_event()) {
            let mut result = ValidationResult::new();
            let value = Value::String(event.clone());
            validate_triggers(&value, &mut result);
            prop_assert!(
                !result.is_valid,
                "Invalid event '{}' should produce issues",
                event
            );
        }

        /// Valid events as sequence should not produce issues
        #[test]
        fn prop_valid_event_sequence_no_issues(events in proptest::collection::vec(arb_valid_event(), 1..=5)) {
            let mut result = ValidationResult::new();
            let value = Value::Sequence(events.into_iter().map(Value::String).collect());
            validate_triggers(&value, &mut result);
            prop_assert!(result.is_valid, "Valid events should not produce issues");
        }

        /// Cron with 5 components should be valid
        #[test]
        fn prop_valid_cron_no_issues(cron in arb_valid_cron()) {
            let mut result = ValidationResult::new();
            validate_cron_syntax(&cron, &mut result);
            prop_assert!(result.is_valid, "5-component cron should be valid: {}", cron);
        }

        /// Cron with wrong number of components should be invalid
        #[test]
        fn prop_invalid_cron_component_count(parts in proptest::collection::vec("[a-zA-Z0-9*/-]+", 1..=10).prop_filter("Not 5 parts", |v| v.len() != 5)) {
            let cron = parts.join(" ");
            let mut result = ValidationResult::new();
            validate_cron_syntax(&cron, &mut result);
            prop_assert!(
                !result.is_valid,
                "Cron with {} components should be invalid",
                parts.len()
            );
        }
    }

    #[test]
    fn test_empty_cron_invalid() {
        let mut result = ValidationResult::new();
        validate_cron_syntax("", &mut result);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_null_trigger_invalid() {
        let mut result = ValidationResult::new();
        validate_triggers(&Value::Null, &mut result);
        assert!(!result.is_valid);
    }
}
