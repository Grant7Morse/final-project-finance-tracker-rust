use crate::transaction::Transaction;
use std::fs;
use std::path::Path;

pub fn load_transactions(file_path: &str) -> Result<Vec<Transaction>, String> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }

    let contents =
        fs::read_to_string(file_path).map_err(|error| format!("Could not read file: {}", error))?;

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&contents).map_err(|error| format!("Could not parse JSON: {}", error))
}

pub fn save_transactions(file_path: &str, transactions: &[Transaction]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(transactions)
        .map_err(|error| format!("Could not serialize transactions: {}", error))?;

    fs::write(file_path, json).map_err(|error| format!("Could not write file: {}", error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{Transaction, TransactionKind};

    #[test]
    fn loads_empty_when_file_does_not_exist() {
        let transactions = load_transactions("test_missing_file.json").unwrap();
        assert!(transactions.is_empty());
    }

    #[test]
    fn saves_and_loads_transactions() {
        let file_path = "test_transactions.json";

        let transactions = vec![
            Transaction::new(
                TransactionKind::Expense,
                12.5,
                "food".into(),
                "lunch".into(),
            )
            .unwrap(),
        ];

        save_transactions(file_path, &transactions).unwrap();
        let loaded = load_transactions(file_path).unwrap();

        fs::remove_file(file_path).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].category, "food");
        assert_eq!(loaded[0].amount, 12.5);
    }
}
