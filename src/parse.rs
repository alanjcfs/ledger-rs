extern crate chrono;

use regex::Regex;
use std;

#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};

#[allow(unused_mut, unused_variables, dead_code)]
pub fn parse<'a>(lines: std::str::Lines<'a>, ledger: &mut Vec<Option<Transaction>>, mut postings: &Vec<Posting>) {
    let mut trans: Option<Transaction> = None;
    let account_to_amount_space = Regex::new(r" {2,}|\t+").unwrap();
    let _unwrapped_transaction: Transaction;
    let _account: Account;

    for line in lines {
        let line_trimmed = line.trim();

        let ignored_chars = [Some(';'), Some('#'), Some('%'), Some('|'), Some('*')];

        if ignored_chars.contains(&line.chars().next()) {
            // noop
        } else if line_trimmed.len() == 0 {
            if trans.is_none() == true {
                // noop
            } else {
                ledger.push(trans);
                trans = None;
            }
        } else {
            let line_split: Vec<&str> = account_to_amount_space.split(line_trimmed).collect();
            if trans.is_none() {
                let mut date_description = line_split[0].splitn(2, " ");
                let naive_date =
                    chrono::NaiveDate::parse_from_str(date_description.next().unwrap(), "%Y-%m-%d")
                        .unwrap();
                let date = chrono::Date::from_utc(naive_date, chrono::Utc);
                let description = date_description.next().unwrap().to_string();
                trans = Some(Transaction::new(description, date));
            } else {
                // unwrapped_transaction = trans.clone().unwrap();
                // account = Account::new(line_split[0].to_string());
                // let posting: Posting =
                //     if line_split.len() >= 2 {
                //         Posting::new(
                //             &unwrapped_transaction,
                //             &account,
                //             Amount::new("$".to_string(), Balance::Price(line_split[1].parse::<f64>().unwrap())),
                //         )
                //     } else {
                //         Posting::new(
                //             &unwrapped_transaction,
                //             &account,
                //             Amount::new("$".to_string(), Balance::NoPrice),
                //         )
                //     };
                // postings.push(posting);
            }
        }
    }
}

#[allow(dead_code)]
fn change_description_and_date(
    transaction: Option<Transaction>,
    description: &String,
    date: &chrono::Date<chrono::Utc>,
) -> Option<Transaction> {
    transaction.map(|transaction| transaction.change_description_and_date(description, date))
}
