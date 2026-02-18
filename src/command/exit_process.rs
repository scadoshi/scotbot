use crate::{chat, command::save_chat_history::SaveChatHistory, ui::horizontal_line};

pub trait ExitProcess {
    fn exit_process(&mut self) -> anyhow::Result<()>;
}

impl ExitProcess for chat::State {
    fn exit_process(&mut self) -> anyhow::Result<()> {
        self.save_chat_history()?;
        horizontal_line();
        println!("Farewell!");
        Ok(())
    }
}
