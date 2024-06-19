use serde::{Serialize};

#[derive(Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: String,
    pub updated_at: String,
}