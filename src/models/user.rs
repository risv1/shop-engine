use crate::schema::users;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}