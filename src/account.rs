use rust_decimal::Decimal;
use serde::Serialize;
use serde::Serializer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AccountError {
    AccountLocked,
    InsufficientFunds,
    TransactionAlreadyDisputed,
    TransactionNotDisputed,
}

impl AccountError {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountError::AccountLocked => "Account is locked",
            AccountError::InsufficientFunds => "Insufficient funds",
            AccountError::TransactionAlreadyDisputed => "Transaction already disputed",
            AccountError::TransactionNotDisputed => "Transaction not disputed",
        }
    }
}

// Serialize Decimal with rounding to 4 decimal places
fn serialize_rounded<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.round_dp(4).to_string())
}

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub client: u16,
    #[serde(serialize_with = "serialize_rounded")]
    pub available: Decimal,
    #[serde(serialize_with = "serialize_rounded")]
    pub held: Decimal,
    #[serde(serialize_with = "serialize_rounded")]
    pub total: Decimal,
    pub locked: bool,
    #[serde(skip)]
    pub disputed_transactions: HashMap<u32, Decimal>,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
            locked: false,
            disputed_transactions: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, amount: Decimal) -> Result<(), AccountError> {
        if self.locked {
            return Err(AccountError::AccountLocked);
        }
        
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), AccountError> {
        if self.locked {
            return Err(AccountError::AccountLocked);
        }
        
        if self.available < amount {
            return Err(AccountError::InsufficientFunds);
        }
        
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    pub fn dispute(&mut self, mut amount: Decimal, tx_id: u32) -> Result<(), AccountError> {
        if self.locked {
            return Err(AccountError::AccountLocked);
        }
        
        if self.disputed_transactions.contains_key(&tx_id) {
            return Err(AccountError::TransactionAlreadyDisputed);
        }
        
        // Adjust amount to available if insufficient
        if self.available < amount {
            amount = self.available;
            eprintln!("Disputing transaction {} with not enough balance available, holding amount {} instead",
                      tx_id, amount);
        }
        
        self.available -= amount;
        self.held += amount;
        self.disputed_transactions.insert(tx_id, amount);
        Ok(())
    }

    pub fn resolve(&mut self, tx_id: u32) -> Result<(), AccountError> {
        if self.locked {
            return Err(AccountError::AccountLocked);
        }
        
        let amount = self.disputed_transactions.get(&tx_id)
            .ok_or(AccountError::TransactionNotDisputed)?;
        
        self.held -= amount;
        self.available += amount;
        self.disputed_transactions.remove(&tx_id);
        Ok(())
    }

    pub fn chargeback(&mut self, tx_id: u32) -> Result<(), AccountError> {
        let amount = self.disputed_transactions.get(&tx_id)
            .ok_or(AccountError::TransactionNotDisputed)?;
        
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        self.disputed_transactions.remove(&tx_id);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_account_deposit() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        assert!(account.deposit(amount).is_ok());
        assert_eq!(account.available, amount);
        assert_eq!(account.total, amount);
        assert_eq!(account.held, Decimal::ZERO);
    }

    #[test]
    fn test_account_withdrawal() {
        let mut account = Account::new(1);
        let deposit_amount = Decimal::from_str("10.0").unwrap();
        let withdraw_amount = Decimal::from_str("5.0").unwrap();
        
        account.deposit(deposit_amount).unwrap();
        assert!(account.withdraw(withdraw_amount).is_ok());
        assert_eq!(account.available, Decimal::from_str("5.0").unwrap());
        assert_eq!(account.total, Decimal::from_str("5.0").unwrap());
    }

    #[test]
    fn test_account_dispute() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        account.deposit(amount).unwrap();
        assert!(account.dispute(amount, 1).is_ok());
        assert_eq!(account.available, Decimal::ZERO);
        assert_eq!(account.held, amount);
        assert_eq!(account.total, amount);
    }

    #[test]
    fn test_account_resolve() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        account.deposit(amount).unwrap();
        account.dispute(amount, 1).unwrap();
        assert!(account.resolve(1).is_ok());
        assert_eq!(account.available, amount);
        assert_eq!(account.held, Decimal::ZERO);
        assert_eq!(account.total, amount);
    }

    #[test]
    fn test_account_chargeback() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        account.deposit(amount).unwrap();
        account.dispute(amount, 1).unwrap();
        assert!(account.chargeback(1).is_ok());
        assert_eq!(account.available, Decimal::ZERO);
        assert_eq!(account.held, Decimal::ZERO);
        assert_eq!(account.total, Decimal::ZERO);
        assert!(account.locked);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        assert!(account.withdraw(amount).is_err());
    }

    #[test]
    fn test_locked_account() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.0").unwrap();
        
        account.deposit(amount).unwrap();
        account.dispute(amount, 1).unwrap();
        account.chargeback(1).unwrap();
        
        // Account is now locked, operations should fail
        assert!(account.deposit(amount).is_err());
        assert!(account.withdraw(amount).is_err());
    }
}
