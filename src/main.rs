mod chat_state;
mod config;
use crate::chat_state::ChatState;
use crate::config::Config;
use rig::completion::Chat;
use rig::message::{AssistantContent, Message, UserContent};
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config = Config::from_env();
    let mut chat = ChatState::new();
    let mut agent = config.build_agent();
    let width = 100;
    println!(
        "Your AI Agent ({}) Preamble: {}",
        config.model(),
        config.preamble()
    );
    println!("{}", "=".repeat(width));
    println!("Type a message and click enter to submit");
    loop {
        println!("{}", "=".repeat(width));
        chat.get_input();
        // ======================================
        //              /exit
        // ======================================
        if chat.input().contains("/exit") {
            if !chat.history().is_empty() {
                let exit_prompt = "The user is requesting to exit with the /exit command. Please bid them farewell in a way that reflects the contents of this chat";
                if let Ok(response) = agent.chat(exit_prompt, chat.history().to_owned()).await {
                    println!("{}", "=".repeat(width));
                    println!("{}: {}", config.model(), response);
                    chat.record_assistant_message(exit_prompt);
                    chat.record_assistant_message(response);
                }
            }
            break;
        }
        // ======================================
        //              /model
        // ======================================
        if chat.input().contains("/model") {
            println!("{}", "=".repeat(width));
            chat.clear_input();
            println!("Current model: {}. Select a model below", config.model());
            for (i, model) in config.model_options().iter().enumerate() {
                println!("{}. {}", i + 1, model);
            }
            println!("{}", "=".repeat(width));
            loop {
                chat.get_input();
                let Some(selection) = config
                    .model_options()
                    .iter()
                    .enumerate()
                    .find(|(i, _)| {
                        chat.input()
                            .trim()
                            .parse::<usize>()
                            .is_ok_and(|u| u - 1 == *i)
                    })
                    .map(|(_, selection)| selection.to_owned())
                else {
                    continue;
                };
                chat.clear_input();
                config.set_model(selection);
                agent = config.build_agent();
                println!("Model updated to: {}", config.model());
                break;
            }
            continue;
        }

        const TRUNCATE_AT: usize = 300;
        const HISTORY_LEN: usize = 10;
        // ======================================
        //             /history
        // ======================================
        if chat.input().contains("/history") {
            if chat.history().is_empty() {
                chat.clear_input();
                println!("{}", "=".repeat(width));
                println!("No chat history to show");
                continue;
            }
            println!("{}", "=".repeat(width));
            println!("Showing last {} messages", HISTORY_LEN);
            println!("{}", "=".repeat(width));
            chat.clear_input();
            let history_to_show: Vec<_> = chat
                .history()
                .iter()
                .rev()
                .take(HISTORY_LEN)
                .rev()
                .collect();
            for (i, message) in history_to_show.iter().enumerate() {
                match message {
                    Message::User { content } => match content.first() {
                        UserContent::Text(text) => {
                            let text = text.text();
                            let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                            let end = if text.len() > TRUNCATE_AT {
                                String::from("...")
                            } else {
                                String::new()
                            };
                            println!("*User*: \"{}{}\"", truncated.trim(), end);
                        }
                        UserContent::Image(_) => println!("*User*: *image*"),
                        UserContent::Audio(_) => println!("*User*: *audio*"),
                        UserContent::Video(_) => println!("*User*: *video*"),
                        UserContent::Document(_) => println!("*User*: *document*"),
                        UserContent::ToolResult(_) => println!("*User*: *tool result*"),
                    },
                    Message::Assistant { content, .. } => match content.first() {
                        AssistantContent::Text(text) => {
                            let text = text.text();
                            let truncated = text.chars().take(TRUNCATE_AT).collect::<String>();
                            let end = if text.len() > TRUNCATE_AT {
                                String::from("...")
                            } else {
                                String::new()
                            };
                            println!("*Assistant*: \"{}{}\"", truncated.trim(), end);
                        }
                        AssistantContent::Image(_) => println!("*Assistant*: *image*"),
                        AssistantContent::ToolCall(_) => println!("*Assistant*: *tool call*"),
                        AssistantContent::Reasoning(_) => println!("*Assistant*: *reasoning*"),
                    },
                }
                if let Some(final_i) = history_to_show.len().checked_sub(1)
                    && i != final_i
                {
                    println!();
                    println!("---");
                }
                println!();
            }
            continue;
        }
        // ======================================
        //              /clear
        // ======================================
        if chat.input() == "/clear" {
            println!("{}", "=".repeat(width));
            println!("Clearing chat history");
            chat.clear_history();
            chat.clear_input();
            continue;
        }
        // ======================================
        //              /help
        // ======================================
        if chat.input() == "/help" {
            chat.clear_input();
            println!("{}", "=".repeat(width));
            println!("Commands: ");
            println!("/model - switch models");
            println!("/history - show last {} messages", HISTORY_LEN);
            println!("/summarize - summarize chat history");
            println!("/clear - clear context");
            println!("/exit - end application");
            continue;
        }
        // ======================================
        //          /summarize
        // ======================================
        if chat.input() == "/summarize" {
            chat.clear_input();
            if !chat.history().is_empty() {
                let summarize_prompt = "The user is requesting a summary of the conversation up to this point with the /summary command. Please respond with a brief summary of the conversation";
                match agent
                    .chat(summarize_prompt, chat.history().to_owned())
                    .await
                {
                    Ok(response) => {
                        println!("{}", "=".repeat(width));
                        println!("{}: {}", config.model(), response);
                        chat.record_assistant_message(summarize_prompt);
                        chat.record_assistant_message(response);
                    }
                    Err(e) => {
                        eprint!("Error: {}", e);
                        println!("Please try again");
                    }
                }
            } else {
                println!("Nothing to summarize");
            }
            continue;
        }
        // ======================================
        //          regular message
        // ======================================
        if chat.input().is_empty() {
            chat.clear_input();
            println!("Type a message and click enter to submit");
            continue;
        }
        let user_input = chat.input().to_owned();
        chat.clear_input();
        match agent.chat(chat.input(), chat.history().to_owned()).await {
            Ok(response) => {
                println!("{}", "=".repeat(width));
                println!("{}: {}", config.model(), response);
                chat.record_user_message(user_input);
                chat.record_assistant_message(response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please try again");
            }
        }
    }
    Ok(())
}
