extern crate chrono;

use regex::Regex;

#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};
use lexer::TokenType;

// Now to confabulate these disgraced and shattered things into a story...
// of money.
// Must validate using state machine;
pub fn parse<'a>(tokens: Vec<TokenType>) {
}






#[cfg(test)]
mod tests {
    use super::*;
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
