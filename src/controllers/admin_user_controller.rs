use actix_web::{HttpRequest, HttpResponse, Responder, get, post, put, delete, web::{Json, Path}};
use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl, TextExpressionMethods};
use serde::{Deserialize};
use crate::database::conn::establish_connection;
use crate::dto::user::User;
use serde_json::json;

pub async fn get_users(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::users::dsl::*;

    let conn = &mut establish_connection();

    let all_users = users
        .load(conn)
        .expect("Error finding users");

    if all_users.len() == 0 {
        return HttpResponse::NotFound().json(json!({
            "message": "Users not found"
        }));
    }

    let get_users: &Vec<User> = &all_users;

    HttpResponse::Ok().json(json!({
        "message": "Users found",
        "users": get_users
    }))
}

pub async fn get_user(path: Path<String> ,req: HttpRequest) -> HttpResponse {
    use crate::database::schema::users::dsl::*;

    let get_user_id = path.into_inner();
    let conn = &mut establish_connection();

    let user = users
        .filter(id.eq(get_user_id))
        .load(conn)
        .expect("Error finding user");

    if user.len() == 0 {
        return HttpResponse::NotFound().json(json!({
            "message": "User not found"
        }));
    }

    let get_user: &Vec<User> = &user;

    HttpResponse::Ok().json(json!({
        "message": "User found",
        "user": get_user
    }))

}