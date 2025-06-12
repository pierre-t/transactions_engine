use std::env;
use std::process;
use std::fs::File;

mod engine;
mod account;
mod transaction;
mod engine_error;

use engine::TransactionEngine;
use engine_error::EngineError;


fn main() -> Result<(), EngineError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <input.csv>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    run(input_file)
}

fn run(input_file: &String) -> Result<(), EngineError> {
    let mut engine = TransactionEngine::new();

    let file = File::open(input_file)?;
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    engine.process_transactions_from_reader(&mut rdr)?;

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    engine.output_account_balances_to_writer(&mut wtr)?;

    Ok(())
}
