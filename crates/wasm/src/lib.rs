//! WebAssembly bindings for wrkflw
//!
//! This crate provides JavaScript/Wasm interfaces for:
//! - Matrix expansion
//! - Workflow validation

use wasm_bindgen::prelude::*;
use wrkflw_matrix::{MatrixConfig, MatrixCombination};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
// (disabled for now, keeping the default allocator)
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize the panic hook for better error messages in the browser
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Expand a matrix configuration into all possible combinations
///
/// # Arguments
/// * `matrix_json` - JSON string of the matrix configuration
///
/// # Returns
/// * JSON string of the expanded combinations
///
/// # Example
/// ```javascript
/// const matrix = {
///   parameters: {
///     os: ["ubuntu-latest", "windows-latest"],
///     node: [14, 16, 18]
///   }
/// };
/// const combinations = expandMatrix(JSON.stringify(matrix));
/// ```
#[wasm_bindgen]
pub fn expandMatrix(matrix_json: &str) -> Result<JsValue, JsValue> {
    // Parse JSON input
    let matrix: MatrixConfig = serde_json::from_str(matrix_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse matrix: {}", e)))?;

    // Expand matrix
    let combinations = wrkflw_matrix::expand_matrix(&matrix)
        .map_err(|e| JsValue::from_str(&format!("Matrix expansion failed: {}", e)))?;

    // Convert to serializable format
    let result: Vec<serde_json::Value> = combinations
        .iter()
        .map(|c| {
            serde_json::json!({
                "values": c.values,
                "is_included": c.is_included
            })
        })
        .collect();

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Format a matrix combination name
///
/// # Arguments
/// * `job_name` - The base job name
/// * `combination_json` - JSON string of the combination values
///
/// # Returns
/// * Formatted combination name
#[wasm_bindgen]
pub fn formatCombinationName(job_name: &str, combination_json: &str) -> Result<String, JsValue> {
    use std::collections::HashMap;

    let values: HashMap<String, serde_yaml::Value> = serde_json::from_str(combination_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse combination: {}", e)))?;

    let combination = MatrixCombination::from_include(values);
    Ok(wrkflw_matrix::format_combination_name(job_name, &combination))
}

/// Validate a workflow structure
///
/// # Arguments
/// * `workflow_json` - JSON string of the workflow YAML content
///
/// # Returns
/// * JSON string of validation results
#[wasm_bindgen]
pub fn validateWorkflow(workflow_json: &str) -> Result<JsValue, JsValue> {
    use serde_yaml::Value;
    use wrkflw_models::ValidationResult;
    use wrkflw_validators::{validate_jobs, validate_triggers};

    let workflow: Value = serde_json::from_str(workflow_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse workflow: {}", e)))?;

    let mut result = ValidationResult::new();

    // Validate triggers
    if let Some(on) = workflow.get("on") {
        validate_triggers(on, &mut result);
    }

    // Validate jobs
    if let Some(jobs) = workflow.get("jobs") {
        validate_jobs(jobs, &mut result);
    }

    // Convert issues to JSON
    let issues: Vec<serde_json::Value> = result
        .issues
        .iter()
        .map(|issue| {
            serde_json::json!({
                "severity": "error",
                "message": issue
            })
        })
        .collect();

    serde_wasm_bindgen::to_value(&serde_json::json!({
        "is_valid": result.is_valid,
        "issues": issues
    }))
    .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_matrix() {
        let matrix = serde_json::json!({
            "parameters": {
                "os": ["ubuntu-latest", "windows-latest"],
                "node": [14, 16]
            }
        });

        let matrix_json = serde_json::to_string(&matrix).unwrap();
        let result = expandMatrix(&matrix_json);

        assert!(result.is_ok());

        let combinations: Vec<serde_json::Value> = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
        assert_eq!(combinations.len(), 4); // 2 os x 2 node = 4 combinations
    }

    #[test]
    fn test_format_combination_name() {
        let combination = serde_json::json!({
            "os": "ubuntu-latest",
            "node": 16
        });

        let combination_json = serde_json::to_string(&combination).unwrap();
        let result = formatCombinationName("test", &combination_json);

        assert!(result.is_ok());
        let name = result.unwrap();
        assert!(name.contains("test"));
        assert!(name.contains("os"));
        assert!(name.contains("node"));
    }
}
