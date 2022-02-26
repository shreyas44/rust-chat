mod utils;
use std::{
    io::{self, BufRead, Write},
    net, thread,
};

use colored::Colorize;

pub struct User {
    name: String,
    stream: net::TcpStream,
}

impl User {
    pub fn new_get_name(stream: net::TcpStream) -> User {
        let name = utils::input("Enter your name: ");
        let user = User { name, stream };
        user
    }

    fn send(&self, msg: &str) {
        let msg = format!("{}: {}\n", self.name.cyan(), msg);
        let mut stream = self.stream.try_clone().unwrap();

        stream.write(msg.as_bytes()).unwrap();
    }

    fn watch_output(&self) {
        let mut reader = io::BufReader::new(&self.stream);

        loop {
            let mut msg = String::new();
            reader.read_line(&mut msg).unwrap();
            utils::print(format!("\r{}{}: ", msg, self.name.green()).as_str());
        }
    }

    fn watch_input(&self) {
        loop {
            let input = utils::input(format!("{}: ", self.name.green()).as_str());
            self.send(&input);
        }
    }

    pub fn start_session(self) {
        let output_clone = self.clone();
        let input_clone = self.clone();

        let output_thread = thread::spawn(move || output_clone.watch_output());
        let input_thread = thread::spawn(move || input_clone.watch_input());

        output_thread.join().unwrap();
        input_thread.join().unwrap();
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            name: self.name.clone(),
            stream: self.stream.try_clone().unwrap(),
        }
    }
}
