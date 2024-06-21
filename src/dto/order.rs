use serde::{Serialize};

#[derive(Serialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub cart_id: String,
    pub total: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}