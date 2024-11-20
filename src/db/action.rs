use crate::db::schema::users::dsl::*;
use diesel::prelude::*;
use teloxide::types::ChatId;

pub fn set_reminder_time(conn: &mut SqliteConnection, _chat_id: ChatId, time: &str) {
    diesel::insert_into(users)
        .values((chat_id.eq(_chat_id.0), reminder_time.eq(time)))
        .on_conflict(chat_id)
        .do_update()
        .set(reminder_time.eq(time))
        .execute(conn)
        .expect("Error on setting reminder time");
}

pub fn clear_reminder_time(conn: &mut SqliteConnection, _chat_id: ChatId) {
    diesel::delete(users.filter(chat_id.eq(_chat_id.0)))
        .execute(conn)
        .expect("Error on clearing reminder time");
}
