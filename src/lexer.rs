extern crate unicode_segmentation;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use std::num::ParseFloatError;
use std::fs::File;
use std::io::{Read, BufReader, Error};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    Colon, Semicolon, Hash, Modulo, Pipe, Star, Bang, Equal, Tilde,

    Indentation,
    Newline,
    String,
    Number,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Token {
        Token { token_type: token_type, lexeme: lexeme, literal: literal, line: line }
    }
}

fn error(line: usize, message: &str) {
    eprintln!("[line {}] Error: {}", line, message)
}

pub fn lex_file(s: &str) -> Result<Vec<Token>, Error> {
    let f = File::open(s)?;
    let mut file = BufReader::new(&f);
    let mut string: String = "".to_string();
    file.read_to_string(&mut string)?;
    let results = lex(&string)?;
    Ok(results)
}

trait AddToken {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize);
}

impl AddToken for Vec<Token> {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize) {
        self.push(Token::new(token_type, grapheme.to_string(), None, line));
    }
}

fn add_token(tokens: &mut Vec<Token>, token_type: TokenType, grapheme: &str, line: usize) {
    tokens.push(Token::new(token_type, grapheme.to_string(), None, line));
}

pub fn lex(string: &String) -> Result<Vec<Token>, Error> {
    let mut line: usize = 1;
    let mut tokens: Vec<Token> = Vec::new();
    let mut graphemes = UnicodeSegmentation::graphemes(&string[..], true).peekable();
    let integer_regex = Regex::new(r"^\d$").unwrap();
    let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9]$")
    let date_dividers = [Some(&"/"), Some(&"-")];
    let mut handled_date = false;

    while graphemes.peek().is_some() {
        let grapheme = graphemes.next().unwrap();

        match grapheme {
            ":" => {
                tokens.add_token(TokenType::Colon, grapheme, line);
            }
            ";" => {
                tokens.add_token(TokenType::Semicolon, grapheme, line);
            }
            "#" => {
                tokens.add_token(TokenType::Hash, grapheme, line);
            }
            "%" => {
                tokens.add_token(TokenType::Modulo, grapheme, line);
            }
            "|" => {
                tokens.add_token(TokenType::Pipe, grapheme, line);
            }
            "*" => {
                tokens.add_token(TokenType::Star, grapheme, line);
            }
            "!" => {
                tokens.add_token(TokenType::Bang, grapheme, line);
            }
            "=" => {
                tokens.add_token(TokenType::Equal, grapheme, line)
            }
            "~" => {
                tokens.add_token(TokenType::Tilde, grapheme, line)
            }
            " " => {
                let mut s = " ".to_string();
                if graphemes.peek() == Some(&" ") {
                    while graphemes.peek() == Some(&" ") {
                        s.push_str(graphemes.next().unwrap());
                    }
                    tokens.add_token(TokenType::Indentation, &s, line);
                } else {
                    while graphemes.peek() != Some(&"\n") {
                        s.push_str(graphemes.next().unwrap());
                    }
                    tokens.add_token(TokenType::String, &s, line);
                }
            }
            "\t" => {
                let mut s = "\t".to_string();
                while graphemes.peek() == Some(&"\t") {
                    s.push_str(graphemes.next().unwrap());
                }
                tokens.add_token(TokenType::Indentation, &s, line)
            }
            digit if integer_regex.is_match(digit) => {
                let mut s = digit.to_string();
                while integer_regex.is_match(graphemes.peek().unwrap()) {
                    s.push_str(graphemes.next().unwrap());
                    // Handle / and - that are dates
                    if date_dividers.contains(&graphemes.peek()) {
                        s.push_str(graphemes.next().unwrap());
                    }
                    // Handle dot in numbers with decimal points
                    // TODO: Multiple decimal points
                    if graphemes.peek() == Some(&".") {
                        let dot = graphemes.next();
                        if integer_regex.is_match(graphemes.peek().unwrap()) {
                            s.push_str(dot.unwrap());
                        }
                    }
                }
            }
            "\n" => {
                tokens.add_token(TokenType::Newline, &s, line);
                line += 1;
            }
            _ => {
                error(line, "Unexpected character.");
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, "".to_string(), None, line));



    //
    // let symbol_chars = [Some(";"), Some("#"), Some("%"), Some("|"), Some("*")];
    // let integer_regex = Regex::new(r"^\d+$").unwrap();
    // let date_dividers = [Some(&"/"), Some(&"-")];
    // let currencies = [Some("$"), Some("USD")];
    //
    // while w.peek().is_some() {
    //     let token = w.next();
    //
    //     // We look at symbols, which depending on context, can be comments or meaningful
    //     if symbol_chars.contains(&token) {
    //         tokens.push(TokenType::Symbol(token.unwrap().to_owned()));
    //         continue;
    //     }
    //
    //     // Multiple spaces are separator
    //     if token == Some(" ") {
    //         // Multiple spaces are treated as separator
    //         if w.peek() == Some(&" ") {
    //             while w.peek() == Some(&" ") {
    //                 w.next();
    //             }
    //             tokens.push(TokenType::Separator);
    //         } else {
    //             tokens.push(TokenType::Space);
    //         }
    //         continue;
    //     }
    //
    //
    //     if integer_regex.is_match(&token.unwrap()) {
    //         // It is probably a date format
    //         if date_dividers.contains(&w.peek()) {
    //             let mut date_token = "".to_owned();
    //             date_token.push_str(token.unwrap());
    //             while w.peek() != Some(&" ") && w.peek() != None {
    //                 // Until it reaches a space or end of file?
    //                 date_token.push_str(w.next().unwrap());
    //             }
    //
    //             tokens.push(TokenType::Date(date_token));
    //             continue;
    //         }
    //     }
    //
    //
    //     let possible_amount: Result<f64, ParseFloatError> = token.unwrap().parse();
    //     match possible_amount {
    //         Ok(money) => {
    //             tokens.push(TokenType::Money(money));
    //             continue;
    //         }
    //         Err(_) => { #<{(| It's not money :-( |)}># }
    //     }
    //
    //
    //     if currencies.contains(&token) {
    //         tokens.push(TokenType::Currency(token.unwrap().to_owned()));
    //         continue;
    //     }
    //
    //
    //     // Single space or word
    //     tokens.push(TokenType::Word(token.unwrap().to_owned()));
    // }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unicode_graphemes() {
        let s = "  Assets:Cash  $100.25\n";
        let w = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
        assert_eq!(w, &[" ", " ", "A", "s", "s", "e", "t", "s", ":", "C", "a", "s", "h", " ", " ", "$", "1", "0", "0", ".", "2", "5", "\n"])
    }

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

    // #[test]
    // fn test_lex_line() {
    //     let s = "  Assets:Cash  $100.25".to_string();
    //     let lexed_line = lex_line(&s);
    //     assert_eq!(
    //         lexed_line,
    //         &[
    //             TokenType::Separator,
    //             TokenType::Word("Assets:Cash".to_string()),
    //             TokenType::Separator,
    //             TokenType::Currency("$".to_string()),
    //             TokenType::Money(100.25f64),
    //         ]
    //     );
    //
    //     let s = "2014-01-01 Assets:Cash  $100.25".to_string();
    //     let lexed_line = lex_line(&s);
    //     assert_eq!(
    //         lexed_line,
    //         &[
    //             TokenType::Date("2014-01-01".to_string()),
    //             TokenType::Space,
    //             TokenType::Word("Assets:Cash".to_string()),
    //             TokenType::Separator,
    //             TokenType::Currency("$".to_string()),
    //             TokenType::Money(100.25f64),
    //         ]
    //     );
    // }

    #[test]
    fn test_lex_file() {
        // TODO: Learn how one would test the ability to lex multiple lines. Probably not necessary
        // to do because we can trust BufRead to behave correctly.
    }
}
