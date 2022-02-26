use new_chat::User;
use std::{env, net};

mod cmd;
mod utils;

const PORT: i32 = 8080;

fn main() {
    let mut args = env::args();
    let cmd = cmd::parse_command(&mut args).unwrap_or_else(|err| panic!("{}", err.to_string()));
    match cmd {
        cmd::Command::Connect(addr) => connect(&addr),
        cmd::Command::Open => open(),
    }
}

fn connect(addr: &str) {
    let stream = net::TcpStream::connect(addr).expect("Failed to connect to server");
    let user = User::new_get_name(stream);
    user.start_session();
}

fn open() {
    let url = format!("{}:{}", "0.0.0.0", PORT);
    let socket = net::TcpListener::bind(url).unwrap();

    println!("Waiting to connect...");
    let (stream, addr) = socket.accept().unwrap();

    println!("Connected to {}", addr);
    let user = User::new_get_name(stream);
    user.start_session()
}
