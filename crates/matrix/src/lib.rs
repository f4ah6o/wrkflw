// matrix crate

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MatrixConfig {
    #[serde(flatten)]
    pub parameters: IndexMap<String, Value>,
    #[serde(default)]
    pub include: Vec<HashMap<String, Value>>,
    #[serde(default)]
    pub exclude: Vec<HashMap<String, Value>>,
    #[serde(default, rename = "max-parallel")]
    pub max_parallel: Option<usize>,
    #[serde(default, rename = "fail-fast")]
    pub fail_fast: Option<bool>,
}

impl Default for MatrixConfig {
    fn default() -> Self {
        Self {
            parameters: IndexMap::new(),
            include: Vec::new(),
            exclude: Vec::new(),
            max_parallel: None,
            fail_fast: Some(true),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixCombination {
    pub values: HashMap<String, Value>,
    pub is_included: bool, // Whether this was added via the include section
}

impl MatrixCombination {
    pub fn new(values: HashMap<String, Value>) -> Self {
        Self {
            values,
            is_included: false,
        }
    }

    pub fn from_include(values: HashMap<String, Value>) -> Self {
        Self {
            values,
            is_included: true,
        }
    }
}

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("Invalid matrix parameter format: {0}")]
    InvalidParameterFormat(String),

    #[error("Failed to expand matrix: {0}")]
    ExpansionError(String),
}

/// Expands a matrix configuration into a list of all valid combinations
pub fn expand_matrix(matrix: &MatrixConfig) -> Result<Vec<MatrixCombination>, MatrixError> {
    let mut combinations = Vec::new();

    // Step 1: Generate base combinations from parameter arrays
    let param_combinations = generate_base_combinations(matrix)?;

    // Step 2: Filter out any combinations that match the exclude patterns
    let filtered_combinations = apply_exclude_filters(param_combinations, &matrix.exclude);
    combinations.extend(filtered_combinations);

    // Step 3: Add any combinations from the include section
    for include_item in &matrix.include {
        combinations.push(MatrixCombination::from_include(include_item.clone()));
    }

    if combinations.is_empty() {
        return Err(MatrixError::ExpansionError(
            "No valid combinations found after applying filters".to_string(),
        ));
    }

    Ok(combinations)
}

/// Deduplicates values in a sequence while preserving order.
/// Uses string representation for comparison since Value doesn't implement Hash.
fn deduplicate_values(values: &[Value]) -> Vec<Value> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for value in values {
        // Use debug representation as a key for deduplication
        let key = format!("{:?}", value);
        if seen.insert(key) {
            result.push(value.clone());
        }
    }

    result
}

/// Generates all possible combinations of the base matrix parameters
pub(crate) fn generate_base_combinations(
    matrix: &MatrixConfig,
) -> Result<Vec<MatrixCombination>, MatrixError> {
    // Extract parameter arrays and prepare for combination generation
    let mut param_arrays: IndexMap<String, Vec<Value>> = IndexMap::new();

    for (param_name, param_value) in &matrix.parameters {
        match param_value {
            Value::Sequence(array) => {
                // Deduplicate values while preserving order
                let deduped = deduplicate_values(array);
                param_arrays.insert(param_name.clone(), deduped);
            }
            _ => {
                // Handle non-array parameters
                let single_value = vec![param_value.clone()];
                param_arrays.insert(param_name.clone(), single_value);
            }
        }
    }

    if param_arrays.is_empty() {
        return Err(MatrixError::InvalidParameterFormat(
            "Matrix has no valid parameters".to_string(),
        ));
    }

    // Generate the Cartesian product of all parameter arrays
    let param_names: Vec<String> = param_arrays.keys().cloned().collect();
    let param_values: Vec<Vec<Value>> = param_arrays.values().cloned().collect();

    // Generate all combinations using itertools
    let combinations = if !param_values.is_empty() {
        generate_combinations(&param_names, &param_values, 0, &mut HashMap::new())?
    } else {
        vec![]
    };

    Ok(combinations)
}

/// Recursive function to generate combinations using depth-first approach
fn generate_combinations(
    param_names: &[String],
    param_values: &[Vec<Value>],
    current_depth: usize,
    current_combination: &mut HashMap<String, Value>,
) -> Result<Vec<MatrixCombination>, MatrixError> {
    if current_depth == param_names.len() {
        // We've reached a complete combination
        return Ok(vec![MatrixCombination::new(current_combination.clone())]);
    }

    let mut result = Vec::new();
    let param_name = &param_names[current_depth];
    let values = &param_values[current_depth];

    for value in values {
        current_combination.insert(param_name.clone(), value.clone());

        let mut new_combinations = generate_combinations(
            param_names,
            param_values,
            current_depth + 1,
            current_combination,
        )?;

        result.append(&mut new_combinations);
    }

    // Remove this level's parameter to backtrack
    current_combination.remove(param_name);

    Ok(result)
}

