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
    _user_timezone: &str,
) {
    println!(
        "UserId: {}, ChatId: {}, UserTimezone: {}",
        _user_id.0, _chat_id.0, _user_timezone
    );
    diesel::update(
        users.filter(
            chat_id
                .eq(_chat_id.0)
                .and(user_id.eq(i64::try_from(_user_id.0).unwrap())),
        ),
    )
    .set(tz_offset.eq(_user_timezone))
    .execute(conn)
    .expect("Error update user timezone");
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
    users.load::<Users>(conn).expect("Error loading user")
}
