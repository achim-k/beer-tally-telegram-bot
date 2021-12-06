use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

mod storage;
use lazy_static::lazy_static;
use std::sync::Mutex;
use storage::{BeerTally, HashMapBeerTally, RegisterPlayerResult};

lazy_static! {
    static ref STORAGE: Mutex<Box<dyn BeerTally + Send + Sync>> =
        Mutex::new(Box::new(HashMapBeerTally::new()));
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "register as a player.")]
    Register(String),
    #[command(description = "unregister as a player.")]
    Unregister,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = cx.update.chat_id();
    let user_id = cx.update.from().unwrap().id;

    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Register(username) => {
            let return_string = match STORAGE
                .lock()
                .unwrap()
                .register_player(chat_id, user_id, &username)
            {
                RegisterPlayerResult::InvalidUsername => {
                    String::from("Invalid username. Only alphanumeric characters are allowed.")
                }
                RegisterPlayerResult::Registered => {
                    format!("Successfully registered as '{}'.", &username)
                }
                RegisterPlayerResult::AlreadyRegistered(existing_name) => {
                    format!("You are already registered as '{}'. Use command /change_name to change your username.", existing_name)
                }
                RegisterPlayerResult::UsernameTaken => {
                    format!("Username '{}' is already taken.", &username)
                }
            };
            cx.answer(return_string).await?
        }
        Command::Unregister => {
            let return_string = match STORAGE.lock().unwrap().unregister_player(chat_id, user_id) {
                Ok(_) => String::from("Successfully unregistered."),
                Err(_) => String::from("You were not registered."),
            };
            cx.answer(return_string).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    let bot_name: String = String::from("Beer tally bot");
    log::info!("Starting {}...", bot_name);

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
