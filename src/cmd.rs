use std::env;

pub enum Command {
    Connect(String),
    Open,
}

pub enum InputError {
    InvalidCmd(String),
    NoCommand,
}

impl InputError {
    pub fn to_string(&self) -> String {
        match self {
            InputError::InvalidCmd(msg) => msg.to_string(),
            InputError::NoCommand => "No command given".to_string(),
        }
    }
}

pub fn parse_command(input: &mut env::Args) -> Result<Command, InputError> {
    input.next();

    match input.next().as_deref() {
        Some("open") => Ok(Command::Open),
        Some("connect") => {
            let host = input.next().ok_or(InputError::InvalidCmd(
                "connect requires IP address".to_string(),
            ))?;

            Ok(Command::Connect(host))
        }

        Some(cmd) => Err(InputError::InvalidCmd(format!("invalid command: {}", cmd))),
        None => Err(InputError::NoCommand),
    }
}
