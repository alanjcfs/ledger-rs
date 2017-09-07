use num::BigRational;
use chrono::prelude::{Date,Utc};

#[derive(Debug)]
pub struct Account {
    name: String,
    balance: f64
}

impl Account {
    pub fn new(s: String, f: f64) -> Account {
        Account { name: s, balance: f }
    }
}

#[derive(Debug)]
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
    pub fn change_payee(mut self: Transaction, s: String) -> Transaction {
        self.payee = s;
        self
    }
}
