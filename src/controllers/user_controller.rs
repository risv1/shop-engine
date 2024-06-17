use actix_web::{HttpResponse};
use crate::models::user::User;

pub async fn get_user() -> HttpResponse {
    let hello_user: User = User {
        id: "1".to_string(),
        name: "John".to_string(),
        email: "john@gmail.com".to_string(),
        password: "password".to_string(),
        role: "user".to_string(),
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(hello_user)
}