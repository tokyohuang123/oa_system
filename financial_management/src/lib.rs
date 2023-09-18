mod comm;

pub use comm::{Account, Transaction};
use comm::FinancialService;

pub struct FinancialManager {
    // This is just a placeholder. You might want to replace this with actual storage or database.
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl FinancialService for FinancialManager {
    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.transactions.push(transaction);
        Ok(())
    }

    fn get_balance(&self, account_id: u32) -> Result<f64, String> {
        // Placeholder logic to get balance. You might want to implement actual logic.
        let balance = self.accounts.iter().find(|&acc| acc.id == account_id).map(|acc| acc.balance).unwrap_or(0.0);
        Ok(balance)
    }
}






pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
