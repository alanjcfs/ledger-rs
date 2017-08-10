use std::io::Read;
use std::fs::File;
use std::io;

pub fn read(s: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(s)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let read_file = super::read("examples/example.ledger").unwrap();
        let lines: Vec<&str> = read_file.lines().collect();
        assert_eq!(lines[0], "; is a comment");
        assert_eq!(lines[1], "# also a comment");
    }
}
