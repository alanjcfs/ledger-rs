extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use regex::Regex;

pub fn read_file(s: &str) -> Result<Vec<String>, Error> {
    let f = File::open(s)?;
    let mut file = BufReader::new(&f);
    let mut string: String = "".to_string();
    file.read_to_string(&mut string)?;
    let unicode_scan: Vec<String> = UnicodeSegmentation::graphemes(&string[..], true).map(|x| x.to_string()).collect();
    Ok(unicode_scan)
}

#[derive(Debug, PartialEq)]
struct State {
    string: String,
    offset: usize,
}

impl State {
    fn new(string: &str, offset: usize) -> State {
        State { string: string.to_string(), offset: offset }
    }

    fn peek(&self, n: usize) -> String {
        self.string[self.offset..self.offset + n].to_string()
    }

    fn read(&self, n: usize) -> State {
        State::new(&self.string, self.offset + n)
    }

    fn is_complete(&self) -> bool {
        self.offset == self.string.len()
    }
}

#[derive(Debug, PartialEq)]
enum Match {
    Str(String),
    Chr(String),
}

#[derive(Debug, PartialEq)]
struct MatchState(Match, State);

fn str_generator(s: String) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let chunk = state.peek(s.len());

        if chunk == s {
            Some(MatchState(Match::Str(chunk), state.read(s.len())))
        } else {
            None
        }
    })
}

fn chr_generator(pattern: String) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let chunk = state.peek(1);

        let pattern_regex = Regex::new(&pattern.to_string()).unwrap();
        if pattern_regex.is_match(&chunk) {
            Some(MatchState(Match::Chr(chunk), state.read(1)))
        } else {
            None
        }
    })
}

// fn seq_generator(parsers: Vec<Fn>) -> Box<Fn(State) -> Option<MatchState>> {
//     Box::new(move |state| {
//         let mut matches = Vec::new();
//
//         for parser in parsers {
//             let (node, new_state) = parser(state);
//             if new_state {
//                 matches.push(node);
//             }
//         }
//
//         if new_state {
//             MatchState(Match::Seq(matches), new_state)
//         } else {
//             None
//         }
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = State::new(&"I'm just a string", 0);
        assert_eq!(state, State { string: "I'm just a string".to_string(), offset: 0 });
    }

    #[test]
    fn test_peek() {
        let state = State::new(&"I'm just a string", 0);
        assert_eq!(state.peek(8), "I'm just");
    }

    #[test]
    fn test_read() {
        let state = State::new("I'm just a string", 11);
        assert_eq!(state.read(6), State { string: "I'm just a string".to_string(), offset: 17 });
    }

    #[test]
    fn test_is_complete() {
        let state = State::new("I'm just a string", 0);
        assert_eq!(state.is_complete(), false);
        let state = State::new("I'm just a string", 17);
        assert_eq!(state.is_complete(), true);
    }

    #[test]
    fn test_str_generator() {
        let input = State::new("hello world", 0);
        let hello = str_generator("hello".to_string());
        let world = str_generator("world".to_string());

        if let Some(MatchState(m, s)) = hello(input) {
            assert_eq!(m, Match::Str("hello".to_string()));
            assert_eq!(s, State::new("hello world", 5));
        }

        let input = State::new("hello world", 0);
        if let Some(MatchState(m, s)) = hello(input.read(6)) {
            assert_eq!(m, Match::Str("hello".to_string()));
            assert_eq!(s, State::new("hello world", 5));
        }
    }

    #[test]
    fn test_chr_generator() {
        let input = State::new("12 + 34", 0);
        let digit = chr_generator(r"0-9".to_string());

        if let Some(MatchState(m, s)) = digit(input.read(1)) {
            assert_eq!(m, Match::Chr("2".to_string()));
            assert_eq!(s, State::new("12 + 34", 1))
        }

        if let Some(MatchState(m, s)) = digit(input.read(5)) {
            assert_eq!(m, Match::Chr("3".to_string()));
            assert_eq!(s, State::new("12 + 34", 6))
        }

        assert_eq!(digit(input.read(2)), None)
    }
}
