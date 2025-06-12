use crate::account::Account;
use crate::engine_error::EngineError;
use crate::transaction::{Transaction, TransactionType};
use csv::{Reader, Writer};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct TransactionEngine {
    accounts: HashMap<u16, Account>,
    transaction_history: HashMap<u32, Transaction>,
}

impl TransactionEngine {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transaction_history: HashMap::new(),
        }
    }

    pub fn process_transactions_from_reader<R: Read>(&mut self, reader: &mut Reader<R>) -> Result<(), EngineError> {
        for result in reader.deserialize() {
            let transaction: Transaction = result?;
            self.process_transaction(transaction)?;
        }

        Ok(())
    }

    fn process_transaction(&mut self, transaction: Transaction) -> Result<(), EngineError> {
        // Validate transaction
        self.validate_transaction(&transaction)?;

        let res = match transaction.transaction_type {
            TransactionType::Deposit => self.process_deposit(&transaction),
            TransactionType::Withdrawal => self.process_withdrawal(&transaction),
            TransactionType::Dispute => self.process_dispute(&transaction),
            TransactionType::Resolve => self.process_resolve(&transaction),
            TransactionType::Chargeback => self.process_chargeback(&transaction),
        };

        if let Err(e) = res {
            // Log the error but continue processing other transactions
            eprintln!("Ignoring error while processing transaction {}: {}", transaction.tx, e);
        }
        Ok(())
    }

    fn validate_transaction(&self, transaction: &Transaction) -> Result<(), EngineError> {
        // Check if transaction requires amount but doesn't have one
        if transaction.requires_amount() && transaction.amount.is_none() {
            return Err(EngineError::InvalidTransaction(
                "Deposit and withdrawal transactions must have an amount".to_string(),
            ));
        }

        // Check if dispute-related transaction has an amount (it shouldn't)
        if transaction.is_dispute_related() && transaction.amount.is_some() {
            return Err(EngineError::InvalidTransaction(
                "Dispute, resolve, and chargeback transactions should not have an amount".to_string(),
            ));
        }

        // Check for negative amounts
        if let Some(amount) = transaction.amount {
            if amount <= Decimal::ZERO {
                return Err(EngineError::InvalidTransaction(
                    "Transaction amount must be positive".to_string(),
                ));
            }
        }

        // Check for duplicate transaction IDs for deposit/withdrawal
        if matches!(transaction.transaction_type, TransactionType::Deposit | TransactionType::Withdrawal) {
            if self.transaction_history.contains_key(&transaction.tx) {
                return Err(EngineError::InvalidTransaction(
                    format!("Duplicate transaction ID: {}", transaction.tx),
                ));
            }
        }

        Ok(())
    }

    fn process_deposit(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        let amount = transaction.amount.unwrap(); // Safe because we validated
        let account = self.accounts.entry(transaction.client).or_insert_with(|| Account::new(transaction.client));
        
        account.deposit(amount)?;
        
        // Store transaction for potential disputes
        self.transaction_history.insert(transaction.tx, transaction.clone());
        Ok(())
    }

    fn process_withdrawal(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        let amount = transaction.amount.unwrap(); // Safe because we validated
        let account = self.accounts.entry(transaction.client).or_insert_with(|| Account::new(transaction.client));
        
        account.withdraw(amount)?;
        
        // Store transaction for potential disputes
        self.transaction_history.insert(transaction.tx, transaction.clone());
        Ok(())
    }

    fn process_dispute(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        // Find the original transaction
        let original_transaction = self.transaction_history.get(&transaction.tx)
            .ok_or_else(|| EngineError::InvalidTransaction(
                format!("Cannot dispute non-existent transaction: {}", transaction.tx)
            ))?;

        // Verify client matches
        if original_transaction.client != transaction.client {
            return Err(EngineError::InvalidTransaction(
                "Cannot dispute transaction from different client".to_string(),
            ));
        }

        // Only deposits can be disputed
        if !matches!(original_transaction.transaction_type, TransactionType::Deposit) {
            return Err(EngineError::InvalidTransaction(
                "Only deposit transactions can be disputed".to_string(),
            ));
        }

        let amount = original_transaction.amount.unwrap();
        let account = self.accounts.get_mut(&transaction.client)
            .ok_or_else(|| EngineError::AccountError("Account not found".to_string()))?;

        account.dispute(amount, transaction.tx)?;
        Ok(())
    }

    fn process_resolve(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        // Find the original transaction
        let original_transaction = self.transaction_history.get(&transaction.tx)
            .ok_or_else(|| EngineError::InvalidTransaction(
                format!("Cannot resolve non-existent transaction: {}", transaction.tx)
            ))?;

        // Verify client matches
        if original_transaction.client != transaction.client {
            return Err(EngineError::InvalidTransaction(
                "Cannot resolve transaction from different client".to_string(),
            ));
        }

        let account = self.accounts.get_mut(&transaction.client)
            .ok_or_else(|| EngineError::AccountError("Account not found".to_string()))?;

        account.resolve(transaction.tx)?;
        Ok(())
    }

    fn process_chargeback(&mut self, transaction: &Transaction) -> Result<(), EngineError> {
        // Find the original transaction
        let original_transaction = self.transaction_history.get(&transaction.tx)
            .ok_or_else(|| EngineError::InvalidTransaction(
                format!("Cannot chargeback non-existent transaction: {}", transaction.tx)
            ))?;

        // Verify client matches
        if original_transaction.client != transaction.client {
            return Err(EngineError::InvalidTransaction(
                "Cannot chargeback transaction from different client".to_string(),
            ));
        }

        let account = self.accounts.get_mut(&transaction.client)
            .ok_or_else(|| EngineError::AccountError("Account not found".to_string()))?;

        account.chargeback(transaction.tx)?;
        Ok(())
    }

    pub fn output_account_balances_to_writer<W: Write>(&mut self, writer: &mut Writer<W>) -> Result<(), EngineError> {
        // Sort accounts by client ID for consistent output
        let mut sorted_accounts: Vec<_> = self.accounts.values().collect();
        sorted_accounts.sort_by_key(|account| account.client);
        
        for account in sorted_accounts {
            writer.serialize(account)?;
        }
        
        writer.flush()?;
        Ok(())
    }
}

