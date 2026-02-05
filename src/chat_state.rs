use std::io::stdin;

use rig::message::Message;

#[derive(Debug, Clone, Default)]
pub struct ChatState {
    history: Vec<Message>,
    input: String,
}

impl ChatState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_assistant_message(&mut self, message: impl Into<String>) {
        self.history.push(Message::assistant(message.into()));
    }
    pub fn record_user_message(&mut self, message: impl Into<String>) {
        self.history.push(Message::user(message.into()));
    }
    pub fn history(&self) -> &[Message] {
        self.history.as_slice()
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    pub fn input(&self) -> &str {
        self.input.as_str()
    }
    pub fn clear_input(&mut self) {
        self.input.clear();
    }
    pub fn get_input(&mut self) {
        match stdin().read_line(&mut self.input) {
            Ok(_) => self.input = self.input.trim().to_string(),
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Input failed");
                self.clear_input();
            }
        }
    }
}
