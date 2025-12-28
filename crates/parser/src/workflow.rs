use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use wrkflw_matrix::MatrixConfig;

use super::schema::SchemaValidator;

// Custom deserializer for needs field that handles both string and array formats
fn deserialize_needs<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let value = Option::<StringOrVec>::deserialize(deserializer)?;
    match value {
        Some(StringOrVec::String(s)) => Ok(Some(vec![s])),
        Some(StringOrVec::Vec(v)) => Ok(Some(v)),
        None => Ok(None),
    }
}

// Custom deserializer for runs-on field that handles both string and array formats
fn deserialize_runs_on<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    let value = Option::<StringOrVec>::deserialize(deserializer)?;
    match value {
        Some(StringOrVec::String(s)) => Ok(Some(vec![s])),
        Some(StringOrVec::Vec(v)) => Ok(Some(v)),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowDefinition {
    pub name: String,
    #[serde(skip, default)] // Skip deserialization of the 'on' field directly
    pub on: Vec<String>,
    #[serde(rename = "on")] // Raw access to the 'on' field for custom handling
    pub on_raw: serde_yaml::Value,
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename = "runs-on", default, deserialize_with = "deserialize_runs_on")]
    pub runs_on: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_needs")]
    pub needs: Option<Vec<String>>,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub matrix: Option<MatrixConfig>,
    #[serde(default)]
    pub services: HashMap<String, Service>,
    #[serde(default, rename = "if")]
    pub if_condition: Option<String>,
    #[serde(default)]
    pub outputs: Option<HashMap<String, String>>,
    #[serde(default)]
    pub permissions: Option<HashMap<String, String>>,
    // Reusable workflow (job-level 'uses') support
    #[serde(default)]
    pub uses: Option<String>,
    #[serde(default)]
    pub with: Option<HashMap<String, String>>,
    #[serde(default)]
    pub secrets: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    pub image: String,
    #[serde(default)]
    pub ports: Option<Vec<String>>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub volumes: Option<Vec<String>>,
    #[serde(default)]
    pub options: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Step {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub uses: Option<String>,
    #[serde(default)]
    pub run: Option<String>,
    #[serde(default)]
    pub with: Option<HashMap<String, String>>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub continue_on_error: Option<bool>,
}

impl WorkflowDefinition {
    pub fn resolve_action(&self, action_ref: &str) -> ActionInfo {
        // Parse GitHub action reference like "actions/checkout@v3"
        let parts: Vec<&str> = action_ref.split('@').collect();

        let (repo, _) = if parts.len() > 1 {
            (parts[0], parts[1])
        } else {
            (parts[0], "main") // Default to main if no version specified
        };

        ActionInfo {
            repository: repo.to_string(),
            is_docker: repo.starts_with("docker://"),
            is_local: repo.starts_with("./"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub repository: String,
    pub is_docker: bool,
    pub is_local: bool,
}

pub fn parse_workflow(path: &Path) -> Result<WorkflowDefinition, String> {
    // First validate against schema
    let validator = SchemaValidator::new()?;
    validator.validate_workflow(path)?;

    // If validation passes, parse the workflow
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read workflow file: {}", e))?;

    // Parse the YAML content
    let mut workflow: WorkflowDefinition = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse workflow structure: {}", e))?;

    // Normalize the trigger events
    workflow.on = normalize_triggers(&workflow.on_raw)?;

    Ok(workflow)
}

pub(crate) fn normalize_triggers(on_value: &serde_yaml::Value) -> Result<Vec<String>, String> {
    let mut triggers = Vec::new();

    match on_value {
        // Simple string trigger: on: push
        serde_yaml::Value::String(event) => {
            triggers.push(event.clone());
        }
        // Array of triggers: on: [push, pull_request]
        serde_yaml::Value::Sequence(events) => {
            for event in events {
                if let Some(event_str) = event.as_str() {
                    triggers.push(event_str.to_string());
                }
            }
        }
        // Map of triggers with configuration: on: {push: {branches: [main]}}
        serde_yaml::Value::Mapping(events_map) => {
            for (event, _) in events_map {
                if let Some(event_str) = event.as_str() {
                    triggers.push(event_str.to_string());
                }
            }
        }
        _ => {
            return Err("'on' section has invalid format".to_string());
        }
    }

    Ok(triggers)
}

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;
    use serde_yaml::Value;

    // Valid GitHub Actions trigger events
    const VALID_EVENTS: &[&str] = &[
        "push",
        "pull_request",
        "workflow_dispatch",
        "schedule",
        "release",
        "create",
        "delete",
        "fork",
        "issues",
        "issue_comment",
    ];

    /// Generate a valid trigger event name
    fn arb_valid_event() -> impl Strategy<Value = String> {
        proptest::sample::select(VALID_EVENTS).prop_map(|s| s.to_string())
    }

    /// Generate a valid action reference with version
    fn arb_action_with_version() -> impl Strategy<Value = String> {
        (
            "[a-zA-Z][a-zA-Z0-9_-]{2,15}",
            "[a-zA-Z][a-zA-Z0-9_-]{2,15}",
            "(v[0-9]+|main|master|[a-f0-9]{7,40})",
        )
            .prop_map(|(owner, repo, version)| format!("{}/{}@{}", owner, repo, version))
    }

    /// Generate a docker action reference
    fn arb_docker_action() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_/-]{2,30}(:[a-zA-Z0-9._-]+)?".prop_map(|image| format!("docker://{}", image))
    }

