use diesel::prelude::*;
use crate::database::schema::users;
use serde::Serialize;

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}