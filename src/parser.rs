#[allow(unused_imports)]
use accounting::{Account, Transaction, Posting, Amount, CommoditySymbol};
use lexer::{Token};
use lexer::TokenType::{Date, Newline, Indentation, Currency, CurrencyInferred, Description, AccountName, Status, EOF};
use error::error;
use chrono;
use chrono::Utc;
use status::Status as TxStatus;
use unicode_segmentation::UnicodeSegmentation;

// Now to confabulate these disgraced and shattered things into a story
// Must validate using state machine;
pub fn parse<'a>(tokens: Vec<Token>) -> Vec<Posting> {
    let mut date: Option<chrono::Date<Utc>> = None;
    let mut status: TxStatus = TxStatus::Unmarked;
    let mut description = "".to_string();
    let mut transaction: Option<Transaction> = None;
    let mut current_account: Option<Account> = None;

    let mut postings: Vec<Posting> = Vec::new();

    for token in tokens {
        match token.token_type() {
            &Newline => {
                // Noop
            }
            &Date => {
                // Reset
                if date.is_some() {
                    date = None;
                    status = TxStatus::Unmarked;
                    description.clear();
                    transaction = None;
                    current_account = None;
                }

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
            &Status => {
                if date.is_none() { error(token.line(), "No Date for Status"); }

                // TODO: Would be nice to revise to not use the catch-all matching
                let status_string = &token.literal();
                status = match &status_string[..] {
                    "!" => { TxStatus::Pending }
                    "*" => { TxStatus::Cleared }
                    _ => { TxStatus::Unmarked }
                }
            }
            &Description => {
                if date.is_none() { error(token.line(), "No Date for Description"); }

                description.push_str(token.literal());

                transaction = Some(Transaction::new(token.line(), date.unwrap(), status, description.clone()));
            }
            &Indentation => {
                if date.is_none() { error(token.line(), "No Date for Indentation"); }
            }
            &AccountName => {
                if date.is_none() { error(token.line(), "No Date for Account Name"); }

                let literal = token.literal();
                current_account = Some(Account::new(literal.clone()));
            }
            &Currency => {
                let literal = token.literal().clone();
                let split_words = literal.split_word_bounds().collect::<Vec<&str>>();
                let iterable_words = split_words.iter();
                let mut currency: Option<CommoditySymbol> = None;
                let mut is_negative = false;
                let mut starts_with_dot = false;
                let mut amount: f64 = 0f64;

                for word in iterable_words {
                    match word {
                        &"$" => {
                            currency = Some(word.to_string())
                        }
                        &"-" => {
                            is_negative = true;
                        }
                        &"." => {
                            starts_with_dot = true;
                        }
                        &number => {
                            let float_str: String =
                                if starts_with_dot == true {
                                    let mut s = ".".to_string();
                                    s.push_str(number);
                                    s
                                }
                                else {
                                    number.to_string()
                                };
                            let parsed = float_str.parse();
                            match parsed {
                                Ok(parsed) => {
                                    amount = parsed;
                                }
                                Err(err) => {
                                    error(token.line(), &format!("{}: Could not parse {}", err, literal))
                                }
                            }
                        }
                    }
                }
                if is_negative == true {
                    amount = -amount;
                }

                let amount = Amount::new(currency.unwrap(), amount);
                postings.push(Posting::new(transaction.clone().unwrap(), current_account.clone().unwrap(), Some(amount)));
            }
            &CurrencyInferred => {
                postings.push(Posting::new(transaction.clone().unwrap(), current_account.clone().unwrap(), None));
            }
            &EOF => {
                // Done
            }
        }
    }

    postings
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_float() {
        assert_eq!("04".parse::<f64>(), Ok(4f64));
        assert_eq!(".04".parse::<f64>(), Ok(0.04));
    }


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
