use num::BigRational;
use chrono::prelude::{Date,Utc};

#[derive(Debug)]
pub struct Account {
    name: String,
    balance: BigRational
}

#[derive(Debug)]
pub struct Transaction {
    payee: String,
    date: Date<Utc>,
    account_changes: Box<Account>
}
