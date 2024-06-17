// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}
