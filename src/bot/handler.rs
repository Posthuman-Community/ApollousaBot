use crate::bot::commands::Command;
use teloxide::prelude::*;

use chrono::naive::NaiveTime;

pub async fn reply(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
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
                    // TODO 更新数据库时间;
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
            // TODO 从数据库删除提醒时间;
            bot.send_message(msg.chat.id, "The reminder time was deleted.")
                .await?;
        }
    }
    Ok(())
}
