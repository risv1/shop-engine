// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Text,
        user_id -> Text,
        product_id -> Text,
        quantity -> Int4,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    category (id) {
        id -> Text,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    order_items (id) {
        id -> Text,
        order_id -> Text,
        product_id -> Text,
        quantity -> Int4,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    orders (id) {
        id -> Text,
        user_id -> Text,
        status -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    product (id) {
        id -> Text,
        name -> Text,
        price -> Float8,
        stock -> Int4,
        description -> Text,
        category_id -> Text,
        image -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    products (id) {
        id -> Text,
        name -> Text,
        price -> Numeric,
        stock -> Int4,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(cart -> product (product_id));
diesel::joinable!(cart -> users (user_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> product (product_id));
diesel::joinable!(product -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    category,
    order_items,
    orders,
    product,
    products,
    users,
);
