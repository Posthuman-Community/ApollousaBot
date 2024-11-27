use std::sync::Arc;

use crate::db::action as operator;
use crate::utils::{get_random_quote, TimezoneOffest};
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

    let all_quotes = operator::get_quotes(conn);
    let random_quote = get_random_quote(&all_quotes);
    let selected_quote = random_quote.unwrap_or("记得起身活动活动，松松背拉拉筋。");

    for user in users_to_remind {
        if let Some(tzoffset) = user.tz_offset.as_ref() {
            let user_timezone: TimezoneOffest = tzoffset.parse().unwrap();

            let duration_secs = user_timezone.to_duration();
            let user_utc_time = Utc::now()
                .with_timezone(&FixedOffset::east_opt(duration_secs).unwrap())
                .format("%H:%M")
                .to_string();

            if user_utc_time == user.reminder_time {
                let _user_id = UserId(u64::try_from(user.user_id).unwrap());
                let _username = user.username;
                let mentioned_user = html::user_mention(_user_id, &_username);
                let notification = format!("{mentioned_user} {selected_quote}");
                bot.send_message(ChatId(user.chat_id), notification)
                    .await
                    .unwrap();
            }
        }
    }
}
