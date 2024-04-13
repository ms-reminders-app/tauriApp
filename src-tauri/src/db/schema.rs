// @generated automatically by Diesel CLI.

diesel::table! {
    reminders (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        completed -> Bool,
        due -> Text,
        reminder -> Float,
    }
}
