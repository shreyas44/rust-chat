use std::{
    io::{self, BufRead, Write},
    net,
    sync::{Arc, Mutex},
    thread,
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::Stylize,
    terminal, Command, ExecutableCommand,
};

use crate::utils::{backspace, get_formatted_time, input, print, println};

pub struct User {
    name: String,
    stream: net::TcpStream,
    current_msg: Arc<Mutex<String>>,
}

impl User {
    pub fn new_get_name(stream: net::TcpStream) -> User {
        let name = input("Enter your name: ");
        let current_msg = Arc::new(Mutex::new(String::new()));
        User {
            name,
            stream,
            current_msg,
        }
    }

    fn send(&self, msg: &str) {
        let msg = format!("{}: {}\n", self.name.clone().red(), msg);
        let mut stream = self.stream.try_clone().unwrap();

        stream.write(msg.as_bytes()).unwrap();
    }

    fn watch_output(&self) {
        let mut reader = io::BufReader::new(&self.stream);

        loop {
            let mut msg = String::new();
            reader.read_line(&mut msg).unwrap();
            io::stdout()
                .execute(terminal::Clear(terminal::ClearType::CurrentLine))
                .unwrap();

            if msg.len() == 0 {
                break;
            }

            notify_rust::Notification::new().body(&msg).show().unwrap();
            let time = get_formatted_time().red();
            print(&format!("\r[{}] {}", time, msg));
            self.print_current_msg();
        }
    }

    fn watch_input(&self) {
        fn move_cursor(cmd: impl Command) {
            io::stdout().execute(cmd).unwrap();
        }

        loop {
            self.print_current_msg();
            let key_code;

            match read().unwrap() {
                Event::Key(KeyEvent { code, .. }) => key_code = code,
                _ => continue,
            }

            let mut current_msg = self.current_msg.lock().unwrap();
            match key_code {
                KeyCode::Char(code) => current_msg.push(code),
                KeyCode::Left => move_cursor(cursor::MoveLeft(1)),
                KeyCode::Right => move_cursor(cursor::MoveRight(1)),
                KeyCode::Backspace => {
                    backspace();
                    current_msg.pop();
                }
                KeyCode::Enter if current_msg.to_string() == "exit" => break,
                KeyCode::Enter if current_msg.len() == 0 => continue,
                KeyCode::Enter => {
                    drop(current_msg); // unlock mutex
                    self.print_current_complete_msg();
                    println("");

                    let mut current_msg = self.current_msg.lock().unwrap();
                    self.send(&current_msg);
                    current_msg.clear();
                }
                _ => continue,
            }
        }
    }

    fn print_current_msg(&self) {
        let msg = self.current_msg.lock().unwrap();
        let msg = format!("{}: {}", self.name.clone().green(), msg);
        print(&msg);
    }

    fn print_current_complete_msg(&self) {
        let msg = self.current_msg.lock().unwrap();
        let name = self.name.clone().green();
        let time = get_formatted_time().green();

        let msg = format!("[{}] {}: {}", time, name, msg);
        print(&msg);
    }

    pub fn start_session(self) {
        terminal::enable_raw_mode().unwrap();

        let (close_tx, close_rx) = std::sync::mpsc::channel::<bool>();
        let output_clone = self.clone();
        let input_clone = self.clone();

        let close_tx_clone = close_tx.clone();
        thread::spawn(move || {
            output_clone.watch_output();
            close_tx_clone.send(true).unwrap();
        });

        thread::spawn(move || {
            input_clone.watch_input();
            close_tx.send(true).unwrap();
        });

        close_rx.recv().unwrap(); // wait for one of them to exit;
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            current_msg: Arc::clone(&self.current_msg),
            name: self.name.clone(),
            stream: self.stream.try_clone().unwrap(),
        }
    }
}

impl Drop for User {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
