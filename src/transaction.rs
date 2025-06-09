use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

impl Transaction {
    pub fn requires_amount(&self) -> bool {
        matches!(self.transaction_type, TransactionType::Deposit | TransactionType::Withdrawal)
    }

    pub fn is_dispute_related(&self) -> bool {
        matches!(
            self.transaction_type,
            TransactionType::Dispute | TransactionType::Resolve | TransactionType::Chargeback
        )
    }
}

