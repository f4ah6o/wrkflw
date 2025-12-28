use serde_yaml::Value;
use wrkflw_models::ValidationResult;

pub fn validate_matrix(matrix: &Value, result: &mut ValidationResult) {
    // Check if matrix is a mapping
    if !matrix.is_mapping() {
        result.add_issue("Matrix must be a mapping".to_string());
        return;
    }

    // Check for include and exclude sections
    if let Some(include) = matrix.get("include") {
        validate_include_exclude(include, "include", result);
    }

    if let Some(exclude) = matrix.get("exclude") {
        validate_include_exclude(exclude, "exclude", result);
    }

    // Check max-parallel
    if let Some(max_parallel) = matrix.get("max-parallel") {
        if !max_parallel.is_number() {
            result.add_issue("max-parallel must be a number".to_string());
        } else if let Some(value) = max_parallel.as_u64() {
            if value == 0 {
                result.add_issue("max-parallel must be greater than 0".to_string());
            }
        }
    }

    // Check fail-fast
    if let Some(fail_fast) = matrix.get("fail-fast") {
        if !fail_fast.is_bool() {
            result.add_issue("fail-fast must be a boolean".to_string());
        }
    }

    // Validate the main matrix parameters (excluding special keywords)
    let special_keys = ["include", "exclude", "max-parallel", "fail-fast"];

    // Use if let to avoid unwrap
    if let Some(mapping) = matrix.as_mapping() {
        for (key, value) in mapping {
            // Safely get the key string, using an empty string as fallback
            let key_str = key.as_str().unwrap_or("");
            if !special_keys.contains(&key_str) {
                validate_matrix_parameter(key_str, value, result);
            }
        }
    } else {
        // This is a safeguard, though we already checked if it's a mapping above
        result.add_issue("Failed to process matrix mapping".to_string());
    }
}

fn validate_include_exclude(section: &Value, section_name: &str, result: &mut ValidationResult) {
    if !section.is_sequence() {
        result.add_issue(format!("{} must be an array of objects", section_name));
        return;
    }

    // Check each item in the include/exclude array
    // Use if let to avoid unwrap
    if let Some(sequence) = section.as_sequence() {
        for (index, item) in sequence.iter().enumerate() {
            if !item.is_mapping() {
                result.add_issue(format!(
                    "{} item at index {} must be an object",
                    section_name, index
                ));
            }
        }
    } else {
        // This is a safeguard, though we already checked if it's a sequence above
        result.add_issue(format!("Failed to process {} sequence", section_name));
    }
}

fn validate_matrix_parameter(name: &str, value: &Value, result: &mut ValidationResult) {
    // Basic matrix parameters should be arrays or simple values
    match value {
        Value::Sequence(_) => {
            // Check that each item in the array has a consistent type
            if let Some(seq) = value.as_sequence() {
                if !seq.is_empty() {
                    let first_type = get_value_type(&seq[0]);

                    for (i, item) in seq.iter().enumerate().skip(1) {
                        let item_type = get_value_type(item);
                        if item_type != first_type {
                            result.add_issue(format!(
                                "Matrix parameter '{}' has inconsistent types: item at index {} is {}, but expected {}",
                                name, i, item_type, first_type
                            ));
                        }
                    }
                }
            }
        }
        Value::Mapping(_) => {
            // For object-based parameters, make sure they have valid structure
            // Here we just check if it's a mapping, but could add more validation
        }
        // Other types (string, number, bool) are valid as single values
        _ => (),
    }
}

