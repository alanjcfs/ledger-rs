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
    let mut trans: Option<Transaction> = None;
    let mut line_count = 0;
    let accountToAmountSpace = Regex::new(r" {2,}|\t+").unwrap();

    for line in lines {
        let lineTrimmed = line.trim();
        line_count += 1;

        let ignored_chars = [Some(';'), Some('#'), Some('%'), Some('|'), Some('*')];

        if ignored_chars.contains(&line.chars().next()) {
            // noop
        } else if lineTrimmed.len() == 0 {
            println!("{:?}", trans);
        } else {
            let mut account: Account;
            let lineSplit: Vec<&str> = accountToAmountSpace.split(lineTrimmed).collect();
            println!("{:?}", lineSplit)
        }
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
