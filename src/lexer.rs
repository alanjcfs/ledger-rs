extern crate unicode_segmentation;

// use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::result::Result;
use error::error;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Star,
    Bang,
    Slash,
    Space,
    Newline,
    Hyphen,
    Indentation,
    Modulo,
    Colon,
    Semicolon,
    Hash,
    Pipe,
    Number,
    String,
    EOF,
}

// We don't need a literal because we don't need to parse 
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String, // Can be None
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token { token_type: token_type, lexeme: lexeme, line: line }
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }
}

trait AddToken {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize);
    fn add_token_type<'a>(&'a mut self, token_type: TokenType, line: usize);
}

impl AddToken for Vec<Token> {
    fn add_token<'a>(&'a mut self, token_type: TokenType, grapheme: &str, line: usize) {
        self.push(Token::new(token_type, grapheme.to_string(), line));
    }
    fn add_token_type<'a>(&'a mut self, token_type: TokenType, line: usize) {
        self.add_token(token_type, &"".to_string(), line);
    }
}

struct Scanner {
    source: Vec<String>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner {
    fn new(source: Vec<String>) -> Scanner {
        Scanner { source: source, tokens: Vec::new(), start: 0, current: 0, line: 1 }
    }

    fn lex(&mut self) -> Result<Vec<Token>, ()> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.add_token(TokenType::EOF, &"".to_string(), self.line);

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        self.current += 1;
        let c = &self.source[self.current - 1];
        match c.as_str() {
            "*" => {
                self.tokens.add_token_type(TokenType::Star, self.line)
            }
            "/" => {
                self.tokens.add_token_type(TokenType::Slash, self.line)
            }
            ";" => {
                self.tokens.add_token_type(TokenType::Semicolon, self.line)
            }
            "%" => {
                self.tokens.add_token_type(TokenType::Modulo, self.line)
            }
            "|" => {
                self.tokens.add_token_type(TokenType::Pipe, self.line)
            }
            "-" => {
                self.tokens.add_token_type(TokenType::Hyphen, self.line)
            }
            " " => {
                if self.is_match(&" ".to_string()) {
                    while self.peek() == " ".to_string() && !self.is_at_end() {
                        self.current += 1;
                    }
                    self.tokens.add_token_type(TokenType::Indentation, self.line);
                }
                else {
                    self.tokens.add_token_type(TokenType::Space, self.line);
                }
            }
            "\n" => {
                self.tokens.add_token_type(TokenType::Newline, self.line);
            }
            _ => {
                error(self.line, "Unexpected character.");
            }
        }
    }

    fn is_match(&self, expected: &String) -> bool {
        if self.is_at_end() {
            return false
        }
        if &self.source[self.current] != expected {
            return false
        }
        true
    }

    fn peek(&self) -> String {
        if self.is_at_end() {
            "\0".to_string()
        }
        else {
            self.source[self.current].clone()
        }
    }
}

pub fn lex_file(s: &str) -> Result<Vec<Token>, Error> {
    let f = File::open(s)?;
    let mut file = BufReader::new(&f);
    let mut string: String = "".to_string();
    file.read_to_string(&mut string)?;
    let mut results = Scanner::new(UnicodeSegmentation::graphemes(&string[..], true).map(|x| x.to_string()).collect::<Vec<String>>());
    results.lex();
    Ok(results.tokens)
}

pub fn lex(string: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut graphemes = UnicodeSegmentation::graphemes(&string[..], true).collect::<Vec<&str>>();


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
        let s = "  Assets:Cash  -$100.25\n".to_string();
        let lexed_line = lex(1, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Indentation, None, &"  ", 1 ),
                Token::new( TokenType::AccountName, None, &"Assets:Cash", 1 ),
                Token::new( TokenType::Indentation, None, &"  ", 1 ),
                Token::new( TokenType::Currency, None, &"-$100.25", 1 ),
            ]
        );

        let s = "  Assets:Cash\n".to_string();
        let lexed_line = lex(2, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Indentation, None, &"  ", 2 ),
                Token::new( TokenType::AccountName, None, &"Assets:Cash", 2 ),
            ]
        );
    }

    #[test]
    fn test_lex_date_description() {
        let s = "2014-01-01 * A Description\n".to_string();
        let lexed_line = lex(1, &s);
        assert_eq!(
            lexed_line,
            &[
                Token::new( TokenType::Date, None, &"2014-01-01".to_string(), 1 ),
                Token::new( TokenType::Status, None, &"*", 1 ),
                Token::new( TokenType::Description, None, &"A Description", 1 ),
            ]
        );
    }

    #[test]
    fn test_lex_file() {
        // TODO: Learn how one would test the ability to lex multiple lines. Probably not necessary
        // to do because we can trust BufRead to behave correctly.
    }
}
