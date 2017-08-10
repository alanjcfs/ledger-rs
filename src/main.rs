extern crate clap;
extern crate ledger;

use clap::{Arg, App, SubCommand};

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
        .get_matches();

    let file = matches.value_of("file").unwrap_or("examples/example.ledger");

    let contents = ledger::read(file).unwrap();
    let mut lines = contents.lines();

    for line in lines {
        let ignored_chars = [Some(';'), Some('#'), Some('%'), Some('|'), Some('*')];

        if ignored_chars.contains(&line.chars().next()) {
            // noop
        } else if line.len() == 0 {
            println!("Empty line")
        }
    }
}
