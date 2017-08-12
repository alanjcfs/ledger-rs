use num::BigRational;
use chrono::prelude::{Date,Utc};

pub struct Account {
    name: String,
    balance: BigRational
}

pub struct Transaction {
    payee: String,
    date: Date<Utc>,
    account_changes: Box<Account>
}
