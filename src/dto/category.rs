use diesel::prelude::*;
use crate::database::schema::{category};
use serde::Serialize;

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}