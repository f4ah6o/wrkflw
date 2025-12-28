pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    pub fn new() -> Self {
        ValidationResult {
            is_valid: true,
            issues: Vec::new(),
        }
    }

    pub fn add_issue(&mut self, issue: String) {
        self.is_valid = false;
        self.issues.push(issue);
    }
}

// GitLab pipeline models
pub mod gitlab {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// Represents a GitLab CI/CD pipeline configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Pipeline {
        /// Default image for all jobs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<Image>,

        /// Global variables available to all jobs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub variables: Option<HashMap<String, String>>,

        /// Pipeline stages in execution order
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stages: Option<Vec<String>>,

        /// Default before_script for all jobs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before_script: Option<Vec<String>>,

        /// Default after_script for all jobs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after_script: Option<Vec<String>>,

        /// Job definitions (name => job)
        #[serde(flatten)]
        pub jobs: HashMap<String, Job>,

        /// Workflow rules for the pipeline
        #[serde(skip_serializing_if = "Option::is_none")]
        pub workflow: Option<Workflow>,

        /// Includes for pipeline configuration
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include: Option<Vec<Include>>,
    }

    /// A job in a GitLab CI/CD pipeline
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Job {
        /// The stage this job belongs to
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stage: Option<String>,

        /// Docker image to use for this job
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<Image>,

        /// Script commands to run
        #[serde(skip_serializing_if = "Option::is_none")]
        pub script: Option<Vec<String>>,

        /// Commands to run before the main script
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before_script: Option<Vec<String>>,

        /// Commands to run after the main script
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after_script: Option<Vec<String>>,

        /// When to run the job (on_success, on_failure, always, manual)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub when: Option<String>,

        /// Allow job failure
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allow_failure: Option<bool>,

        /// Services to run alongside the job
        #[serde(skip_serializing_if = "Option::is_none")]
        pub services: Option<Vec<Service>>,

        /// Tags to define which runners can execute this job
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,

        /// Job-specific variables
        #[serde(skip_serializing_if = "Option::is_none")]
        pub variables: Option<HashMap<String, String>>,

        /// Job dependencies
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dependencies: Option<Vec<String>>,

        /// Artifacts to store after job execution
        #[serde(skip_serializing_if = "Option::is_none")]
        pub artifacts: Option<Artifacts>,

        /// Cache configuration
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache: Option<Cache>,

        /// Rules for when this job should run
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rules: Option<Vec<Rule>>,

        /// Only run on specified refs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub only: Option<Only>,

        /// Exclude specified refs
        #[serde(skip_serializing_if = "Option::is_none")]
        pub except: Option<Except>,

        /// Retry configuration
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retry: Option<Retry>,

        /// Timeout for the job in seconds
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<String>,

        /// Mark job as parallel and specify instance count
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parallel: Option<usize>,

        /// Flag to indicate this is a template job
        #[serde(skip_serializing_if = "Option::is_none")]
        pub template: Option<bool>,

        /// List of jobs this job extends from
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extends: Option<Vec<String>>,
    }

    /// Docker image configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Image {
        /// Simple image name as string
        Simple(String),
        /// Detailed image configuration
        Detailed {
            /// Image name
            name: String,
            /// Entrypoint to override in the image
            #[serde(skip_serializing_if = "Option::is_none")]
            entrypoint: Option<Vec<String>>,
        },
    }

    /// Service container to run alongside a job
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Service {
        /// Simple service name as string
        Simple(String),
        /// Detailed service configuration
        Detailed {
            /// Service name/image
            name: String,
            /// Command to run in the service container
            #[serde(skip_serializing_if = "Option::is_none")]
            command: Option<Vec<String>>,
            /// Entrypoint to override in the image
            #[serde(skip_serializing_if = "Option::is_none")]
            entrypoint: Option<Vec<String>>,
        },
    }

    /// Artifacts configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Artifacts {
        /// Paths to include as artifacts
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paths: Option<Vec<String>>,
        /// Artifact expiration duration
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expire_in: Option<String>,
        /// When to upload artifacts (on_success, on_failure, always)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub when: Option<String>,
    }

    /// Cache configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Cache {
        /// Cache key
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        /// Paths to cache
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paths: Option<Vec<String>>,
        /// When to save cache (on_success, on_failure, always)
        #[serde(skip_serializing_if = "Option::is_none")]
        pub when: Option<String>,
        /// Cache policy
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policy: Option<String>,
    }

    /// Rule for conditional job execution
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Rule {
        /// If condition expression
        #[serde(skip_serializing_if = "Option::is_none")]
        pub if_: Option<String>,
        /// When to run if condition is true
        #[serde(skip_serializing_if = "Option::is_none")]
        pub when: Option<String>,
        /// Variables to set if condition is true
        #[serde(skip_serializing_if = "Option::is_none")]
        pub variables: Option<HashMap<String, String>>,
    }

    /// Only/except configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Only {
        /// Simple list of refs
        Refs(Vec<String>),
        /// Detailed configuration
        Complex {
            /// Refs to include
            #[serde(skip_serializing_if = "Option::is_none")]
            refs: Option<Vec<String>>,
            /// Branch patterns to include
            #[serde(skip_serializing_if = "Option::is_none")]
            branches: Option<Vec<String>>,
            /// Tags to include
            #[serde(skip_serializing_if = "Option::is_none")]
            tags: Option<Vec<String>>,
            /// Pipeline types to include
            #[serde(skip_serializing_if = "Option::is_none")]
            variables: Option<Vec<String>>,
            /// Changes to files that trigger the job
            #[serde(skip_serializing_if = "Option::is_none")]
            changes: Option<Vec<String>>,
        },
    }

    /// Except configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Except {
        /// Simple list of refs
        Refs(Vec<String>),
        /// Detailed configuration
        Complex {
            /// Refs to exclude
            #[serde(skip_serializing_if = "Option::is_none")]
            refs: Option<Vec<String>>,
            /// Branch patterns to exclude
            #[serde(skip_serializing_if = "Option::is_none")]
            branches: Option<Vec<String>>,
            /// Tags to exclude
            #[serde(skip_serializing_if = "Option::is_none")]
            tags: Option<Vec<String>>,
            /// Pipeline types to exclude
            #[serde(skip_serializing_if = "Option::is_none")]
            variables: Option<Vec<String>>,
            /// Changes to files that don't trigger the job
            #[serde(skip_serializing_if = "Option::is_none")]
            changes: Option<Vec<String>>,
        },
    }

    /// Workflow configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Workflow {
        /// Rules for when to run the pipeline
        pub rules: Vec<Rule>,
    }

    /// Retry configuration
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Retry {
        /// Simple max attempts
        MaxAttempts(u32),
        /// Detailed retry configuration
        Detailed {
            /// Maximum retry attempts
            max: u32,
            /// When to retry
            #[serde(skip_serializing_if = "Option::is_none")]
            when: Option<Vec<String>>,
        },
    }

    /// Include configuration for external pipeline files
    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum Include {
        /// Simple string include
        Local(String),
        /// Detailed include configuration
        Detailed {
            /// Local file path
            #[serde(skip_serializing_if = "Option::is_none")]
            local: Option<String>,
            /// Remote file URL
            #[serde(skip_serializing_if = "Option::is_none")]
            remote: Option<String>,
            /// Include from project
            #[serde(skip_serializing_if = "Option::is_none")]
            project: Option<String>,
            /// Include specific file from project
            #[serde(skip_serializing_if = "Option::is_none")]
            file: Option<String>,
            /// Include template
            #[serde(skip_serializing_if = "Option::is_none")]
            template: Option<String>,
            /// Ref to use when including from project
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_: Option<String>,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod proptest_tests {
        use super::*;
        use proptest::prelude::*;

        /// Generate an arbitrary issue string
        fn arb_issue() -> impl Strategy<Value = String> {
            "[a-zA-Z0-9 :._-]{1,100}"
        }

        /// Generate an arbitrary list of issues
        fn arb_issues() -> impl Strategy<Value = Vec<String>> {
            proptest::collection::vec(arb_issue(), 0..10)
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(256))]

            /// ValidationResult::new() should always be valid with no issues
            #[test]
            fn prop_new_is_valid(_dummy in Just(())) {
                let result = ValidationResult::new();
                prop_assert!(result.is_valid);
                prop_assert!(result.issues.is_empty());
            }

            /// ValidationResult::default() should equal new()
            #[test]
            fn prop_default_equals_new(_dummy in Just(())) {
                let new_result = ValidationResult::new();
                let default_result = ValidationResult::default();
                prop_assert_eq!(new_result.is_valid, default_result.is_valid);
                prop_assert_eq!(new_result.issues, default_result.issues);
            }

            /// add_issue should set is_valid to false
            #[test]
            fn prop_add_issue_invalidates(issue in arb_issue()) {
                let mut result = ValidationResult::new();
                result.add_issue(issue);
                prop_assert!(!result.is_valid);
            }

            /// add_issue should add the issue to the list
            #[test]
            fn prop_add_issue_appends(issue in arb_issue()) {
                let mut result = ValidationResult::new();
                result.add_issue(issue.clone());
                prop_assert_eq!(result.issues.len(), 1);
                prop_assert_eq!(&result.issues[0], &issue);
            }

            /// Multiple add_issue calls accumulate issues
            #[test]
            fn prop_issues_accumulate(issues in arb_issues()) {
                let mut result = ValidationResult::new();
                for issue in &issues {
                    result.add_issue(issue.clone());
                }
                prop_assert_eq!(result.issues.len(), issues.len());
                prop_assert_eq!(result.issues, issues);
            }

            /// is_valid remains false after becoming false
            #[test]
            fn prop_is_valid_stays_false(issues in proptest::collection::vec(arb_issue(), 1..5)) {
                let mut result = ValidationResult::new();
                for issue in &issues {
                    result.add_issue(issue.clone());
                    prop_assert!(!result.is_valid, "is_valid should stay false");
                }
            }

            /// Empty issues means is_valid should be true (only for new instances)
            #[test]
            fn prop_empty_issues_valid(_dummy in Just(())) {
                let result = ValidationResult::new();
                prop_assert!(result.issues.is_empty());
                prop_assert!(result.is_valid);
            }
        }
    }

    mod gitlab_proptest_tests {
        use super::gitlab::*;
        use proptest::prelude::*;
        use std::collections::HashMap;

        /// Generate an arbitrary image name
        fn arb_image_name() -> impl Strategy<Value = String> {
            "[a-z][a-z0-9_/-]{1,30}(:[a-z0-9._-]{1,20})?"
        }

        /// Generate an arbitrary stage name
        fn arb_stage_name() -> impl Strategy<Value = String> {
            "(build|test|deploy|lint|release|validate)"
        }

        /// Generate an arbitrary job name
        fn arb_job_name() -> impl Strategy<Value = String> {
            "[a-z][a-z0-9_-]{1,20}"
        }

        /// Generate arbitrary script commands
        fn arb_scripts() -> impl Strategy<Value = Vec<String>> {
            proptest::collection::vec("[a-zA-Z0-9 ./|&-]{1,50}", 1..5)
        }

        /// Generate arbitrary variable map
        fn arb_variables() -> impl Strategy<Value = HashMap<String, String>> {
            proptest::collection::hash_map(
                "[A-Z][A-Z0-9_]{1,20}",
                "[a-zA-Z0-9_.-]{0,50}",
                0..5,
            )
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(256))]

            /// Image::Simple roundtrip through serde_yaml
            #[test]
            fn prop_image_simple_roundtrip(name in arb_image_name()) {
                let image = Image::Simple(name.clone());
                let yaml = serde_yaml::to_string(&image).unwrap();
                let parsed: Image = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Image::Simple(parsed_name) => prop_assert_eq!(parsed_name, name),
                    _ => prop_assert!(false, "Expected Simple variant"),
                }
            }

            /// Artifacts roundtrip through serde_yaml
            #[test]
            fn prop_artifacts_roundtrip(
                paths in proptest::option::of(proptest::collection::vec("[a-z./]{1,20}", 1..3)),
                expire_in in proptest::option::of("(1 hour|1 day|1 week|30 days)"),
                when in proptest::option::of("(on_success|on_failure|always)")
            ) {
                let artifacts = Artifacts { paths, expire_in, when };
                let yaml = serde_yaml::to_string(&artifacts).unwrap();
                let parsed: Artifacts = serde_yaml::from_str(&yaml).unwrap();
                prop_assert_eq!(parsed.paths, artifacts.paths);
                prop_assert_eq!(parsed.expire_in, artifacts.expire_in);
                prop_assert_eq!(parsed.when, artifacts.when);
            }

            /// Cache roundtrip through serde_yaml
            #[test]
            fn prop_cache_roundtrip(
                key in proptest::option::of("[a-z0-9_-]{1,20}"),
                paths in proptest::option::of(proptest::collection::vec("[a-z./]{1,20}", 1..3)),
                when in proptest::option::of("(on_success|on_failure|always)"),
                policy in proptest::option::of("(pull|push|pull-push)")
            ) {
                let cache = Cache { key, paths, when, policy };
                let yaml = serde_yaml::to_string(&cache).unwrap();
                let parsed: Cache = serde_yaml::from_str(&yaml).unwrap();
                prop_assert_eq!(parsed.key, cache.key);
                prop_assert_eq!(parsed.paths, cache.paths);
                prop_assert_eq!(parsed.when, cache.when);
                prop_assert_eq!(parsed.policy, cache.policy);
            }

            /// Retry::MaxAttempts roundtrip
            #[test]
            fn prop_retry_max_attempts_roundtrip(max in 1u32..5) {
                let retry = Retry::MaxAttempts(max);
                let yaml = serde_yaml::to_string(&retry).unwrap();
                let parsed: Retry = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Retry::MaxAttempts(parsed_max) => prop_assert_eq!(parsed_max, max),
                    _ => prop_assert!(false, "Expected MaxAttempts variant"),
                }
            }

            /// Service::Simple roundtrip
            #[test]
            fn prop_service_simple_roundtrip(name in arb_image_name()) {
                let service = Service::Simple(name.clone());
                let yaml = serde_yaml::to_string(&service).unwrap();
                let parsed: Service = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Service::Simple(parsed_name) => prop_assert_eq!(parsed_name, name),
                    _ => prop_assert!(false, "Expected Simple variant"),
                }
            }

            /// Include::Local roundtrip
            #[test]
            fn prop_include_local_roundtrip(path in "[a-z./]{1,30}\\.ya?ml") {
                let include = Include::Local(path.clone());
                let yaml = serde_yaml::to_string(&include).unwrap();
                let parsed: Include = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Include::Local(parsed_path) => prop_assert_eq!(parsed_path, path),
                    _ => prop_assert!(false, "Expected Local variant"),
                }
            }

            /// Only::Refs roundtrip
            #[test]
            fn prop_only_refs_roundtrip(refs in proptest::collection::vec("(main|master|develop|feature/.*)", 1..3)) {
                let only = Only::Refs(refs.clone());
                let yaml = serde_yaml::to_string(&only).unwrap();
                let parsed: Only = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Only::Refs(parsed_refs) => prop_assert_eq!(parsed_refs, refs),
                    _ => prop_assert!(false, "Expected Refs variant"),
                }
            }

            /// Except::Refs roundtrip
            #[test]
            fn prop_except_refs_roundtrip(refs in proptest::collection::vec("(main|master|develop|feature/.*)", 1..3)) {
                let except = Except::Refs(refs.clone());
                let yaml = serde_yaml::to_string(&except).unwrap();
                let parsed: Except = serde_yaml::from_str(&yaml).unwrap();
                match parsed {
                    Except::Refs(parsed_refs) => prop_assert_eq!(parsed_refs, refs),
                    _ => prop_assert!(false, "Expected Refs variant"),
                }
            }

            /// Rule struct roundtrip
            #[test]
            fn prop_rule_roundtrip(
                if_ in proptest::option::of("\\$[A-Z_]+ == \"[a-z]+\""),
                when in proptest::option::of("(on_success|on_failure|always|manual|delayed)")
            ) {
                let rule = Rule { if_, when, variables: None };
                let yaml = serde_yaml::to_string(&rule).unwrap();
                let parsed: Rule = serde_yaml::from_str(&yaml).unwrap();
                prop_assert_eq!(parsed.if_, rule.if_);
                prop_assert_eq!(parsed.when, rule.when);
            }

            /// Job with script roundtrip
            #[test]
            fn prop_job_with_script_roundtrip(
                stage in proptest::option::of(arb_stage_name()),
                script in proptest::option::of(arb_scripts())
            ) {
                let job = Job {
                    stage,
                    image: None,
                    script,
                    before_script: None,
                    after_script: None,
                    when: None,
                    allow_failure: None,
                    services: None,
                    tags: None,
                    variables: None,
                    dependencies: None,
                    artifacts: None,
                    cache: None,
                    rules: None,
                    only: None,
                    except: None,
                    retry: None,
                    timeout: None,
                    parallel: None,
                    template: None,
                    extends: None,
                };
                let yaml = serde_yaml::to_string(&job).unwrap();
                let parsed: Job = serde_yaml::from_str(&yaml).unwrap();
                prop_assert_eq!(parsed.stage, job.stage);
                prop_assert_eq!(parsed.script, job.script);
            }
        }
    }

    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new();
        assert!(result.is_valid);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_validation_result_add_issue() {
        let mut result = ValidationResult::new();
        result.add_issue("Test issue".to_string());

        assert!(!result.is_valid);
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.issues[0], "Test issue");
    }

    #[test]
    fn test_validation_result_multiple_issues() {
        let mut result = ValidationResult::new();
        result.add_issue("Issue 1".to_string());
        result.add_issue("Issue 2".to_string());

        assert!(!result.is_valid);
        assert_eq!(result.issues.len(), 2);
    }
}
