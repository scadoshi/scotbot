use crate::runner::Runner;

mod anthropic;
mod chat;
mod command;
mod config;
mod runner;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::welcome_message()?;
    let state = chat::State::new().await?;
    Runner::run(state).await?;
    Ok(())
}
