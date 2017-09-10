use num::BigRational;
use chrono::prelude::{Date,Utc};

#[derive(Debug,Clone)]
pub struct Account {
    name: String,
    balance: f64
}

impl Account {
    pub fn new(s: String, f: f64) -> Account {
        Account { name: s, balance: f }
    }
    pub fn balance(self: &Account) -> f64 {
        self.balance
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
    pub fn account_sum(self: &Transaction) -> f64 {
        self.account_changes.iter().fold(0., |acc, ref item| acc + item.balance())
    }
    pub fn change_payee(mut self: Transaction, s: String) -> Transaction {
        self.payee = s;
        self
    }
    pub fn set_date(mut self: Transaction, d: Date<Utc>) -> Transaction {
        self.date = d;
        self
    }
}
