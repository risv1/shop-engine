use diesel::prelude::*;
use crate::database::schema::{order_items};
use serde::Serialize;

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