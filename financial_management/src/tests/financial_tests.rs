#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_transaction() {
        let mut manager = FinancialManager {
            accounts: vec![],
            transactions: vec![],
        };

        let transaction = Transaction {
            id: 1,
            amount: 100.0,
            description: "Test transaction".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        manager.add_transaction(transaction).unwrap();
        assert_eq!(manager.transactions.len(), 1);
    }

    #[test]
    fn test_get_balance() {
        let manager = FinancialManager {
            accounts: vec![Account {
                id: 1,
                name: "Test Account".to_string(),
                balance: 500.0,
            }],
            transactions: vec![],
        };

        let balance = manager.get_balance(1).unwrap();
        assert_eq!(balance, 500.0);
    }
}
