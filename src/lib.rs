extern crate chrono;
extern crate num;
extern crate regex;

pub mod accounting;
pub mod read;
pub mod parse;

use accounting::{Balance, Transaction};

pub fn check_extra_empty_accounts(ledger: &Vec<Option<Transaction>>) -> &Vec<Option<Transaction>> {
    for t in ledger.clone() {
        match t {
            Some(ref u) => if u.account_changes()
                .into_iter()
                .filter(|v| {
                    let bal = v.balance().clone();
                    bal.is_empty()
                })
                .count() > 1
            {
                panic!("No more than one null amount value")
            },
            None => { /* noop */ }
        }
    }
    &ledger
}

pub fn verify<'a>(ledger: Vec<Option<Transaction>>) {
    for t in ledger {
        match t {
            Some(u) => {
                let mut _sum = 0.;
                for a in u.account_changes() {
                    match a.balance() {
                        &Balance::Amount(f) => _sum += f,
                        &Balance::Empty => {}
                    }
                }
            }
            None => { /* noop */ }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
