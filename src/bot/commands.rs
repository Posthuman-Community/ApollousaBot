use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    Start,
    SetTime(String),
    Stop,
}