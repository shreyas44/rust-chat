use std::io::{self, Write};

use chrono::Local;

pub fn get_formatted_time() -> String {
    let now = Local::now();
    let formatted_time = now.format("%d-%m-%Y %H:%M:%S");
    formatted_time.to_string()
}

pub fn print(msg: &str) {
    print!("\r{}", msg);
    io::stdout().flush().unwrap();
}

pub fn println(msg: &str) {
    print(format!("{}\n", msg).as_str());
}

pub fn input(msg: &str) -> String {
    let mut input = String::new();
    print(msg);
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    println!("");
    input.trim().to_string()
}

pub fn backspace() {
    let back = 8u8 as char;
    print!("{} {}", back, back);
    io::stdout().flush().unwrap();
}
