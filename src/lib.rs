extern crate num;
extern crate chrono;
extern crate regex;

pub mod accounting;

use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;

use accounting::{Account,Transaction};

pub fn read(s: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(s)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse<'a>(lines: std::str::Lines<'a>, ledger: &[Transaction]) {
    let mut line_count = 0;
    let mut trans: Option<Transaction> = None;
    let mut list_of_trans: Vec<Transaction> = vec!();
    let accountToAmountSpace = Regex::new(r" {2,}|\t+").unwrap();

    for line in lines {
        let lineTrimmed = line.trim();
        line_count += 1;

        let ignored_chars = [Some(';'), Some('#'), Some('%'), Some('|'), Some('*')];

        if ignored_chars.contains(&line.chars().next()) {
            // noop
        } else if lineTrimmed.len() == 0 {
            // TODO: Check transaction to make sure it balances
            if trans.is_none() == true {
                // noop
            } else {
                list_of_trans.push(trans.unwrap());
                trans = None;
            }
        } else {
            if trans.is_none() == true {
                trans = Some(Transaction::new_default());
            }
            let lineSplit: Vec<&str> = accountToAmountSpace.split(lineTrimmed).collect();
            if lineSplit.len() == 2 {
                let mut account: Account;
                let account = Account::new(lineSplit[0].to_string(), lineSplit[1].parse::<f64>().unwrap());
                trans = Some(trans.unwrap().add_account(account));
            } else if lineSplit.len() == 1 {
                let payee = lineSplit[0].to_string();
                trans = Some(trans.unwrap().change_payee(payee));
            }
        }
    }
    for t in list_of_trans {
        println!("{:?}", t)
    }
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
