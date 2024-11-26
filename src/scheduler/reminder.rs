use std::sync::Arc;

use crate::db::action as operator;
use crate::utils::TimezoneOffest;
use chrono::{FixedOffset, Utc};
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::utils::html;

use crate::db::establish_connection;

type Bot = DefaultParseMode<teloxide::Bot>;

pub async fn schedule_reminders(bot: &Arc<Bot>) {
    let conn = &mut establish_connection();
    let users_to_remind = operator::get_user_reminders(conn);

    for user in users_to_remind {
        if let Some(tzoffset) = user.tz_offset.as_ref() {
            let user_timezone: TimezoneOffest = tzoffset.parse().unwrap();

            let duration_secs = user_timezone.to_duration();
            let user_utc_time = Utc::now()
                .with_timezone(&FixedOffset::east_opt(duration_secs).unwrap())
                .format("%H:%M")
                .to_string();

            // println!("User UTC time: {}", user_utc_time);

            if user_utc_time == user.reminder_time {
                // println!("{}: {}", user.reminder_time, user.username);
                let _user_id = UserId(u64::try_from(user.user_id).unwrap());
                let _username = user.username;
                let mentioned_user = html::user_mention(_user_id, &_username);
                let notification = format!("{mentioned_user}，记得起身活动活动，松松背拉拉筋。");
                bot.send_message(ChatId(user.chat_id), notification)
                    .await
                    .unwrap();
            }
        }
    }
}
