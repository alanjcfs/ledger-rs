extern crate clap;
extern crate ledger;

use clap::{Arg, App, SubCommand};
use ledger::accounting::Transaction;

fn main() {
    let matches = App::new("ledger-rs")
        .version("0.1")
        .about("Rust port of Ledger CLI")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("Set the file to use")
             .takes_value(true))
        .subcommand(SubCommand::with_name("balance"))
        .subcommand(SubCommand::with_name("budget"))
        .get_matches();

    let file = matches.value_of("file").unwrap_or("examples/example.ledger");

    let contents = ledger::read(file).unwrap();
    let lines = contents.lines();
    let mut ledger: Vec<Option<Transaction>> = Vec::new();
    ledger::parse(lines, &mut ledger);

    // When balance passed, print out transactions
    //    20  Assets
    //  -600    Bank of America
    //    20    Cash
    //   600    Savings
    // -1720  Equity:Opening Balance
    //  1720  Expenses
    //   160    Food
    //  1560    Rent
    //  ----
    //     0
    //
    // In each transaction, the accounts should add up to zero to be balanced.
    // We need to store the total ins and outs of each account too.

}
