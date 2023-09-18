#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: u32,
    amount: f64,
    description: String,
    timestamp: std::time::SystemTime,
    transaction_type: TransactionType, // 例如：存款、取款、转账等
    currency: String, // 例如：USD, CNY, EUR等
    recipient_account_id: Option<u32>, // 如果是转账，则可能需要接收方的账户ID
    category: String, // 例如：食品、交通、娱乐等
    status: TransactionStatus, // 例如：已完成、待处理、失败等
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Completed,
    Pending,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    id: u32,
    name: String,
    balance: f64,
    account_type: AccountType, // 例如：储蓄、支票、信用卡等
    currency: String, // 例如：USD, CNY, EUR等
    interest_rate: Option<f64>, // 如果是储蓄账户，则可能有利率
    credit_limit: Option<f64>, // 如果是信用卡账户，则可能有信用额度
    status: AccountStatus, // 例如：活跃、冻结、关闭等
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccountType {
    Savings,
    Checking,
    CreditCard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

impl Transaction {
    // Getters for Transaction
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_timestamp(&self) -> &std::time::SystemTime {
        &self.timestamp
    }

    pub fn get_transaction_type(&self) -> &TransactionType {
        &self.transaction_type
    }

    pub fn get_currency(&self) -> &String {
        &self.currency
    }

    pub fn get_recipient_account_id(&self) -> &Option<u32> {
        &self.recipient_account_id
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn get_status(&self) -> &TransactionStatus {
        &self.status
    }

    // Setters for Transaction
    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_timestamp(&mut self, timestamp: std::time::SystemTime) {
        self.timestamp = timestamp;
    }

    pub fn set_transaction_type(&mut self, transaction_type: TransactionType) {
        self.transaction_type = transaction_type;
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_recipient_account_id(&mut self, recipient_account_id: Option<u32>) {
        self.recipient_account_id = recipient_account_id;
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
}

impl Account {
    // Getters for Account
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_account_type(&self) -> &AccountType {
        &self.account_type
    }

    pub fn get_currency(&self) -> &String {
        &self.currency
    }

    pub fn get_interest_rate(&self) -> &Option<f64> {
        &self.interest_rate
    }

    pub fn get_credit_limit(&self) -> &Option<f64> {
        &self.credit_limit
    }

    pub fn get_status(&self) -> &AccountStatus {
        &self.status
    }

    // Setters for Account
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }

    pub fn set_account_type(&mut self, account_type: AccountType) {
        self.account_type = account_type;
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn set_interest_rate(&mut self, interest_rate: Option<f64>) {
        self.interest_rate = interest_rate;
    }

    pub fn set_credit_limit(&mut self, credit_limit: Option<f64>) {
        self.credit_limit = credit_limit;
    }

    pub fn set_status(&mut self, status: AccountStatus) {
        self.status = status;
    }
}
