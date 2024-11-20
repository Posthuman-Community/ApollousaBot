use bot::commands::Command;
use bot::handler::reply;
use teloxide::prelude::*;

mod bot;
mod db;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();
    Command::repl(bot, reply).await;
}