/// Filters out combinations that match any of the exclude patterns
pub(crate) fn apply_exclude_filters(
    combinations: Vec<MatrixCombination>,
    exclude_patterns: &[HashMap<String, Value>],
) -> Vec<MatrixCombination> {
    if exclude_patterns.is_empty() {
        return combinations;
    }

    combinations
        .into_iter()
        .filter(|combination| !is_excluded(combination, exclude_patterns))
        .collect()
}

/// Checks if a combination matches any exclude pattern
pub(crate) fn is_excluded(
    combination: &MatrixCombination,
    exclude_patterns: &[HashMap<String, Value>],
) -> bool {
    for exclude in exclude_patterns {
        let mut excluded = true;

        for (key, value) in exclude {
            match combination.values.get(key) {
                Some(combo_value) if combo_value == value => {
                    // This exclude condition matches
                    continue;
                }
                _ => {
                    // This exclude condition doesn't match
                    excluded = false;
                    break;
                }
            }
        }

        if excluded {
            return true;
        }
    }

    false
}

/// Formats a combination name for display, e.g. "test (ubuntu, node 14)"
pub fn format_combination_name(job_name: &str, combination: &MatrixCombination) -> String {
    let params = combination
        .values
        .iter()
        .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
        .collect::<Vec<_>>()
        .join(", ");

    format!("{} ({})", job_name, params)
}

