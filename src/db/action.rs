use crate::db::schema::timezone::dsl::{
    chat_id as tz_chat_id, timezone, user_id as tz_user_id, user_timezone,
};
use crate::db::schema::users::dsl::{
    chat_id as u_chat_id, reminder_time, user_id as u_user_id, username, users,
};
use diesel::prelude::*;
use teloxide::types::{ChatId, UserId};

use super::model::{UserTimezone, Users};

pub fn set_reminder_time(
    conn: &mut SqliteConnection,
    _chat_id: ChatId,
    _user_id: UserId,
    _username: &str,
    time: &str,
) {
    diesel::insert_into(users)
        .values((
            u_chat_id.eq(_chat_id.0),
            u_user_id.eq(i64::try_from(_user_id.0).unwrap()),
            username.eq(_username),
            reminder_time.eq(time),
        ))
        .on_conflict(u_chat_id)
        .do_update()
        .set(reminder_time.eq(time))
        .execute(conn)
        .expect("Error on setting reminder time");
}

pub fn set_user_timezone(
    conn: &mut SqliteConnection,
    _user_id: UserId,
    _chat_id: ChatId,
    _user_timezone: &str,
) {
    println!(
        "UserId: {}, ChatId: {}, UserTimezone: {}",
        _user_id.0, _chat_id.0, _user_timezone
    );
    diesel::insert_into(timezone)
        .values((
            tz_user_id.eq(i64::try_from(_user_id.0).unwrap()),
            tz_chat_id.eq(_chat_id.0),
            user_timezone.eq(_user_timezone),
        ))
        .on_conflict(tz_user_id)
        .do_update()
        .set(user_timezone.eq(_user_timezone))
        .execute(conn)
        .expect("Error on setting reminder time");
}

pub fn clear_reminder_time(conn: &mut SqliteConnection, _chat_id: ChatId, _user_id: UserId) {
    diesel::delete(
        users.filter(
            u_chat_id
                .eq(_chat_id.0)
                .and(u_user_id.eq(i64::try_from(_user_id.0).unwrap())),
        ),
    )
    .execute(conn)
    .expect("Error on clearing reminder time");

    diesel::delete(
        timezone.filter(
            tz_chat_id
                .eq(_chat_id.0)
                .and(tz_user_id.eq(i64::try_from(_user_id.0).unwrap())),
        ),
    )
    .execute(conn)
    .expect("Error on clearing timezone");
}

pub fn get_user_reminders(conn: &mut SqliteConnection, t_now: &str) -> Vec<Users> {
    users
        .filter(reminder_time.eq(t_now))
        .load::<Users>(conn)
        .expect("Error loading user")
}
