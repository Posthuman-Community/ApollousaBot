// @generated automatically by Diesel CLI.

diesel::table! {
    users (chat_id) {
        chat_id -> BigInt,
        user_id -> BigInt,
        username -> Text,
        reminder_time -> Text,
        tz_offset -> Nullable<Text>,
    }
}
