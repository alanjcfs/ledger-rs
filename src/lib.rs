extern crate chrono;
extern crate num;
extern crate regex;

pub mod accounting;
pub mod read;
pub mod parse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
