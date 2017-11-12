#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};
use lexer::{Token};
use lexer::TokenType::{Date, Newline, Indentation, Currency, Description, AccountName, Status, EOF};
use error::error;
use chrono;
use chrono::Utc;
use status::Status as TxStatus;

// Now to confabulate these disgraced and shattered things into a story
// Must validate using state machine;
pub fn parse<'a>(tokens: Vec<Token>) {
    let mut date: Option<chrono::Date<Utc>> = None;
    let mut status: TxStatus = TxStatus::Unmarked;

    for token in tokens {
        match token.token_type() {
            &Newline => {
                // Noop
            }
            &Date => {
                if date.is_none() {
                    let date_string = token.literal();
                    let mut naive_date = chrono::NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
                    if naive_date.is_ok() {
                        date = Some(chrono::Date::from_utc(naive_date.unwrap(), chrono::Utc));
                    }
                    else {
                        naive_date = chrono::NaiveDate::parse_from_str(date_string, "%Y-%m-%d");
                    }
                    if naive_date.is_ok() {
                        date = Some(chrono::Date::from_utc(naive_date.unwrap(), chrono::Utc));
                    }
                    else {
                        error(token.line(), "Date is not parseable");
                    }

                }
            }
            &Status => {
                if date.is_none() { error(token.line(), "No Date"); }

                let status_string = &token.literal();
                status = match &status_string[..] {
                    "!" => { TxStatus::Pending }
                    "*" => { TxStatus::Cleared }
                    _ => { TxStatus::Unmarked }
                }
            }
            &Description => {
            }
            &Indentation => {
            }
            &AccountName => {
            }
            &Currency => {
            }
            &EOF => {
                // Done
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // #[should_panic]
    // fn test_panic_on_newline_with_indentation() {
        // parse(vec![TokenType::Indentation]);
    // }
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
