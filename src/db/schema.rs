diesel::table! {
    users (id) {
        id -> Integer,
        chat_id -> BigInt,
        reminder_time -> Text,
        last_reminder_time -> Nullable<Text>
    }
}
