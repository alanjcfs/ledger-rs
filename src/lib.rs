extern crate chrono;
extern crate num;
extern crate regex;
extern crate unicode_segmentation;

pub mod accounting;
pub mod parser;
pub mod lexer;
pub mod status;
pub mod error;
pub mod parser_combinator;

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
 * # ACCOUNTS
 *
 * Contains amounts?
 * Has a name, which can be colon separated.
 */


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
