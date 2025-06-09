use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
    #[serde(skip)]
    pub disputed_transactions: HashSet<u32>,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
            locked: false,
            disputed_transactions: HashSet::new(),
        }
    }

    pub fn deposit(&mut self, amount: Decimal) -> Result<(), &'static str> {
        if self.locked {
            return Err("Account is locked");
        }
        
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), &'static str> {
        if self.locked {
            return Err("Account is locked");
        }
        
        if self.available < amount {
            return Err("Insufficient funds");
        }
        
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    pub fn dispute(&mut self, amount: Decimal, tx_id: u32) -> Result<(), &'static str> {
        if self.locked {
            return Err("Account is locked");
        }
        
        if self.available < amount {
            return Err("Insufficient available funds to dispute");
        }
        
        if self.disputed_transactions.contains(&tx_id) {
            return Err("Transaction already disputed");
        }
        
        self.available -= amount;
        self.held += amount;
        self.disputed_transactions.insert(tx_id);
        Ok(())
    }

    pub fn resolve(&mut self, amount: Decimal, tx_id: u32) -> Result<(), &'static str> {
        if self.locked {
            return Err("Account is locked");
        }
        
        if !self.disputed_transactions.contains(&tx_id) {
            return Err("Transaction not disputed");
        }
        
        if self.held < amount {
            return Err("Insufficient held funds");
        }
        
        self.held -= amount;
        self.available += amount;
        self.disputed_transactions.remove(&tx_id);
        Ok(())
    }

    pub fn chargeback(&mut self, amount: Decimal, tx_id: u32) -> Result<(), &'static str> {
        if !self.disputed_transactions.contains(&tx_id) {
            return Err("Transaction not disputed");
        }
        
        if self.held < amount {
            return Err("Insufficient held funds");
        }
        
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        self.disputed_transactions.remove(&tx_id);
        Ok(())
    }

    pub fn round_to_four_decimals(&mut self) {
        self.available = self.available.round_dp(4);
        self.held = self.held.round_dp(4);
        self.total = self.total.round_dp(4);
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
        assert!(account.resolve(amount, 1).is_ok());
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
        assert!(account.chargeback(amount, 1).is_ok());
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
        account.chargeback(amount, 1).unwrap();
        
        // Account is now locked, operations should fail
        assert!(account.deposit(amount).is_err());
        assert!(account.withdraw(amount).is_err());
    }

    #[test]
    fn test_precision() {
        let mut account = Account::new(1);
        let amount = Decimal::from_str("10.12345").unwrap();
        
        account.deposit(amount).unwrap();
        account.round_to_four_decimals();
        assert_eq!(account.available, Decimal::from_str("10.1234").unwrap());
    }
}
