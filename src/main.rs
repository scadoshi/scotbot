use crate::runner::Runner;

mod chat;
mod command;
mod config;
mod runner;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Runner::run(chat::State::new()).await?;
    Ok(())
}
