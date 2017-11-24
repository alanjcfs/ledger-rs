extern crate unicode_segmentation;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use std::fs::File;
use std::io::{BufReader, Error, BufRead};
use std;
use error::error;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Comment,
    Date,
    Status,
    Description,
    Indentation,
    AccountName,
    Currency,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: String, // Should be the whole string and should not be None
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, literal: &str, line: usize) -> Token {
        Token { token_type: token_type, literal: literal.to_string(), line: line }
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn literal(&self) -> &String {
        &self.literal
    }
}


pub fn lex_file(s: &str) -> Result<Vec<Token>, Error> {
    let f = File::open(s)?;
    let file = BufReader::new(&f);
    let results = lex_lines(file.lines())?;
    Ok(results)
}

trait AddToken {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize);
    fn add_token_type<'a>(&'a mut self, token_type: TokenType, line: usize);
}

impl AddToken for Vec<Token> {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize) {
        self.push(Token::new(token_type, grapheme, line));
    }
    fn add_token_type<'a>(&'a mut self, token_type: TokenType, line: usize) {
        self.push(Token::new(token_type, &"", line));
    }
}

fn lex_lines<T: BufRead>(lines: std::io::Lines<T>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line_count = 0;
    for (i, line) in lines.enumerate() {
        if let Ok(line) = line {
                line_count = i;
                tokens.append(&mut lex(i, &line));
        }
        else {
            error(i, "Corrupted text file that cannot be enumerated");
        }
    }

    tokens.add_token_type(TokenType::EOF, line_count + 1);

    Ok(tokens)
}

pub fn lex(idx: usize, string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut graphemes = UnicodeSegmentation::graphemes(&string[..], true).peekable();
    let integer_regex = Regex::new(r"^\d$").unwrap();
    let date_dividers = [Some(&"/"), Some(&"-")];
    // let indentation_regex = Regex::new(r" |\t+").unwrap();

    while let Some(grapheme) = graphemes.next() {
        match grapheme {
            // ignore comments
            // TODO:
            // - Process comments for tags and other metadata
            // - Process comment blocks as well
            ";" | "#" | "%" | "|" | "*" => {
                let mut s = grapheme.to_string();
                while let Some(g) = graphemes.next() {
                    s.push_str(g);
                }
                tokens.add_token(TokenType::Comment, &s, idx)
            }
            // Begins with space, process as account
            // TODO:
            // - Elegant approach to indented comments would be nice
            // - Handle tabs
            " " => {
                while graphemes.peek() == Some(&" ") {
                    graphemes.next().unwrap();
                }
                tokens.add_token_type(TokenType::Indentation, idx);

                let mut account_name = "".to_string();
                if graphemes.peek().is_some() {
                    while graphemes.peek().is_some() {
                        let mut account_char = graphemes.next().unwrap();
                        account_name.push_str(account_char);

                        if [Some(&";"), Some(&"#"), Some(&"%"), Some(&"|"), Some(&"*")].contains(&graphemes.peek()) {
                            break;
                        }
                        if graphemes.peek() == Some(&" ") {
                            account_char = graphemes.next().unwrap();
                            if graphemes.peek() == Some(&" ") {

                                // separator
                                tokens.add_token(TokenType::AccountName, &account_name, idx);
                                account_name.clear();

                                while graphemes.peek() == Some(&" ") {
                                    graphemes.next();
                                }
                                tokens.add_token_type(TokenType::Indentation, idx);

                                // process as currency
                                let mut currency = "".to_string();
                                if graphemes.peek().is_some() {
                                    while graphemes.peek().is_some() {
                                        currency.push_str(graphemes.next().unwrap());
                                    }
                                    tokens.add_token(TokenType::Currency, &currency, idx);
                                }
                            }

                            // account name
                            else {
                                account_name.push_str(account_char);
                            }
                        }
                    }
                    if !account_name.is_empty() {
                        tokens.add_token(TokenType::AccountName, &account_name, idx);
                        account_name.clear();
                    }
                }
            }

            // Begins with digit, process as date (*|~)? description
            // TODO:
            // - Process edate, which is = sign followed by a second date
            digit if integer_regex.is_match(digit) => {
                let mut s = digit.to_string();
                while graphemes.peek().is_some() && integer_regex.is_match(graphemes.peek().unwrap()) {
                    s.push_str(graphemes.next().unwrap());
                    // Handle / and - that are dates
                    if date_dividers.contains(&graphemes.peek()) {
                        s.push_str(graphemes.next().unwrap());
                    }
                }
                if graphemes.peek() == Some(&" ") {
                    tokens.add_token(TokenType::Date, &s, idx);
                    s.clear();
                    // process for */! and description
                    while graphemes.peek() == Some(&" "){
                        graphemes.next();
                    }
                }
                if graphemes.peek().is_some() {
                    if [Some(&"*"), Some(&"!")].contains(&graphemes.peek()) {
                        let status = graphemes.next().unwrap();
                        if graphemes.peek() == Some(&" ") {
                            tokens.add_token(TokenType::Status, &status, idx);
                            graphemes.next();
                        }
                        else {
                            // There is no space so it might be part of the description
                            s.push_str(status);
                        }
                    }
                    while graphemes.peek().is_some() {
                        s.push_str(graphemes.next().unwrap());
                    }

                    tokens.add_token(TokenType::Description, &s, idx);
                    s.clear();
                }
            }
            _other_char => {
                // TODO: We're currently ignoring other characters
                // error(idx, &format!("An unknown character: {}", other_char));
            }
        }
    }

    tokens
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
    fn test_unicode_currency() {
        let s = "$1234.23";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &["$", "1234.23"]);

        let s = "-$1234.23";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &["-", "$", "1234.23"]);

        let s = "$-1234.23";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &["$", "-", "1234.23"]);

        let s = "$.04";
        let w = s.split_word_bounds().collect::<Vec<&str>>();
        assert_eq!(w, &["$", ".", "04"]);
    }

    #[test]
    fn test_lex_account() {
        let s = "  Assets:Cash  -$100.25".to_string();
        let lexed_line = lex(1, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Indentation, &"", 1 ),
                Token::new( TokenType::AccountName, &"Assets:Cash", 1 ),
                Token::new( TokenType::Indentation, &"", 1 ),
                Token::new( TokenType::Currency, &"-$100.25", 1 ),
            ]
        );

        let s = "  Assets:Cash".to_string();
        let lexed_line = lex(2, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Indentation, &"", 2 ),
                Token::new( TokenType::AccountName, &"Assets:Cash", 2 ),
            ]
        );
    }

    #[test]
    fn test_lex_date_description() {
        let s = "2014-01-01 * A Description".to_string();
        let lexed_line = lex(1, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Date, &"2014-01-01".to_string(), 1 ),
                Token::new( TokenType::Status, &"*", 1 ),
                Token::new( TokenType::Description, &"A Description", 1 ),
            ]
        );
    }

    #[test]
    fn test_lex_file() {
        // TODO: Learn how one would test the ability to lex multiple lines. Probably not necessary
        // to do because we can trust BufRead to behave correctly.
    }
}
