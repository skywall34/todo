// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        creator -> Varchar,
        title -> Varchar,
        body -> Text,
        completed -> Bool,
    }
}
