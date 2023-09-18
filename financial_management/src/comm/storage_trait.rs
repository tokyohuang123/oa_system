pub trait FinancialStorage {
    /// Saves a transaction to storage.
    fn save_transaction(&mut self, transaction: &Transaction) -> Result<(), String>;

    /// Retrieves a transaction from storage by its ID.
    fn get_transaction(&self, transaction_id: u32) -> Result<Option<Transaction>, String>;

    /// Saves an account to storage.
    fn save_account(&mut self, account: &Account) -> Result<(), String>;

    /// Retrieves an account from storage by its ID.
    fn get_account(&self, account_id: u32) -> Result<Option<Account>, String>;
}
