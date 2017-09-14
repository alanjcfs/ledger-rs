extern crate num;
extern crate chrono;
extern crate regex;

pub mod accounting;

use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;

use accounting::{Account,Transaction,Balance};

pub fn read(s: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(s)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse<'a>(lines: std::str::Lines<'a>, ledger: &mut Vec<Option<Transaction>>) {
    let mut trans: Option<Transaction> = None;
    let account_to_amount_space = Regex::new(r" {2,}|\t+").unwrap();

    for line in lines {
        let line_trimmed = line.trim();

        let ignored_chars = [Some(';'), Some('#'), Some('%'), Some('|'), Some('*')];

        if ignored_chars.contains(&line.chars().next()) {
            // noop
        } else if line_trimmed.len() == 0 {
            // TODO: Check transaction to make sure it balances
            if trans.is_none() == true {
                // noop
            } else {
                ledger.push(trans);
                trans = None;
            }
        } else {
            let line_split: Vec<&str> = account_to_amount_space.split(line_trimmed).collect();
            if trans.is_none() {
                trans = Some(Transaction::new_default());
                let mut date_payee = line_split[0].splitn(2, " ");
                let naive_date = chrono::NaiveDate::parse_from_str(date_payee.next().unwrap(), "%Y-%m-%d").unwrap();
                let date = chrono::Date::from_utc(naive_date, chrono::Utc);
                let payee = date_payee.next().unwrap().to_string();
                trans = change_payee_and_date(trans, &payee, &date);
            } else {
                let account: Account =
                    if line_split.len() >= 2 {
                        Account::new(line_split[0].to_string(), Balance::Amount(line_split[1].parse::<f64>().unwrap()))
                    } else {
                        Account::new(line_split[0].to_string(), Balance::Empty)
                    };
                trans = trans.map(|trans| trans.add_account(account));
            }
        }
    }
}

fn change_payee_and_date(transaction: Option<Transaction>, payee: &String, date: &chrono::Date<chrono::Utc>) -> Option<Transaction> {
    transaction.map(|transaction| transaction.change_payee_and_date(payee, date))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let read_file = super::read("examples/example.ledger").unwrap();
        let lines: Vec<&str> = read_file.lines().collect();
        assert_eq!(lines[0], "; is a comment");
        assert_eq!(lines[1], "# also a comment");
    }
}
