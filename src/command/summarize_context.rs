use rig::message::Message;

use crate::chat;

pub trait SummarizeContext {
    fn summarize_context(&mut self) -> impl Future<Output = anyhow::Result<()>>;
}

impl SummarizeContext for chat::State {
    async fn summarize_context(&mut self) -> anyhow::Result<()> {
        self.clear_input();
        if self.history().is_empty() {
            println!("Nothing to summarize");
        } else {
            let prompt = "Summarize our conversation so far in 2-4 sentences. Focus on the key topics discussed and any conclusions reached.";
            self.stream(Message::user(prompt)).await;
        }
        Ok(())
    }
}
