use crate::{
    bot::commands::Command,
    db::action::{clear_reminder_time, set_reminder_time},
};
use teloxide::{adaptors::DefaultParseMode, prelude::*, utils::html};

use chrono::naive::NaiveTime;

use crate::db::establish_connection;

type Bot = DefaultParseMode<teloxide::Bot>;

pub async fn reply(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
    let user_id = msg.from.clone().unwrap().id;
    let username = msg.from.clone().unwrap().full_name();
    let mentioned_user = html::user_mention(user_id, &username);

    let conn = &mut establish_connection();
    match command {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                format!("Hi, {mentioned_user}, Welcome to the Exercise Reminder Bot! Please use /settime <code>HH:MM</code> to set the reminder time",
            ))
            .await?;
        }
        Command::SetTime(time) => {
            let parsed_time = NaiveTime::parse_from_str(&time, "%H:%M");
            match parsed_time {
                Ok(parsed_time) => {
                    set_reminder_time(
                        conn,
                        msg.chat.id,
                        user_id,
                        &username,
                        &parsed_time.format("%H:%M").to_string(),
                    );
                    bot.send_message(
                        msg.chat.id,
                        format!("Hi, {mentioned_user}, your reminder time is set to every day at {time}."),
                    )
                    .await?;
                }
                Err(_) => {
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Oh {mentioned_user}, invalid time format, please use <code>HH:MM</code> format"
                        ),
                    )
                    .await?;
                }
            }
        }
        Command::Stop => {
            clear_reminder_time(conn, msg.chat.id, user_id);
            bot.send_message(
                msg.chat.id,
                format!("{mentioned_user}, your reminder was deleted."),
            )
            .await?;
        }
    }
    Ok(())
}
