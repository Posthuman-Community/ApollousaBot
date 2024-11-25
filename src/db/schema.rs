// @generated automatically by Diesel CLI.

diesel::table! {
    timezone (user_id) {
        user_id -> BigInt,
        chat_id -> BigInt,
        user_timezone -> Text,
    }
}

diesel::table! {
    users (chat_id) {
        chat_id -> BigInt,
        user_id -> BigInt,
        username -> Text,
        reminder_time -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    timezone,
    users,
);
