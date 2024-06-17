use crate::models::product::Product;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub items: Vec<Product>,
    pub total: f64,
    pub created_at: String,
    pub updated_at: String,
}