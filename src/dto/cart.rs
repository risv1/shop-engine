use diesel::prelude::*;
use crate::database::schema::{cart};
use serde::Serialize;

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