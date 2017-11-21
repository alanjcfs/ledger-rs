use chrono::prelude::{Date, Utc};
use status::Status;

pub type AccountName = String;
pub type CommoditySymbol = String;

#[derive(Debug, Clone)]
pub struct Account {
    name: AccountName,
}

impl Account {
    pub fn new(s: String) -> Account {
        Account {
            name: s,
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct Posting {
    transaction: Transaction,
    account: Account,
    amount: Option<Amount>,
}

impl Posting {
    pub fn new(transaction: Transaction, account: Account, amount: Option<Amount>) -> Posting {
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
    price: i64,
}

impl Amount {
    pub fn new(commodity: CommoditySymbol, price: f64) -> Amount {
        Amount {
            commodity: commodity,
            price: (price * 100.) as i64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    id: usize,
    date: Date<Utc>,
    edate: Option<Date<Utc>>,
    status: Status,
    description: String,
}

impl Transaction {
    pub fn new(id: usize, date: Date<Utc>, status: Status, desc: String) -> Transaction {
        Transaction {
            id: id,
            date: date,
            edate: None,
            status: status,
            description: desc,
        }
    }
}
