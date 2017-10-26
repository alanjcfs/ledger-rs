use chrono::prelude::{Date, Utc};
use status::Status;

#[derive(Debug, Clone)]
pub enum Balance {
    Price(f64),
    NoPrice,
}

#[derive(Debug, Clone)]
pub struct Account {
    name: AccountName,
}

type AccountName = String;

#[derive(Debug, Clone)]
pub struct Posting<'a> {
    transaction: &'a Transaction,
    account: &'a Account,
    amount: Amount,
}

impl<'a> Posting<'a> {
    pub fn new(transaction: &'a Transaction, account: &'a Account, amount: Amount) -> Posting<'a> {
        Posting {
            transaction: transaction,
            account: account,
            amount: amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Amount {
    commodity: CommoditySymbol,
    price: Balance,
}

impl Amount {
    pub fn new(commodity: String, price: Balance) -> Amount {
        Amount {
            commodity: commodity,
            price: price,
        }
    }
}

type CommoditySymbol = String;

impl Account {
    pub fn new(s: String) -> Account {
        Account {
            name: s,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    date: Date<Utc>,
    edate: Option<Date<Utc>>,
    status: Status,
    code: String,
    description: String,
}

impl Transaction {
    pub fn new_default() -> Transaction {
        Transaction {
            description: "".to_string(),
            date: Utc::today(),
            edate: None,
            status: Status::Unmarked,
            code: "".to_string(),
        }
    }
    pub fn new(desc: String, date: Date<Utc>) -> Transaction {
        Transaction {
            description: desc,
            date: date,
            edate: None,
            status: Status::Unmarked,
            code: "".to_string(),
        }
    }
    pub fn change_description_and_date(mut self: Transaction, s: &String, d: &Date<Utc>) -> Transaction {
        self.date = d.clone();
        self.description = s.clone();
        self
    }
}
