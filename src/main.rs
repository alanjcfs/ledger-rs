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
    let mut lines = contents.lines();
    let mut ledger: Vec<Transaction> = Vec::new();
    ledger::parse(lines, &ledger);
}
