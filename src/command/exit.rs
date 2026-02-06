use crate::{chat, command::Command};

pub struct Exit;

impl Command for Exit {
    fn execute(_: &mut chat::State) -> anyhow::Result<()> {
        println!("Farewell!");
        Ok(())
    }
}
