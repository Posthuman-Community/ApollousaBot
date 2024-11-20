// @generated automatically by Diesel CLI.

diesel::table! {
    users (chat_id) {
        chat_id -> BigInt,
        reminder_time -> Text,
        last_reminder_time -> Nullable<Text>,
    }
}
