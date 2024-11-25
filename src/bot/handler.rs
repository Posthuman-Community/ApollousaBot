use crate::{
    bot::commands::Command,
    db::action::{clear_reminder_time, set_reminder_time, set_user_timezone},
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
        Command::Help => {
            bot.send_message(
                msg.chat.id,
                format!("Hi, {mentioned_user}, Welcome to the Exercise Reminder Bot! 😉\n\nPlease use <code>/settime HH:MM</code> to set the reminder time.\n\nAnd use <code>/settimezone +1:00</code> <b>(according to your location)</b> to accurate the reminder.\n\nIf you are not sure your timezone, please check this <a href=\"https://en.wikipedia.org/wiki/List_of_UTC_offsets\">page</a>.",
                ),
            )
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
        Command::SetTimezone(timezone) => {
            set_user_timezone(conn, user_id, msg.chat.id, &timezone);
            bot.send_message(
                msg.chat.id,
                format!(
                    "Hi, {mentioned_user}, your timezone is set to <code>UTC{timezone}</code>."
                ),
            )
            .await?;
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
