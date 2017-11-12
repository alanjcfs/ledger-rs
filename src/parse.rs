extern crate chrono;

#[allow(unused_imports)]
use accounting::{Account, Balance, Transaction, Posting, Amount};
use lexer::{Token, TokenType};
use error::error;

// Now to confabulate these disgraced and shattered things into a story
// Must validate using state machine;
pub fn parse<'a>(tokens: Vec<Token>) {
    let mut prev_token = TokenType::Newline;
    for token in tokens {
        match token.token_type() {
            &TokenType::Newline => {
                prev_token = TokenType::Newline;
            }
            &TokenType::Indentation => {
                if prev_token == TokenType::Newline {
                    error(token.line(), "Identation error");
                }
            }
            &TokenType::Currency => {
                if prev_token != TokenType::AccountName {
                    error(token.line(), "No account name")
                }

            }
            &TokenType::Description => {

            }
            &TokenType::AccountName => {

            }
            &TokenType::Status => {

            }
            &TokenType::Date => {

            }
            &TokenType::EOF => {

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
