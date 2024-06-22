use diesel::prelude::*;
use crate::database::schema::{product};
use serde::Serialize;

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = product)]
pub struct NewProduct {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub description: String,
    pub category_id: String,
    pub image: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub description: String,
    pub category_id: String,
    pub image: String,
    pub created_at: String,
    pub updated_at: String,
}

