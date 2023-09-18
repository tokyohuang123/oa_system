pub trait FinancialService {
    /// Adds a new transaction.
    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String>;

    /// Gets the balance of an account.
    fn get_balance(&self, account_id: u32) -> Result<f64, String>;
}
