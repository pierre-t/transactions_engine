use std::process::Command;
use std::path::Path;
use std::fs;

#[test]
fn test_transactions_basic() {
    run_integration_test("basic");
}

#[test]
fn test_transactions_comprehensive() {
    run_integration_test("comprehensive");
}

fn run_integration_test(test_name: &str) {
    // Get input and expected files
    let input_file = format!("tests/data/{}.csv", test_name);
    let expected_file = format!("tests/expected/{}.expected", test_name);
    
    assert!(Path::new(&input_file).exists(), "Input file not found: {}", input_file);
    assert!(Path::new(&expected_file).exists(), "Expected file not found: {}", expected_file);
    
    // Run the binary with the test input
    let output = Command::new(env!("CARGO_BIN_EXE_transactions_engine"))
        .arg(&input_file)
        .output()
        .expect("Failed to execute binary");
    
    assert!(output.status.success(), "Binary execution failed: {}", 
            String::from_utf8_lossy(&output.stderr));

    let expected = fs::read_to_string(&expected_file)
        .expect("Failed to read expected output file");
    let actual = String::from_utf8_lossy(&output.stdout);
    
    // Normalize line endings and trim whitespace
    let expected_normalized = expected.trim().replace("\r\n", "\n");
    let actual_normalized = actual.trim().replace("\r\n", "\n");
    
    assert_eq!(actual_normalized, expected_normalized, 
        "Output mismatch for test '{}'\nExpected:\n{}\nActual:\n{}", 
        test_name, expected_normalized, actual_normalized);
}

