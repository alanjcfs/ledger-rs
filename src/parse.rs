extern crate chrono;

use regex::Regex;
use std;

#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};

#[allow(unused_mut, unused_variables, dead_code)]
pub fn parse<'a>(tokens: Vec<Token>) {
    let description = date_description.next().unwrap().to_string();
    trans = Some(Transaction::new(description, date));
}

// let date = chrono::Date::from_utc(naive_date, chrono::Utc);
// let naive_date =
//     chrono::NaiveDate::parse_from_str(date_description.next().unwrap(), "%Y-%m-%d")
//         .unwrap();
