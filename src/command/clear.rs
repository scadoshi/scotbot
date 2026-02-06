use crate::{command::Command, ui::horizontal_line};

pub struct Clear;

impl Command for Clear {
    fn execute(state: &mut crate::chat::State) -> anyhow::Result<()> {
        horizontal_line();
        println!("Clearing chat history");
        state.clear_history();
        state.clear_input();
        Ok(())
    }
}
