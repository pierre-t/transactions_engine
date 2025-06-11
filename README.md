# Financial Transactions Engine

A Rust-based financial transactions processing engine that handles deposits, withdrawals, disputes, resolutions, and chargebacks with precise decimal arithmetic.

## Features

- **Transaction Types**: deposit, withdrawal, dispute, resolve, chargeback
- **Account Management**: Tracks available, held, total balances and locked status
- **Precise Arithmetic**: Uses `rust_decimal` for exact financial calculations (4 decimal places)
- **CSV Input/Output**: Reads transactions from a CSV file, outputs account balances to `stdout` in a CSV format
- **Error Handling**: Fails completely on invalid CSV file, ignores invalid transactions

## Usage

```bash
cargo run <input.csv>
```

### Input Format (CSV)
The input CSV must have the following columns:
- `type`: Transaction type (deposit, withdrawal, dispute, resolve, chargeback)
- `client`: Client ID (u16)
- `tx`: Transaction ID (u32)
- `amount`: Transaction amount (only for deposit/withdrawal)

Example:
```csv
type,client,tx,amount
deposit,1,1,1.0
deposit,2,2,2.0
deposit,1,3,2.0
withdrawal,1,4,1.5
```

### Output Format (CSV)
Outputs account balances with columns:
- `client`: Client ID
- `available`: Available balance
- `held`: Held balance (disputed funds)
- `total`: Total balance (available + held)
- `locked`: Account locked status

Example:
```csv
client,available,held,total,locked
1,1.5,0,1.5,false
2,2,0,2,false
```

## Transaction Rules

### Deposits
- Increase available and total balance
- Must have positive amount
- Cannot process if account is locked

### Withdrawals
- Decrease available and total balance
- Must have sufficient available funds
- Must have positive amount
- Cannot process if account is locked

### Disputes
- Move funds from available to held
- Can only dispute deposit transactions
- Client must match original transaction
- Cannot dispute already disputed transactions
- Cannot process if account is locked

### Resolves
- Move funds from held back to available
- Can only resolve disputed transactions
- Client must match original transaction
- Cannot process if account is locked

### Chargebacks
- Remove disputed funds from total balance
- Lock the account permanently
- Can only chargeback disputed transactions
- Client must match original transaction

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Dependencies

- `serde`: Serialization/deserialization
- `csv`: CSV file processing
- `rust_decimal`: Precise decimal arithmetic for financial calculations

## Examples

Run with the provided test file:
```bash
cargo run -- example.csv
```

## Error Handling

The engine fails completely on an invalid CSV file or if a transaction row doesn't have the required information, such as:
- Invalid transaction types
- Missing amounts for deposits/withdrawals
- Amounts provided for dispute-related transactions
- Negative amounts

The engine will ignore correctly formed transactions that are invalid, such as:
- Insufficient funds for withdrawals
- Duplicate transaction IDs (for deposits/withdrawals)
- Operations on locked accounts (except chargebacks)
- Invalid dispute operations (wrong client, non-existent transactions, etc.)

## Architecture

- `main.rs`: CLI entry point
- `lib.rs`: Library interface that will call the engine
- `transaction.rs`: Transaction type
- `account.rs`: Account management and balance operations
- `engine.rs`: Main transaction processing engine
- `engine_error.rs`: Engine error type
