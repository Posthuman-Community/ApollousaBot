use std::sync::Arc;

use crate::db::action as operator;
use chrono::Local;
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::utils::html;

use crate::db::establish_connection;

type Bot = DefaultParseMode<teloxide::Bot>;

pub async fn schedule_reminders(bot: &Arc<Bot>) {
    let conn = &mut establish_connection();
    let current_time = Local::now().time().format("%H:%M").to_string();

    let users_to_remind = operator::get_user_reminders(conn, &current_time);

    for user in users_to_remind {
        println!("{}", user.reminder_time);
        let _user_id = UserId(u64::try_from(user.user_id).unwrap());
        let _username = user.username;
        let mentioned_user = html::user_mention(_user_id, &_username);
        let notification = format!("{mentioned_user}，记得起身活动活动，松松背拉拉筋。");
        bot.send_message(ChatId(user.chat_id), notification)
            .await
            .unwrap();
    }
}
