use std::io::{self, Write};

pub fn print(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

pub fn input(msg: &str) -> String {
    let mut input = String::new();
    print(msg);
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_string()
}

pub fn backspace() {
    print!("{}", 8u8 as char);
    io::stdout().flush().unwrap();
}
