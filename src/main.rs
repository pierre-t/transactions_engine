use std::env;
use std::process;

use transactions_engine::error::EngineError;

fn main() -> Result<(), EngineError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <input.csv>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    transactions_engine::run(input_file)
}
