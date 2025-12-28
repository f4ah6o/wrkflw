//! Demonstrates that duplicate values in matrix parameter arrays are deduplicated.
//!
//! Run with: cargo run -p wrkflw-matrix --example deduplicate_demo

use indexmap::IndexMap;
use serde_yaml::Value;
use wrkflw_matrix::{expand_matrix, format_combination_name, MatrixConfig};

fn main() {
    println!("=== Matrix Deduplication Demo ===\n");

    // Example 1: Duplicate OS values
    println!("Example 1: Duplicate values in 'os' parameter");
    println!("Input: os: [ubuntu, ubuntu, windows, ubuntu]");
    println!("       node: [18, 20]");

    let mut params1 = IndexMap::new();
    params1.insert(
        "os".to_string(),
        Value::Sequence(vec![
            Value::String("ubuntu".to_string()),
            Value::String("ubuntu".to_string()), // duplicate
            Value::String("windows".to_string()),
            Value::String("ubuntu".to_string()), // duplicate
        ]),
    );
    params1.insert(
        "node".to_string(),
        Value::Sequence(vec![
            Value::Number(18.into()),
            Value::Number(20.into()),
        ]),
    );

    let matrix1 = MatrixConfig {
        parameters: params1,
        include: vec![],
        exclude: vec![],
        max_parallel: None,
        fail_fast: Some(true),
    };

    match expand_matrix(&matrix1) {
        Ok(combinations) => {
            println!("\nResult: {} combinations (duplicates removed)\n", combinations.len());
            for combo in &combinations {
                println!("  - {}", format_combination_name("build", combo));
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Multiple parameters with duplicates
    println!("\n---\n");
    println!("Example 2: Multiple parameters with duplicates");
    println!("Input: browser: [chrome, firefox, chrome]");
    println!("       env: [dev, prod, dev, staging]");

    let mut params2 = IndexMap::new();
    params2.insert(
        "browser".to_string(),
        Value::Sequence(vec![
            Value::String("chrome".to_string()),
            Value::String("firefox".to_string()),
            Value::String("chrome".to_string()), // duplicate
        ]),
    );
    params2.insert(
        "env".to_string(),
        Value::Sequence(vec![
            Value::String("dev".to_string()),
            Value::String("prod".to_string()),
            Value::String("dev".to_string()),     // duplicate
            Value::String("staging".to_string()),
        ]),
    );

    let matrix2 = MatrixConfig {
        parameters: params2,
        include: vec![],
        exclude: vec![],
        max_parallel: None,
        fail_fast: Some(true),
    };

    match expand_matrix(&matrix2) {
        Ok(combinations) => {
            println!("\nResult: {} combinations", combinations.len());
            println!("Expected: 2 browsers x 3 envs = 6 combinations\n");
            for combo in &combinations {
                println!("  - {}", format_combination_name("test", combo));
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 3: Order preservation
    println!("\n---\n");
    println!("Example 3: Order preservation");
    println!("Input: version: [3, 1, 2, 1, 3]");

    let mut params3 = IndexMap::new();
    params3.insert(
        "version".to_string(),
        Value::Sequence(vec![
            Value::Number(3.into()),
            Value::Number(1.into()),
            Value::Number(2.into()),
            Value::Number(1.into()), // duplicate
            Value::Number(3.into()), // duplicate
        ]),
    );

    let matrix3 = MatrixConfig {
        parameters: params3,
        include: vec![],
        exclude: vec![],
        max_parallel: None,
        fail_fast: Some(true),
    };

    match expand_matrix(&matrix3) {
        Ok(combinations) => {
            println!("\nResult: {} combinations (order: 3, 1, 2)\n", combinations.len());
            for combo in &combinations {
                println!("  - {}", format_combination_name("run", combo));
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Demo Complete ===");
}
