mod engine;
mod account;
mod transaction;
pub mod error;

use engine::TransactionEngine;
use error::EngineError;

pub fn run(input_file: &String) -> Result<(), EngineError> {
    let mut engine = TransactionEngine::new();

    engine.process_transactions_from_file(input_file)?;
    engine.output_account_balances()?;

    Ok(())
}