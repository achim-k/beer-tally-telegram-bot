use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "select someone who has to pay the beers.")]
    PayTheBeers(String),
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::PayTheBeers(username) => {
            cx.answer(format!("{} pays the beers 🍻", username)).await?
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