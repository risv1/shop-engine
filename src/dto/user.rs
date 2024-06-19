use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}