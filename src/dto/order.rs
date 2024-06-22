use diesel::prelude::*;
use crate::database::schema::{orders};
use serde::Serialize;

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