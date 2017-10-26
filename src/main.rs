extern crate clap;
extern crate ledger;

use clap::{App, Arg, SubCommand};
use ledger::accounting::{Transaction, Posting};

fn main() {
    let matches = App::new("ledger-rs")
        .version("0.1")
        .about("Rust port of Ledger CLI")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Set the file to use")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("balance"))
        .subcommand(SubCommand::with_name("budget"))
        .get_matches();

    let file = matches
        .value_of("file")
        .unwrap_or("examples/example.journal");

    // let contents = ledger::read::read(file).unwrap();
    // let lines = contents.lines();
    // let mut ledger: Vec<Option<Transaction>> = Vec::new();
    // let mut postings: Vec<Posting> = Vec::new();
    // ledger::parse::parse(lines, &mut ledger, &mut postings);

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

    // println!("{:?}", ledger);
    // println!("{:?}", postings);

    let result = ledger::lexer::lex_file(file);
    match result {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(res) => {
            println!("Could not open and read file");
        }
    }
}
