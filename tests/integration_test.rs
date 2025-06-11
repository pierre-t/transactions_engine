use std::process::Command;
use std::path::Path;
use std::fs;

#[test]
fn test_transactions_basic() {
    run_success_test("basic");
}

#[test]
fn test_transactions_comprehensive() {
    run_success_test("comprehensive");
    run_success_test("repeated_dispute");
    run_success_test("repeated_chargeback");
    run_success_test("dispute_after_withdrawal");
}

#[test]
fn test_transactions_failed() {
    run_success_test("withdrawal_fail");
    run_success_test("deposit_fail");
    run_success_test("dispute_fail");
    run_success_test("resolve_fail");
    run_success_test("chargeback_fail");
    run_success_test("account_locked");
}

#[test]
fn test_malformed_csv_error() {
    run_error_test("malformed");
    run_error_test("malformed_type");
    run_error_test("malformed_type_nonexistent");
    run_error_test("malformed_client");
    run_error_test("malformed_client_overflow");
    run_error_test("malformed_tx");
    run_error_test("malformed_amount");
}

#[test]
fn test_invalid_tx_error() {
    run_error_test("invalid_no_amount");
    run_error_test("invalid_unexpected_amount");
    run_error_test("invalid_negative_amount");
    run_error_test("invalid_duplicate_id");
}

fn run_success_test(test_name: &str) {
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

fn run_error_test(test_name: &str) {
    let input_file = format!("tests/data/{}.csv", test_name);

    assert!(Path::new(&input_file).exists(), "Input file not found: {}", input_file);

    // Run the binary with the test input
    let output = Command::new(env!("CARGO_BIN_EXE_transactions_engine"))
        .arg(&input_file)
        .output()
        .expect("Failed to execute binary");
    
    // Assert that the binary failed (non-zero exit code)
    assert!(!output.status.success(), 
        "Expected binary to fail for malformed input '{}', but it succeeded. Output: {}",
        test_name, String::from_utf8_lossy(&output.stdout));

    // Optionally check that stderr contains error information
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.is_empty(), 
        "Expected error output for malformed input '{}', but stderr was empty",
        test_name);
}

