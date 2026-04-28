use crate::transaction::{Transaction, TransactionKind};

pub fn total_income(transactions: &[Transaction]) -> f64 {
    transactions
        .iter()
        .filter(|t| t.kind == TransactionKind::Income)
        .map(|t| t.amount)
        .sum()
}

pub fn total_expenses(transactions: &[Transaction]) -> f64 {
    transactions
        .iter()
        .filter(|t| t.kind == TransactionKind::Expense)
        .map(|t| t.amount)
        .sum()
}

pub fn balance(transactions: &[Transaction]) -> f64 {
    total_income(transactions) - total_expenses(transactions)
}

pub fn filter_by_category<'a>(
    transactions: &'a [Transaction],
    category: &str,
) -> Vec<&'a Transaction> {
    transactions
        .iter()
        .filter(|t| t.category.eq_ignore_ascii_case(category))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{Transaction, TransactionKind};

    fn sample_data() -> Vec<Transaction> {
        vec![
            Transaction::new(
                TransactionKind::Income,
                500.0,
                "work".into(),
                "paycheck".into(),
            )
            .unwrap(),
            Transaction::new(
                TransactionKind::Expense,
                50.0,
                "food".into(),
                "lunch".into(),
            )
            .unwrap(),
            Transaction::new(
                TransactionKind::Expense,
                20.0,
                "food".into(),
                "snack".into(),
            )
            .unwrap(),
        ]
    }

    #[test]
    fn calculates_total_income() {
        let data = sample_data();
        assert_eq!(total_income(&data), 500.0);
    }

    #[test]
    fn calculates_total_expenses() {
        let data = sample_data();
        assert_eq!(total_expenses(&data), 70.0);
    }

    #[test]
    fn calculates_balance() {
        let data = sample_data();
        assert_eq!(balance(&data), 430.0);
    }

    #[test]
    fn filters_by_category() {
        let data = sample_data();
        let filtered = filter_by_category(&data, "food");
        assert_eq!(filtered.len(), 2);
    }
}
