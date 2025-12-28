use serde_yaml::{self, Value};

#[cfg(feature = "cli")]
use colored::*;

#[cfg(feature = "cli")]
use std::fs;

#[cfg(feature = "cli")]
use std::path::Path;

use wrkflw_models::ValidationResult;
use wrkflw_validators::{validate_jobs, validate_triggers};

/// Evaluate a workflow from YAML content string.
/// This function does not depend on filesystem access.
pub fn evaluate_workflow_content(content: &str) -> Result<ValidationResult, String> {
    // Parse YAML content
    let workflow: Value =
        serde_yaml::from_str(content).map_err(|e| format!("Invalid YAML: {}", e))?;

    let mut result = ValidationResult::new();

    // Check for required structure
    if !workflow.is_mapping() {
        result.add_issue("Workflow file is not a valid YAML mapping".to_string());
        return Ok(result);
    }

    // Check if jobs section exists
    match workflow.get("jobs") {
        Some(jobs) if jobs.is_mapping() => {
            validate_jobs(jobs, &mut result);
        }
        Some(_) => {
            result.add_issue("'jobs' section is not a mapping".to_string());
        }
        None => {
            result.add_issue("Workflow is missing 'jobs' section".to_string());
        }
    }

    // Check for valid triggers
    match workflow.get("on") {
        Some(on) => {
            validate_triggers(on, &mut result);
        }
        None => {
            result.add_issue("Workflow is missing 'on' section (triggers)".to_string());
        }
    }

    Ok(result)
}

/// Evaluate a workflow from a file path.
/// This function requires filesystem access (cli feature).
#[cfg(feature = "cli")]
pub fn evaluate_workflow_file(path: &Path, verbose: bool) -> Result<ValidationResult, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let result = evaluate_workflow_content(&content)?;

    if verbose && result.is_valid {
        println!(
            "{} Validated structure of workflow: {}",
            "✓".green(),
            path.display()
        );
    }

    Ok(result)
}
