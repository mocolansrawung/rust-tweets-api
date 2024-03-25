// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}
