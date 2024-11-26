use crate::db::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub chat_id: i64,
    pub user_id: i64,
    pub username: String,
    pub reminder_time: String,
    pub tz_offset: Option<String>,
}
