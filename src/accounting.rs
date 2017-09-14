use num::BigRational;
use chrono::prelude::{Date,Utc};

#[derive(Debug,Clone)]
pub enum Balance {
    Amount(f64),
    Empty
}

impl Balance {
    pub fn is_empty(self: Balance) -> bool {
        match self {
            Balance::Amount(_) => false,
            Balance::Empty => true
        }
    }
}

#[derive(Debug,Clone)]
pub struct Account {
    name: String,
    balance: Balance
}

impl Account {
    pub fn new(s: String, f: Balance) -> Account {
        Account { name: s, balance: f }
    }
    pub fn balance(self: &Account) -> &Balance {
        &self.balance
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Transaction {
    payee: String,
    date: Date<Utc>,
    account_changes: Vec<Account>
}

impl Transaction {
    pub fn new_default() -> Transaction {
        Transaction { payee: "".to_string(), date: Utc::today(), account_changes: vec!() }
    }
    pub fn new(p: String, d: Date<Utc>, a: Vec<Account>) -> Transaction {
        Transaction { payee: p, date: d, account_changes: a }
    }
    pub fn add_account(mut self: Transaction, a: Account) -> Transaction {
        self.account_changes.push(a);
        self
    }
    pub fn change_payee_and_date(mut self: Transaction, s: &String, d: &Date<Utc>) -> Transaction {
        self.date = d.clone();
        self.payee = s.clone();
        self
    }
    pub fn account_changes(self: &Transaction) -> &Vec<Account> {
        &self.account_changes
    }
}
