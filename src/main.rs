use new_chat::cmd;
use std::env;

fn main() {
    let mut args = env::args();
    let cmd = cmd::parse_command(&mut args).unwrap_or_else(|err| panic!("{}", err.to_string()));
    match cmd {
        cmd::Command::Connect(addr) => cmd::connect(&addr),
        cmd::Command::Open => cmd::open(),
    }
}
