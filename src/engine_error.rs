use std::fmt;
use crate::account::AccountError;

#[derive(Debug)]
pub enum EngineError {
    IoError(std::io::Error),
    CsvError(csv::Error),
    InvalidTransaction(String),
    AccountError(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::IoError(err) => write!(f, "IO error: {}", err),
            EngineError::CsvError(err) => write!(f, "CSV error: {}", err),
            EngineError::InvalidTransaction(msg) => write!(f, "Invalid transaction: {}", msg),
            EngineError::AccountError(msg) => write!(f, "Account error: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::IoError(err)
    }
}

impl From<csv::Error> for EngineError {
    fn from(err: csv::Error) -> Self {
        EngineError::CsvError(err)
    }
}

impl From<AccountError> for EngineError {
    fn from(err: AccountError) -> Self {
        EngineError::AccountError(err.as_str().to_string())
    }
}
