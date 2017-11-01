extern crate unicode_segmentation;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use std::num::ParseFloatError;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};

#[derive(Debug, PartialEq)]
pub enum Token {
    Separator,
    Space,
    Word(String),
    Date(String),
    Money(f64), // For now, I hark too well to the problems of floats
    Currency(String),
    Symbol(String),
    Newline(usize),
}

pub fn lex_file(s: &str) -> Result<Vec<Token>, Error> {
    let f = File::open(s)?;
    let file = BufReader::new(&f);
    let results = lex_lines(file.lines())?;
    Ok(results)
}

pub fn lex_lines<T: BufRead>(lines: Lines<T>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    for (i, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                tokens.append(&mut lex_line(&line));
                tokens.push(Token::Newline(i+1));
            }
            Err(_) => { panic!("Perhaps corrupted text file"); }
        }
    }
    Ok(tokens)
}

pub fn lex_line<'a>(line: &'a String) -> Vec<Token> {
    let mut w = line.split_word_bounds().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    let symbol_chars = [Some(";"), Some("#"), Some("%"), Some("|"), Some("*")];
    let integer_regex = Regex::new(r"^\d+$").unwrap();
    let date_dividers = [Some(&"/"), Some(&"-")];
    let currencies = [Some("$"), Some("USD")];

    while w.peek().is_some() {
        let token = w.next();

        // We look at symbols, which depending on context, can be comments or meaningful
        if symbol_chars.contains(&token) {
            tokens.push(Token::Symbol(token.unwrap().to_owned()));
            continue;
        }

        // Multiple spaces are separator
        if token == Some(" ") {
            // Multiple spaces are treated as separator
            if w.peek() == Some(&" ") {
                while w.peek() == Some(&" ") {
                    w.next();
                }
                tokens.push(Token::Separator);
            } else {
                tokens.push(Token::Space);
            }
            continue;
        }


        if integer_regex.is_match(&token.unwrap()) {
            // It is probably a date format
            if date_dividers.contains(&w.peek()) {
                let mut date_token = "".to_owned();
                date_token.push_str(token.unwrap());
                while w.peek() != Some(&" ") && w.peek() != None {
                    // Until it reaches a space or end of file?
                    date_token.push_str(w.next().unwrap());
                }

                tokens.push(Token::Date(date_token));
                continue;
            }
        }


        let possible_amount: Result<f64, ParseFloatError> = token.unwrap().parse();
        match possible_amount {
            Ok(money) => {
                tokens.push(Token::Money(money));
                continue;
            }
            Err(_) => { /* It's not money :-( */ }
        }


        if currencies.contains(&token) {
            tokens.push(Token::Currency(token.unwrap().to_owned()));
            continue;
        }


        // Single space or word
        tokens.push(Token::Word(token.unwrap().to_owned()));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unicode_segmentation() {
        let s = "  Assets:Cash  $100.25";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &[" ", " ", "Assets:Cash", " ", " ", "$", "100.25"]);

        let s = "2014-01-01 * FUNHOUSE TRANSACT";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(
            w,
            &[
                "2014",
                "-",
                "01",
                "-",
                "01",
                " ",
                "*",
                " ",
                "FUNHOUSE",
                " ",
                "TRANSACT",
            ]
        );

        let s = "  Something; to go";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &[" ", " ", "Something", ";", " ", "to", " ", "go"]);
    }

    #[test]
    fn test_lex_line() {
        let s = "  Assets:Cash  $100.25".to_string();
        let lexed_line = lex_line(&s);
        assert_eq!(
            lexed_line,
            &[
                Token::Separator,
                Token::Word("Assets:Cash".to_string()),
                Token::Separator,
                Token::Currency("$".to_string()),
                Token::Money(100.25f64),
            ]
        );

        let s = "2014-01-01 Assets:Cash  $100.25".to_string();
        let lexed_line = lex_line(&s);
        assert_eq!(
            lexed_line,
            &[
                Token::Date("2014-01-01".to_string()),
                Token::Space,
                Token::Word("Assets:Cash".to_string()),
                Token::Separator,
                Token::Currency("$".to_string()),
                Token::Money(100.25f64),
            ]
        );
    }

    #[test]
    fn test_lex_file() {
        // TODO: Learn how one would test the ability to lex multiple lines. Probably not necessary
        // to do because we can trust BufRead to behave correctly.
    }
}
