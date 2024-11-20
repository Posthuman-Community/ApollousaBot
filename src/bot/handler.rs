use crate::{
    bot::commands::Command,
    db::action::{clear_reminder_time, set_reminder_time},
};
use teloxide::prelude::*;

use chrono::naive::NaiveTime;

use crate::db::establish_connection;

pub async fn reply(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
    let conn = &mut establish_connection();
    match command {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "Welcome to the Exercise Reminder Bot! Please use /settime HH:MM to set the reminder time.",
            )
            .await?;
        }
        Command::SetTime(time) => {
            let parsed_time = NaiveTime::parse_from_str(&time, "%H:%M");
            match parsed_time {
                Ok(parsed_time) => {
                    set_reminder_time(conn, msg.chat.id, &parsed_time.format("%H:%M").to_string());
                    bot.send_message(
                        msg.chat.id,
                        format!("The reminder time is set to every day at {time}。"),
                    )
                    .await?;
                }
                Err(_) => {
                    bot.send_message(msg.chat.id, "Invalid time format, please use HH:MM format")
                        .await?;
                }
            }
        }
        Command::Stop => {
            clear_reminder_time(conn, msg.chat.id);
            bot.send_message(msg.chat.id, "The reminder time was deleted.")
                .await?;
        }
    }
    Ok(())
}
