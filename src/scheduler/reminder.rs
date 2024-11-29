use std::sync::Arc;

use crate::db::action as operator;
use crate::db::model::Users;
use crate::utils::{get_random_quote, TimezoneOffest};
use chrono::{FixedOffset, Utc};
use teloxide::adaptors::DefaultParseMode;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::utils::html;

use crate::db::establish_connection;

type Bot = DefaultParseMode<teloxide::Bot>;

async fn handle_user_reminder(
    user: &Users,
    bot: &Arc<Bot>,
    selected_quote: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let tzoffset = user.tz_offset.as_ref().ok_or(format!(
        "User {} does not have a timezone offset set.",
        user.username
    ))?;
    let user_timezone = tzoffset
        .parse::<TimezoneOffest>()
        .map_err(|e| format!("Failed to parse timezone offset: {}", e))?;
    let duration_secs = user_timezone.to_duration();
    let user_utc_time = FixedOffset::east_opt(duration_secs).map_or(
        "Invalid timezone offset".to_string(),
        |offset| {
            Utc::now()
                .with_timezone(&offset)
                .format("%H:%M")
                .to_string()
        },
    );

    if let Some(reminder) = &user.reminder_time {
        if user_utc_time == *reminder {
            let _user_id = UserId(u64::try_from(user.user_id)?);
            let mentioned_user = html::user_mention(_user_id, &user.username);
            let notification = format!("{mentioned_user} {selected_quote}");
            bot.send_message(ChatId(user.chat_id), notification).await?;
        }
    }

    Ok(())
}

pub async fn schedule_reminders(bot: &Arc<Bot>) {
    let conn = &mut establish_connection();
    let users_to_remind = operator::get_user_reminders(conn);

    let all_quotes = operator::get_quotes(conn);
    let selected_quote =
        get_random_quote(&all_quotes).unwrap_or("记得起身活动活动，松松背拉拉筋。");

    for user in users_to_remind {
        if let Err(e) = handle_user_reminder(&user, bot, selected_quote).await {
            eprintln!("Error handling reminder for user {}: {}", user.username, e);
        }
    }
}