/// Converts a serde_yaml::Value to a string for display
pub(crate) fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Sequence(seq) => {
            let items = seq
                .iter()
                .map(value_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", items)
        }
        Value::Mapping(map) => {
            let items = map
                .iter()
                .map(|(k, v)| format!("{}: {}", value_to_string(k), value_to_string(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", items)
        }
        Value::Null => "null".to_string(),
        _ => "unknown".to_string(),
    }
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::collection::{hash_map, vec};
    use proptest::prelude::*;
    use serde_yaml::Number;
    use std::collections::HashSet;

    // ============================================================
    // STRATEGIES
    // ============================================================

    /// Generates simple YAML values suitable for matrix parameters.
    fn arb_simple_value() -> impl Strategy<Value = Value> {
        prop_oneof![
            "[a-zA-Z][a-zA-Z0-9_]{0,15}".prop_map(Value::String),
            (0i64..=100).prop_map(|n| Value::Number(Number::from(n))),
            any::<bool>().prop_map(Value::Bool),
        ]
    }

    /// Generates parameter names (valid identifiers)
    fn arb_param_name() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_]{0,10}"
    }

    /// Generates a non-empty sequence of simple values for a single parameter.
    fn arb_param_array() -> impl Strategy<Value = Value> {
        vec(arb_simple_value(), 1..=4).prop_map(Value::Sequence)
    }

    /// Generates matrix parameters as an IndexMap.
    fn arb_parameters() -> impl Strategy<Value = IndexMap<String, Value>> {
        vec((arb_param_name(), arb_param_array()), 1..=3).prop_map(|pairs| {
            let mut map = IndexMap::new();
            for (k, v) in pairs {
                map.insert(k, v);
            }
            map
        })
    }

    /// Generates a single exclude/include pattern.
    fn arb_pattern() -> impl Strategy<Value = HashMap<String, Value>> {
        hash_map(arb_param_name(), arb_simple_value(), 1..=2)
    }

    /// Generates a vector of patterns (for include/exclude lists)
    fn arb_pattern_list() -> impl Strategy<Value = Vec<HashMap<String, Value>>> {
        vec(arb_pattern(), 0..=2)
    }

    /// Generates a complete, valid MatrixConfig suitable for testing.
    fn arb_matrix_config() -> impl Strategy<Value = MatrixConfig> {
        (
            arb_parameters(),
            arb_pattern_list(),
            arb_pattern_list(),
            proptest::option::of(1usize..=10),
            proptest::option::of(any::<bool>()),
        )
            .prop_map(|(params, include, exclude, max_parallel, fail_fast)| MatrixConfig {
                parameters: params,
                include,
                exclude,
                max_parallel,
                fail_fast,
            })
    }

    /// Generates a MatrixCombination for testing format_combination_name
    fn arb_matrix_combination() -> impl Strategy<Value = MatrixCombination> {
        (hash_map(arb_param_name(), arb_simple_value(), 1..=4), any::<bool>()).prop_map(
            |(values, is_included)| MatrixCombination { values, is_included },
        )
    }

    /// Generates a job name for testing format_combination_name
    fn arb_job_name() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_-]{0,20}"
    }

    /// Extended strategy for all Value variants including complex types
    fn arb_any_value() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            "[a-zA-Z0-9_-]{0,15}".prop_map(Value::String),
            (0i64..100).prop_map(|n| Value::Number(Number::from(n))),
            any::<bool>().prop_map(Value::Bool),
            Just(Value::Null),
        ];

        leaf.prop_recursive(2, 32, 5, |inner| {
            prop_oneof![
                vec(inner.clone(), 0..4).prop_map(Value::Sequence),
                vec((arb_simple_value(), inner), 0..3).prop_map(|pairs| {
                    let mut map = serde_yaml::Mapping::new();
                    for (k, v) in pairs {
                        map.insert(k, v);
                    }
                    Value::Mapping(map)
                }),
            ]
        })
    }

    // ============================================================
    // PROPERTY TESTS: Cartesian Product
    // ============================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// The number of base combinations equals the product of deduplicated parameter array lengths.
        #[test]
        fn prop_cartesian_product_size(params in arb_parameters()) {
            let matrix = MatrixConfig {
                parameters: params.clone(),
                include: vec![],
                exclude: vec![],
                max_parallel: None,
                fail_fast: Some(true),
            };

            // Calculate expected size based on deduplicated arrays
            let expected_size: usize = params.values()
                .map(|v| match v {
                    Value::Sequence(seq) => {
                        let unique: HashSet<_> = seq.iter().map(|x| format!("{:?}", x)).collect();
                        unique.len()
                    },
                    _ => 1,
                })
                .product();

            let result = expand_matrix(&matrix);
            prop_assert!(result.is_ok(), "expand_matrix failed: {:?}", result);

            let combinations = result.unwrap();
            prop_assert_eq!(
                combinations.len(),
                expected_size,
                "Expected {} combinations but got {}",
                expected_size,
                combinations.len()
            );
        }

        /// No combination in the output should match any exclude pattern.
        #[test]
        fn prop_exclude_invariant(config in arb_matrix_config()) {
            if let Ok(combinations) = expand_matrix(&config) {
                for combination in &combinations {
                    if combination.is_included {
                        continue;
                    }

                    for exclude_pattern in &config.exclude {
                        let matches_exclude = exclude_pattern.iter().all(|(key, value)| {
                            combination.values.get(key) == Some(value)
                        });

                        prop_assert!(
                            !matches_exclude,
                            "Combination {:?} matches exclude pattern {:?}",
                            combination.values,
                            exclude_pattern
                        );
                    }
                }
            }
        }

        /// All items from the include section should appear in the output.
        #[test]
        fn prop_include_invariant(config in arb_matrix_config()) {
            if let Ok(combinations) = expand_matrix(&config) {
                for include_item in &config.include {
                    let found = combinations.iter().any(|c| {
                        c.is_included &&
                        include_item.iter().all(|(k, v)| c.values.get(k) == Some(v))
                    });

                    prop_assert!(
                        found,
                        "Include item {:?} not found in combinations",
                        include_item
                    );
                }
            }
        }

        /// The number of included combinations equals the number of include items.
        #[test]
        fn prop_include_count(config in arb_matrix_config()) {
            if let Ok(combinations) = expand_matrix(&config) {
                let included_count = combinations.iter()
                    .filter(|c| c.is_included)
                    .count();

                prop_assert_eq!(
                    included_count,
                    config.include.len(),
                    "Expected {} included items but got {}",
                    config.include.len(),
                    included_count
                );
            }
        }

        /// Base combinations (non-included) should never have duplicates,
        /// even when parameter arrays contain duplicate values.
        #[test]
        fn prop_no_duplicate_base_combinations(params in arb_parameters()) {
            let matrix = MatrixConfig {
                parameters: params,
                include: vec![],
                exclude: vec![],
                max_parallel: None,
                fail_fast: Some(true),
            };

            if let Ok(combinations) = expand_matrix(&matrix) {
                for (i, c1) in combinations.iter().enumerate() {
                    for (j, c2) in combinations.iter().enumerate() {
                        if i < j && !c1.is_included && !c2.is_included {
                            let are_equal = c1.values.len() == c2.values.len()
                                && c1.values.iter().all(|(k, v)| c2.values.get(k) == Some(v));

                            prop_assert!(
                                !are_equal,
                                "Duplicate combinations found at indices {} and {}: {:?}",
                                i, j, c1.values
                            );
                        }
                    }
                }
            }
        }
    }

    // ============================================================
    // PROPERTY TESTS: format_combination_name
    // ============================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]

        /// The formatted name should always contain the job name.
        #[test]
        fn prop_format_contains_job_name(
            job_name in arb_job_name(),
            combination in arb_matrix_combination()
        ) {
            let formatted = format_combination_name(&job_name, &combination);

            prop_assert!(
                formatted.contains(&job_name),
                "Formatted name '{}' does not contain job name '{}'",
                formatted,
                job_name
            );
        }

        /// The formatted name should contain parameter keys.
        #[test]
        fn prop_format_contains_param_keys(
            job_name in arb_job_name(),
            combination in arb_matrix_combination()
        ) {
            let formatted = format_combination_name(&job_name, &combination);

            for key in combination.values.keys() {
                prop_assert!(
                    formatted.contains(key),
                    "Formatted name '{}' does not contain key '{}'",
                    formatted,
                    key
                );
            }
        }

        /// The formatted name should have the pattern "job_name (params)"
        #[test]
        fn prop_format_structure(
            job_name in arb_job_name(),
            combination in arb_matrix_combination()
        ) {
            let formatted = format_combination_name(&job_name, &combination);

            prop_assert!(
                formatted.starts_with(&job_name),
                "Formatted name should start with job name"
            );
            prop_assert!(
                formatted.contains('(') && formatted.contains(')'),
                "Formatted name should contain parentheses"
            );
        }
    }

    // ============================================================
    // PROPERTY TESTS: value_to_string
    // ============================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(500))]

        /// value_to_string should never panic for any Value variant.
        #[test]
        fn prop_value_to_string_no_panic(value in arb_any_value()) {
            // Should not panic
            let result = value_to_string(&value);

            // Empty result is valid for empty strings and null
            let is_valid_empty = matches!(&value, Value::Null)
                || matches!(&value, Value::String(s) if s.is_empty());

            prop_assert!(
                !result.is_empty() || is_valid_empty,
                "value_to_string returned unexpected empty for {:?}",
                value
            );
        }

        /// String values should return their content unchanged.
        #[test]
        fn prop_value_to_string_identity_for_strings(s in "[a-zA-Z0-9_-]{1,30}") {
            let value = Value::String(s.clone());
            let result = value_to_string(&value);
            prop_assert_eq!(result, s);
        }

        /// Boolean values should return "true" or "false".
        #[test]
        fn prop_value_to_string_booleans(b in any::<bool>()) {
            let value = Value::Bool(b);
            let result = value_to_string(&value);
            prop_assert_eq!(result, b.to_string());
        }

        /// Sequences should be formatted with brackets.
        #[test]
        fn prop_value_to_string_sequence(items in vec(arb_simple_value(), 0..4)) {
            let value = Value::Sequence(items);
            let result = value_to_string(&value);

            prop_assert!(result.starts_with('['), "Sequence should start with [");
            prop_assert!(result.ends_with(']'), "Sequence should end with ]");
        }

        /// Mappings should be formatted with braces.
        #[test]
        fn prop_value_to_string_mapping(pairs in vec((arb_simple_value(), arb_simple_value()), 0..3)) {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in pairs {
                map.insert(k, v);
            }
            let value = Value::Mapping(map);
            let result = value_to_string(&value);

            prop_assert!(result.starts_with('{'), "Mapping should start with {{");
            prop_assert!(result.ends_with('}'), "Mapping should end with }}");
        }
    }

    // ============================================================
    // PROPERTY TESTS: Edge Cases
    // ============================================================

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        /// Single value parameters work correctly.
        #[test]
        fn prop_single_value_params(value in arb_simple_value()) {
            let mut params = IndexMap::new();
            params.insert("single".to_string(), value);

            let matrix = MatrixConfig {
                parameters: params,
                include: vec![],
                exclude: vec![],
                max_parallel: None,
                fail_fast: Some(true),
            };

            let result = expand_matrix(&matrix);
            prop_assert!(result.is_ok());

            let combinations = result.unwrap();
            prop_assert_eq!(combinations.len(), 1, "Single value should produce 1 combination");
        }

        /// All combinations excluded but includes present = success with only includes.
        #[test]
        fn prop_all_excluded_with_includes_succeeds(include_item in arb_pattern()) {
            let mut params = IndexMap::new();
            params.insert(
                "only".to_string(),
                Value::Sequence(vec![Value::String("value".to_string())]),
            );

            let mut exclude = HashMap::new();
            exclude.insert("only".to_string(), Value::String("value".to_string()));

            let matrix = MatrixConfig {
                parameters: params,
                include: vec![include_item],
                exclude: vec![exclude],
                max_parallel: None,
                fail_fast: Some(true),
            };

            let result = expand_matrix(&matrix);
            prop_assert!(result.is_ok(), "Should succeed with includes");

            let combinations = result.unwrap();
            prop_assert_eq!(combinations.len(), 1);
            prop_assert!(combinations[0].is_included);
        }

        /// expand_matrix is deterministic - same input produces same output.
        #[test]
        fn prop_expand_deterministic(config in arb_matrix_config()) {
            let result1 = expand_matrix(&config);
            let result2 = expand_matrix(&config);

            match (&result1, &result2) {
                (Ok(c1), Ok(c2)) => {
                    prop_assert_eq!(
                        c1.len(),
                        c2.len(),
                        "Determinism failed: different lengths"
                    );
                }
                (Err(_), Err(_)) => {}
                _ => prop_assert!(false, "Inconsistent results"),
            }
        }

        /// Each base combination has exactly the parameters from the config.
        #[test]
        fn prop_combination_has_all_params(config in arb_matrix_config()) {
            if let Ok(combinations) = expand_matrix(&config) {
                let param_keys: HashSet<String> = config.parameters.keys().cloned().collect();

                for combo in &combinations {
                    if !combo.is_included {
                        let combo_keys: HashSet<String> = combo.values.keys().cloned().collect();

                        prop_assert_eq!(
                            param_keys.clone(),
                            combo_keys,
                            "Combination keys don't match parameter keys"
                        );
                    }
                }
            }
        }
    }

    // ============================================================
    // UNIT TESTS: Null handling
    // ============================================================

    #[test]
    fn test_value_to_string_null() {
        let result = value_to_string(&Value::Null);
        assert_eq!(result, "null");
    }

    #[test]
    fn test_empty_params_error() {
        let matrix = MatrixConfig {
            parameters: IndexMap::new(),
            include: vec![],
            exclude: vec![],
            max_parallel: None,
            fail_fast: Some(true),
        };

        let result = expand_matrix(&matrix);
        assert!(result.is_err(), "Empty matrix should produce error");
    }

    #[test]
    fn test_all_excluded_with_no_includes_error() {
        let mut params = IndexMap::new();
        params.insert(
            "only".to_string(),
            Value::Sequence(vec![Value::String("value".to_string())]),
        );

        let mut exclude = HashMap::new();
        exclude.insert("only".to_string(), Value::String("value".to_string()));

        let matrix = MatrixConfig {
            parameters: params,
            include: vec![],
            exclude: vec![exclude],
            max_parallel: None,
            fail_fast: Some(true),
        };

        let result = expand_matrix(&matrix);
        assert!(result.is_err(), "All excluded with no includes should error");
    }

    #[test]
    fn test_duplicate_values_deduplicated() {
        // Test that duplicate values in parameter arrays are deduplicated
        let mut params = IndexMap::new();
        params.insert(
            "os".to_string(),
            Value::Sequence(vec![
                Value::String("ubuntu".to_string()),
                Value::String("ubuntu".to_string()), // duplicate
                Value::String("windows".to_string()),
            ]),
        );

        let matrix = MatrixConfig {
            parameters: params,
            include: vec![],
            exclude: vec![],
            max_parallel: None,
            fail_fast: Some(true),
        };

        let result = expand_matrix(&matrix);
        assert!(result.is_ok());

        let combinations = result.unwrap();
        // Should be 2 combinations (ubuntu, windows), not 3
        assert_eq!(combinations.len(), 2, "Duplicates should be removed");
    }

    #[test]
    fn test_deduplicate_preserves_order() {
        let values = vec![
            Value::String("c".to_string()),
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("a".to_string()), // duplicate
            Value::String("c".to_string()), // duplicate
        ];

        let deduped = super::deduplicate_values(&values);

        assert_eq!(deduped.len(), 3);
        assert_eq!(deduped[0], Value::String("c".to_string()));
        assert_eq!(deduped[1], Value::String("a".to_string()));
        assert_eq!(deduped[2], Value::String("b".to_string()));
    }
}
