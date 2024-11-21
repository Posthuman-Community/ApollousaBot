use std::sync::Arc;

use crate::db::action as operator;
use chrono::Local;
use teloxide::prelude::*;
use teloxide::types::ChatId;

use crate::db::establish_connection;

pub async fn schedule_reminders(bot: &Arc<Bot>) {
    let conn = &mut establish_connection();
    let current_time = Local::now().time().format("%H:%M").to_string();
    println!("{}", current_time);

    let users_to_remind = operator::get_user_reminders(conn, &current_time);

    for user in users_to_remind {
        println!("{}", user.reminder_time);
        bot.send_message(ChatId(user.chat_id), "记得起身活动活动，松松背拉拉筋")
            .await
            .unwrap();
    }
}
