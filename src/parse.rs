extern crate chrono;

use regex::Regex;
use std;

#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};
use lexer::Token;

#[derive(PartialEq)]
enum State {
    Nothing,
    Date,
    Account
}

pub fn parse<'a>(tokens: Vec<Token>) {
    // Now to confabulate these broken pieces into a story of money.
    //
    // All the ledger's a stage, and the accounts in it merely players;
    // They have their exits (or debits) and their entrances (or credits).
    // And money in transaction plays two acts, the first its addition,
    // The second its complete negation. In keeping of the book,
    // An entry of a moment and many postings may be made
    // Until the balance of both creed and debt has been recorded
    // And the accounts are settled.
    //

    // Perhaps even at the parser level, we can only construct an abstract syntax tree?
    let mut state: State = State::Nothing;
    let line_number: usize = 1;

    for token in tokens {
        match token {
            Token::Separator => {
                if state == State::Nothing {
                    panic!("Unexpected whitespace at beginning of line: line {}", line_number)
                }
            }
            Token::Space => {}
            Token::Word(_string) => {}
            Token::Date(_string) => {}
            Token::Money(_amount) => {}
            Token::Currency(_string) => {}
            Token::Symbol(_string) => {}
            Token::Newline(_line) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_panic_on_newline_with_indentation() {
        parse(vec![Token::Separator]);
    }
}

// let date = chrono::Date::from_utc(naive_date, chrono::Utc);
// let naive_date =
//     chrono::NaiveDate::parse_from_str(date_description.next().unwrap(), "%Y-%m-%d")
//         .unwrap();


/*
 * # TRANSACTIONS
 *
 * Contains only date, edate, clear status, code, description
 *
 * # POSTINGS
 *
 * Linked to transaction and account (a single record per account seems the simplest
 * implementation, more like how a database would handle it, than trying to store multiple accounts
 * in an array. It's also probably the easiest way to export and import into SQLite)
 *
 *
 * # ACCOUNTS
 *
 * Contains amounts?
 * Has a name, which can be colon separated.
 */
