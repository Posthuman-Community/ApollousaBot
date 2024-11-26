use bot::commands::Command;
use bot::handler::reply;
use scheduler::reminder::schedule_reminders;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use tokio::time::{interval, Duration};

mod bot;
mod db;
mod scheduler;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Arc::new(Bot::from_env().parse_mode(ParseMode::Html));
    let bot_clone = Arc::clone(&bot);

    let command_task = tokio::spawn(async move {
        Command::repl((*bot_clone).clone(), reply).await;
    });

    let schedual_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            schedule_reminders(&bot).await;
        }
    });

    tokio::select! {
        _ = command_task => {},
        _ = schedual_task => {}
    }
}
