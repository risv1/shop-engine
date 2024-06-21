use diesel::prelude::*;
use crate::database::schema::{users, cart, orders, order_items};
use serde::Serialize;

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = cart)]
pub struct NewItemToCart {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = cart)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cart {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub status: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}


#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem<'a> {
    pub id: &'a str,
    pub order_id: &'a str,
    pub product_id: &'a str,
    pub quantity: i32,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
}