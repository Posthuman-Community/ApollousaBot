use bot::commands::Command;
use bot::handler::reply;
use scheduler::reminder::schedule_reminders;
use std::sync::Arc;
use teloxide::prelude::*;
use tokio::time::{interval, Duration};

mod bot;
mod db;
mod scheduler;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Arc::new(Bot::from_env());
    let bot_clone = Arc::clone(&bot);

    tokio::spawn(async move {
        Command::repl((*bot_clone).clone(), reply).await;
    });

    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        schedule_reminders(&bot).await;
    }
}
