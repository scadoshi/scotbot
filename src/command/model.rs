use crate::{
    chat::{self, input::Input},
    command::Command,
    ui::horizontal_line,
};

pub struct Model;

impl Command for Model {
    fn execute(state: &mut chat::State) -> anyhow::Result<()> {
        horizontal_line();
        state.clear_input();
        println!("Current model: {}", state.model());
        for (i, model) in state.model_options().iter().enumerate() {
            println!("{}. {}", i + 1, model);
        }
        horizontal_line();
        loop {
            println!("Select a model");
            state.get_input();
            match state.input() {
                Input::Message(message) => {
                    let Some(selection) = state
                        .model_options()
                        .iter()
                        .enumerate()
                        .find(|(i, _)| {
                            message
                                .parse::<usize>()
                                .ok()
                                .and_then(|u| u.checked_sub(1))
                                .is_some_and(|u| u == *i)
                        })
                        .map(|(_, selection)| selection.to_owned())
                    else {
                        continue;
                    };
                    state.clear_input();
                    state.set_model(selection);
                    state.refresh_agent();
                    println!("Model updated: {}", state.model());
                    break;
                }
                _ => break, // any other command should go to main loop for triaging
            }
        }
        Ok(())
    }
}
