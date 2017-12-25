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

#[derive(Debug, PartialEq, Clone)]
struct State {
    string: String,
    offset: usize,
}

impl State {
    fn new(string: &str, offset: usize) -> State {
        State { string: string.to_string(), offset: offset }
    }

    fn peek(&self, n: usize) -> Option<String> {
        if self.offset + n <= self.string.len() {
            Some(self.string[self.offset..self.offset + n].to_string())
        } else {
            None
        }
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
    Seq(Vec<Match>),
    Rep(Vec<Match>),
}

#[derive(Debug, PartialEq)]
struct MatchState(Match, State);

fn str_comb(s: String) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let chunk = state.peek(s.len());

        if let Some(chunk) = chunk {
            if chunk == s {
                Some(MatchState(Match::Str(chunk), state.read(s.len())))
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn chr_comb(pattern: String) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let chunk = state.peek(1);

        let pattern_regex = Regex::new(&format!("[{}]", pattern)).unwrap();
        if let Some(chunk) = chunk {
            if pattern_regex.is_match(&chunk) {
                Some(MatchState(Match::Chr(chunk), state.read(1)))
            } else {
                None
            }
        } else {
            None
        }
    })
}

enum Func {
    Str(Box<Fn(State) -> Option<MatchState>>),
    Chr(Box<Fn(State) -> Option<MatchState>>),
}

fn seq_comb(combinators: Vec<Func>) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let mut matches = Vec::new();
        // Feed input state through a chain of other combinators, output state of one combinator
        // becomes the input for the next. If all combinators match, return the sequence
        // with state set to the last state otherwise, return None.
        let mut current_state = Some(state.clone());

        for combinator in &combinators {
            let result = match combinator {
                &Func::Str(ref f) => {
                    f(current_state.clone().unwrap())
                }
                &Func::Chr(ref f) => {
                    f(current_state.clone().unwrap())
                }
            };
            println!("match {:?}", result);
            if let Some(MatchState(node, new_state)) = result {
                println!("state: {:?}, {:?}", node, new_state);
                matches.push(node);
                current_state = Some(new_state.clone());
            }
        }

        if current_state.is_some(){
            Some(MatchState(Match::Seq(matches), current_state.unwrap()))
        }
        else {
            None
        }
    })
}

fn rep_comb(combinator: Func, n: usize) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        let mut matches = Vec::new();
        let mut last_state = None;
        let mut current_state = Some(state.clone());

        while current_state.is_some() {
            let s = current_state.clone().unwrap();
            last_state = Some(s.clone());
            let result = match combinator {
                Func::Str(ref f) => {
                    f(s.clone())
                }
                Func::Chr(ref f) => {
                    f(s.clone())
                }
            };
            println!("{:?}", result);
            if let Some(MatchState(node, new_state)) = result {
                current_state = Some(new_state);
                matches.push(node);
            } else {
                current_state = None;
            }
        }

        if matches.len() >= n {
            Some(MatchState(Match::Rep(matches), last_state.unwrap().clone()))
        } else {
            None
        }

    })
}

fn alt_comb(parsers: Vec<Func>) -> Box<Fn(State) -> Option<MatchState>> {
    Box::new(move |state| {
        for parser in &parsers {
            let r = match parser {
                &Func::Str(ref f) => {
                    f(state.clone())
                }
                &Func::Chr(ref f) => {
                    f(state.clone())
                }
            };
            if let Some(result) = r {
                return Some(result)
            }
        }

        None
    })
}

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
        assert_eq!(state.peek(8).unwrap(), "I'm just");
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
    fn test_str_comb() {
        let input = State::new("hello world", 0);
        let hello = str_comb(r"hello".to_string());
        let world = str_comb(r"world".to_string());

        if let Some(MatchState(m, s)) = hello(input) {
            assert_eq!(m, Match::Str("hello".to_string()));
            assert_eq!(s, State::new("hello world", 5));
        } else {
            panic!("no output")

        }

        let input = State::new("hello world", 0);
        if let Some(MatchState(m, s)) = world(input.read(6)) {
            assert_eq!(m, Match::Str("world".to_string()));
            assert_eq!(s, State::new("hello world", 11));
        } else {
            panic!("no output")
        }
    }

    #[test]
    fn test_chr_comb() {
        let input = State::new("12 + 34", 0);
        let digit = chr_comb(r"0-9".to_string());

        if let Some(MatchState(m, s)) = digit(input.read(1)) {
            assert_eq!(m, Match::Chr("2".to_string()));
            assert_eq!(s, State::new("12 + 34", 2))
        } else {
            panic!("no output")
        }

        if let Some(MatchState(m, s)) = digit(input.read(5)) {
            assert_eq!(m, Match::Chr("3".to_string()));
            assert_eq!(s, State::new("12 + 34", 6))
        } else {
            panic!("no output")
        }

        assert_eq!(digit(input.read(2)), None);
    }

    #[test]
    fn test_seq_comb() {
        let input = State::new("7+8", 0);
        // Sanity check
        let digit = chr_comb(r"0-9".to_string());
        let reg = str_comb("+".to_string());
        assert_eq!(digit(input.read(0)), Some(MatchState(Match::Chr("7".to_string()), State::new("7+8", 1))));
        assert_eq!(reg(input.read(1)), Some(MatchState(Match::Str("+".to_string()), State::new("7+8", 2))));
        assert_eq!(digit(input.read(2)), Some(MatchState(Match::Chr("8".to_string()), State::new("7+8", 3))));


        let addition = seq_comb(vec![Func::Chr(chr_comb("0-9".to_string())), Func::Str(str_comb(r"+".to_string())), Func::Chr(chr_comb("0-9".to_string()))]);

        let results = addition(input);
        println!("{:?}", results);
        if let Some(MatchState(m, s)) = results {
            assert_eq!(
                m,
                Match::Seq(
                    vec![
                        Match::Chr("7".to_string()),
                        Match::Str("+".to_string()),
                        Match::Chr("8".to_string()),
                        ]
                          )
                      );
            assert_eq!(s, State::new("7+8", 3));
        } else {
            panic!("addition(input) did not generate a result")
        }
    }

    #[test]
    fn test_rep_comb() {
        let input = State::new("2017", 0);
        let number = rep_comb(Func::Chr(chr_comb("0-9".to_string())), 1);
        let results = number(input);
        println!("{:?}", results);
        if let Some(MatchState(m, s)) = results {
            assert_eq!(
                m,
                Match::Rep(
                    vec![
                    Match::Chr("2".to_string()),
                    Match::Chr("0".to_string()),
                    Match::Chr("1".to_string()),
                    Match::Chr("7".to_string()),
                    ]
                          )
                      );
        } else {
            panic!("number(input) did not generate a result")
        }
    }

    #[test]
    fn test_alt_comb() {
        let w = rep_comb(str_comb(" "), 0);
        let number = alt_comb(str_comb("0"), seq_comb(chr_comb("1-9"), rep_comb(chr_comb("0-9"), 0)))
    }
}