    /// Generate a local action reference
    fn arb_local_action() -> impl Strategy<Value = String> {
        "[a-zA-Z][a-zA-Z0-9_/-]{1,20}".prop_map(|path| format!("./{}", path))
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// normalize_triggers handles string format
        #[test]
        fn prop_normalize_string_trigger(event in arb_valid_event()) {
            let value = Value::String(event.clone());
            let result = normalize_triggers(&value);
            prop_assert!(result.is_ok());
            let triggers = result.unwrap();
            prop_assert_eq!(triggers.len(), 1);
            prop_assert_eq!(&triggers[0], &event);
        }

        /// normalize_triggers handles array format
        #[test]
        fn prop_normalize_array_triggers(events in proptest::collection::vec(arb_valid_event(), 1..=5)) {
            let value = Value::Sequence(events.iter().map(|s| Value::String(s.clone())).collect());
            let result = normalize_triggers(&value);
            prop_assert!(result.is_ok());
            let triggers = result.unwrap();
            prop_assert_eq!(triggers.len(), events.len());
            for (expected, actual) in events.iter().zip(triggers.iter()) {
                prop_assert_eq!(expected, actual);
            }
        }

        /// normalize_triggers handles mapping format
        #[test]
        fn prop_normalize_mapping_triggers(events in proptest::collection::vec(arb_valid_event(), 1..=5)) {
            let mut map = serde_yaml::Mapping::new();
            for event in &events {
                map.insert(Value::String(event.clone()), Value::Null);
            }
            let value = Value::Mapping(map.clone());
            let result = normalize_triggers(&value);
            prop_assert!(result.is_ok());
            let triggers = result.unwrap();
            // Mapping keys are unique, so check against actual map size
            prop_assert_eq!(triggers.len(), map.len());
        }

        /// normalize_triggers rejects null
        #[test]
        fn prop_normalize_null_fails(_dummy in Just(())) {
            let result = normalize_triggers(&Value::Null);
            prop_assert!(result.is_err());
        }

        /// resolve_action correctly parses versioned actions
        #[test]
        fn prop_resolve_versioned_action(action_ref in arb_action_with_version()) {
            let workflow = WorkflowDefinition {
                name: "test".to_string(),
                on: vec![],
                on_raw: Value::Null,
                jobs: HashMap::new(),
            };
            let info = workflow.resolve_action(&action_ref);
            prop_assert!(!info.is_docker);
            prop_assert!(!info.is_local);
            // Repository should be the part before @
            let expected_repo = action_ref.split('@').next().unwrap();
            prop_assert_eq!(info.repository, expected_repo);
        }

        /// resolve_action correctly identifies docker actions
        #[test]
        fn prop_resolve_docker_action(action_ref in arb_docker_action()) {
            let workflow = WorkflowDefinition {
                name: "test".to_string(),
                on: vec![],
                on_raw: Value::Null,
                jobs: HashMap::new(),
            };
            let info = workflow.resolve_action(&action_ref);
            prop_assert!(info.is_docker, "Docker action should be identified as docker");
            prop_assert!(!info.is_local);
        }

        /// resolve_action correctly identifies local actions
        #[test]
        fn prop_resolve_local_action(action_ref in arb_local_action()) {
            let workflow = WorkflowDefinition {
                name: "test".to_string(),
                on: vec![],
                on_raw: Value::Null,
                jobs: HashMap::new(),
            };
            let info = workflow.resolve_action(&action_ref);
            prop_assert!(info.is_local, "Local action should be identified as local");
            prop_assert!(!info.is_docker);
        }
    }

    #[test]
    fn test_normalize_empty_sequence() {
        let value = Value::Sequence(vec![]);
        let result = normalize_triggers(&value);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_resolve_action_without_version() {
        let workflow = WorkflowDefinition {
            name: "test".to_string(),
            on: vec![],
            on_raw: Value::Null,
            jobs: HashMap::new(),
        };
        let info = workflow.resolve_action("actions/checkout");
        assert_eq!(info.repository, "actions/checkout");
        assert!(!info.is_docker);
        assert!(!info.is_local);
    }

    #[test]
    fn test_resolve_well_known_actions() {
        let workflow = WorkflowDefinition {
            name: "test".to_string(),
            on: vec![],
            on_raw: Value::Null,
            jobs: HashMap::new(),
        };

        // Test actions/checkout
        let info = workflow.resolve_action("actions/checkout@v4");
        assert_eq!(info.repository, "actions/checkout");

        // Test docker action
        let info = workflow.resolve_action("docker://alpine:3.14");
        assert!(info.is_docker);

        // Test local action
        let info = workflow.resolve_action("./.github/actions/my-action");
        assert!(info.is_local);
    }
}
