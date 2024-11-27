// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Integer,
        msg -> Text,
    }
}

diesel::table! {
    users (chat_id) {
        chat_id -> BigInt,
        user_id -> BigInt,
        username -> Text,
        reminder_time -> Text,
        tz_offset -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    quotes,
    users,
);
