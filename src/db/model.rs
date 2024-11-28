use crate::db::schema::{quotes, users};
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub chat_id: i64,
    pub user_id: i64,
    pub username: String,
    pub reminder_time: Option<String>,
    pub tz_offset: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = quotes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Quotes {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub msg: String,
}
