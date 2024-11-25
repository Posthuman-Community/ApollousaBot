use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub chat_id: i64,
    pub user_id: i64,
    pub username: String,
    pub reminder_time: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::timezone)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserTimezone {
    pub chat_id: i64,
    pub user_id: i64,
    pub user_timezone: String,
}
