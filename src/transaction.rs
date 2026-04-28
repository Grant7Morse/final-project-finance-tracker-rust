use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionKind {
    Income,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub kind: TransactionKind,
    pub amount: f64,
    pub category: String,
    pub note: String,
}

impl Transaction {
    pub fn new(
        kind: TransactionKind,
        amount: f64,
        category: String,
        note: String,
    ) -> Result<Self, String> {
        if amount <= 0.0 {
            return Err(String::from("Amount must be greater than zero"));
        }

        if category.trim().is_empty() {
            return Err(String::from("Category cannot be empty"));
        }

        Ok(Transaction {
            kind,
            amount,
            category,
            note,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_income_transaction() {
        let transaction = Transaction::new(
            TransactionKind::Income,
            100.0,
            String::from("work"),
            String::from("paycheck"),
        );

        assert!(transaction.is_ok());
    }

    #[test]
    fn rejects_negative_amount() {
        let transaction = Transaction::new(
            TransactionKind::Expense,
            -10.0,
            String::from("food"),
            String::from("bad amount"),
        );

        assert!(transaction.is_err());
    }

    #[test]
    fn rejects_empty_category() {
        let transaction = Transaction::new(
            TransactionKind::Expense,
            10.0,
            String::from(""),
            String::from("missing category"),
        );

        assert!(transaction.is_err());
    }
}
