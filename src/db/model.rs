use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub id: i32,
    pub chat_id: i64,
    pub reminder_time: String,
    pub last_reminder_time: Option<String>,
}
