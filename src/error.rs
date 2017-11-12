pub fn error(line: usize, message: &str) {
    eprintln!("[line {}] Error: {}", line, message)
}