pub(crate) fn get_value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Sequence(_) => "array",
        Value::Mapping(_) => "object",
        _ => "unknown",
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;
    use serde_yaml::Number;

    /// Generate a simple YAML value
    fn arb_simple_value() -> impl Strategy<Value = Value> {
        prop_oneof![
            "[a-zA-Z][a-zA-Z0-9_]{0,10}".prop_map(Value::String),
            (0i64..100).prop_map(|n| Value::Number(Number::from(n))),
            any::<bool>().prop_map(Value::Bool),
        ]
    }

    /// Generate a valid matrix parameter (array of same-type values)
    fn arb_consistent_array() -> impl Strategy<Value = Value> {
        prop_oneof![
            proptest::collection::vec("[a-zA-Z][a-zA-Z0-9]{0,8}", 1..=4)
                .prop_map(|v| Value::Sequence(v.into_iter().map(Value::String).collect())),
            proptest::collection::vec(0i64..100, 1..=4)
                .prop_map(|v| Value::Sequence(v.into_iter().map(|n| Value::Number(Number::from(n))).collect())),
            proptest::collection::vec(any::<bool>(), 1..=4)
                .prop_map(|v| Value::Sequence(v.into_iter().map(Value::Bool).collect())),
        ]
    }

    /// Generate a matrix with consistent types
    fn arb_valid_matrix() -> impl Strategy<Value = Value> {
        proptest::collection::vec(
            ("[a-zA-Z][a-zA-Z0-9_]{1,10}", arb_consistent_array()),
            1..=3,
        )
        .prop_map(|pairs| {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in pairs {
                map.insert(Value::String(k), v);
            }
            Value::Mapping(map)
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// get_value_type should return correct type for all values
        #[test]
        fn prop_get_value_type_correct(value in arb_simple_value()) {
            let type_str = get_value_type(&value);
            match &value {
                Value::String(_) => prop_assert_eq!(type_str, "string"),
                Value::Number(_) => prop_assert_eq!(type_str, "number"),
                Value::Bool(_) => prop_assert_eq!(type_str, "boolean"),
                _ => {}
            }
        }

        /// Valid matrix with consistent types should not produce type errors
        #[test]
        fn prop_valid_matrix_no_type_errors(matrix in arb_valid_matrix()) {
            let mut result = ValidationResult::new();
            validate_matrix(&matrix, &mut result);
            let has_type_error = result.issues.iter().any(|i| i.contains("inconsistent types"));
            prop_assert!(!has_type_error, "Valid matrix should not have type errors");
        }

        /// Matrix must be a mapping
        #[test]
        fn prop_non_mapping_matrix_invalid(value in arb_simple_value()) {
            let mut result = ValidationResult::new();
            validate_matrix(&value, &mut result);
            prop_assert!(
                !result.is_valid,
                "Non-mapping value should be invalid as matrix"
            );
        }

        /// max-parallel must be a positive number
        #[test]
        fn prop_max_parallel_zero_invalid(_dummy in Just(())) {
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                Value::String("os".to_string()),
                Value::Sequence(vec![Value::String("ubuntu".to_string())]),
            );
            map.insert(
                Value::String("max-parallel".to_string()),
                Value::Number(Number::from(0)),
            );

            let mut result = ValidationResult::new();
            validate_matrix(&Value::Mapping(map), &mut result);
            let has_zero_error = result.issues.iter().any(|i| i.contains("greater than 0"));
            prop_assert!(has_zero_error, "max-parallel=0 should produce error");
        }

        /// fail-fast must be a boolean
        #[test]
        fn prop_fail_fast_non_bool_invalid(value in arb_simple_value().prop_filter("Not bool", |v| !v.is_bool())) {
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                Value::String("os".to_string()),
                Value::Sequence(vec![Value::String("ubuntu".to_string())]),
            );
            map.insert(Value::String("fail-fast".to_string()), value);

            let mut result = ValidationResult::new();
            validate_matrix(&Value::Mapping(map), &mut result);
            let has_bool_error = result.issues.iter().any(|i| i.contains("must be a boolean"));
            prop_assert!(has_bool_error, "Non-bool fail-fast should produce error");
        }
    }

    #[test]
    fn test_get_value_type_null() {
        assert_eq!(get_value_type(&Value::Null), "null");
    }

    #[test]
    fn test_get_value_type_sequence() {
        assert_eq!(get_value_type(&Value::Sequence(vec![])), "array");
    }

    #[test]
    fn test_get_value_type_mapping() {
        assert_eq!(
            get_value_type(&Value::Mapping(serde_yaml::Mapping::new())),
            "object"
        );
    }

    #[test]
    fn test_include_must_be_array() {
        let mut map = serde_yaml::Mapping::new();
        map.insert(
            Value::String("os".to_string()),
            Value::Sequence(vec![Value::String("ubuntu".to_string())]),
        );
        map.insert(
            Value::String("include".to_string()),
            Value::String("invalid".to_string()),
        );

        let mut result = ValidationResult::new();
        validate_matrix(&Value::Mapping(map), &mut result);
        assert!(result.issues.iter().any(|i| i.contains("must be an array")));
    }
}
