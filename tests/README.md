# Integration Tests

This directory contains integration tests for the transactions engine binary.

## Structure

```
tests/
├── README.md                    # This file
├── integration_test.rs          # Rust integration test file
├── data/                        # Test input CSV files
│   ├── basic.csv
│   ├── comprehensive.csv
│   └── ...
└── expected/                    # Expected output files
    ├── basic.expected
    ├── comprehensive.expected
    └── ...
```

## How it works

The integration tests work by:

1. Running the transactions_engine binary with CSV input files from `tests/data/`
2. Capturing the output and comparing it to expected results in `tests/expected/`
3. Failing if the actual output doesn't match the expected output

## Running the tests

To run all tests (including integration tests):

```bash
cargo test
```

To run only the integration tests:

```bash
cargo test --test integration_test
```

To run a specific integration test:

```bash
cargo test test_transactions_basic
cargo test test_transactions_comprehensive
```

## Adding new tests

To add a new integration test:

1. Add your CSV input file to `tests/data/your_test_name.csv`
2. Add the corresponding expected output file `tests/expected/your_test_name.expected`
3. Add a new test function to `integration_test.rs`:
   ```rust
   #[test]
   fn test_your_test_name() {
       run_integration_test("your_test_name");
   }
   ```
