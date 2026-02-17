use crate::{chat, command::show_message_history::HISTORY_LEN, ui::horizontal_line};

pub trait ShowHelpMessage {
    fn show_help_message(&mut self);
}

impl ShowHelpMessage for chat::State {
    fn show_help_message(&mut self) {
        self.clear_input();
        horizontal_line();
        println!("Commands: ");
        println!("/model - switch models");
        println!("/tokens - show token usage");
        println!("/history - show last {} messages", HISTORY_LEN);
        println!("/summarize - summarize chat history");
        println!("/compact - consolidate context");
        println!("/clear - clear context");
        println!("/exit - end application");
    }
}
