diesel::table! {
    todos (
        id) {
            id -> Integer,
            title -> Text,
            completed -> Bool,
            created_at -> Timestamp,
            updated_at -> Timestamp,
    }
}