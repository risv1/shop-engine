// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}
