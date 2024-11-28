use crate::db::schema::quotes::dsl::*;
use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use teloxide::types::{ChatId, UserId};

use super::model::Users;

pub fn set_reminder_time(
    conn: &mut SqliteConnection,
    _chat_id: ChatId,
    _user_id: UserId,
    _username: &str,
    time: &str,
) {
    println!(
        "set_reminder_time -> UserId: {}, ChatId: {}, Username: {} , time: {}",
        _user_id.0, _chat_id.0, _username, time
    );
    diesel::insert_into(users)
        .values((
            chat_id.eq(_chat_id.0),
            user_id.eq(i64::try_from(_user_id.0).unwrap()),
            username.eq(_username),
            reminder_time.eq(time),
        ))
        .on_conflict(chat_id)
        .do_update()
        .set(reminder_time.eq(time))
        .execute(conn)
        .expect("Error on setting reminder time");
}

pub fn set_user_timezone(
    conn: &mut SqliteConnection,
    _user_id: UserId,
    _chat_id: ChatId,
    _username: &str,
    _user_timezone: &str,
) {
    println!(
        "set_user_timezone -> UserId: {}, ChatId: {}, Username: {} ,UserTimezone: {}",
        _user_id.0, _chat_id.0, _username, _user_timezone
    );
    let user_exists = users
        .filter(
            chat_id
                .eq(_chat_id.0)
                .and(user_id.eq(i64::try_from(_user_id.0).unwrap())),
        )
        .select((chat_id, user_id))
        .first::<(i64, i64)>(conn)
        .optional()
        .expect("Error checking if user exists");

    if user_exists.is_some() {
        diesel::update(
            users.filter(
                chat_id
                    .eq(_chat_id.0)
                    .and(user_id.eq(i64::try_from(_user_id.0).unwrap())),
            ),
        )
        .set(tz_offset.eq(_user_timezone))
        .execute(conn)
        .expect("Error updating user timezone");
    } else {
        diesel::insert_into(users)
            .values((
                chat_id.eq(_chat_id.0),
                user_id.eq(i64::try_from(_user_id.0).unwrap()),
                username.eq(_username),
                tz_offset.eq(_user_timezone),
            ))
            .execute(conn)
            .expect("Error inserting new user with timezone");
    }
}

pub fn clear_reminder_time(conn: &mut SqliteConnection, _chat_id: ChatId, _user_id: UserId) {
    diesel::delete(
        users.filter(
            chat_id
                .eq(_chat_id.0)
                .and(user_id.eq(i64::try_from(_user_id.0).unwrap())),
        ),
    )
    .execute(conn)
    .expect("Error on clearing reminder time");
}

pub fn get_user_reminders(conn: &mut SqliteConnection) -> Vec<Users> {
    // Only work for group members
    users
        .select((chat_id, user_id, username, reminder_time, tz_offset))
        .filter(chat_id.ne(user_id))
        .load::<Users>(conn)
        .expect("Error loading user")
}

pub fn get_quotes(conn: &mut SqliteConnection) -> Vec<String> {
    quotes
        .select(msg)
        .load::<String>(conn)
        .expect("Error loading Quotes")
}
