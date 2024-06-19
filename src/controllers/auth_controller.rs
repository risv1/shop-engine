use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Json;
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::database::models::{NewUser, User};
use crate::database::conn::establish_connection;
use crate::utils::hash::{hash_password, verify_password};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use dotenvy::dotenv;
use std::env;
use chrono;

#[derive(Deserialize)]
pub struct RegisterDto {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register(user: Json<RegisterDto>) -> HttpResponse {
    
    use crate::database::schema::users;

    let conn = &mut establish_connection();
 
    let new_user = NewUser {
        id: &Uuid::new_v4().to_string(),
        name: &user.name,
        email: &user.email,
        password: &hash_password(&user.password),
        created_at: &chrono::Utc::now().to_rfc3339(),
        updated_at: &chrono::Utc::now().to_rfc3339(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json(json!({
        "message": "User created successfully",
        "data": new_user
    }))
}

pub async fn login(login_data: Json<LoginDto>) -> HttpResponse {
    use crate::database::schema::users::dsl::*;

    dotenv().ok();
    let conn = &mut establish_connection();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let user = users
        .filter(email.eq(&login_data.email))
        .limit(1)
        .load(conn)
        .expect("Error finding user");

    if user.len() == 0 {
        return HttpResponse::Unauthorized().json(json!({
            "message": "Invalid email or password"
        }));
    }

    let checked_user: &User = &user[0];

    if !verify_password(&login_data.password, &checked_user.password) {
        return HttpResponse::Unauthorized().json(json!({
            "message": "Invalid email or password"
        }));
    }

    let expiration = &chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60 * 60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: checked_user.id.clone(),
        exp: *expiration as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .expect("Error encoding token");

    let cookie = Cookie::build("token", token)
        .http_only(true)
        .max_age(Duration::seconds(3600))
        .finish();

    HttpResponse::Ok().cookie(cookie).json(json!({
        "message": "Login successful",
    }))
}

pub async fn logout(req: HttpRequest) -> HttpResponse {
    if let Some(_cookie) = req.cookie("token") {
        let cookie = Cookie::build("token", "")
            .http_only(true)
            .max_age(Duration::seconds(0))
            .finish();
        HttpResponse::Ok().cookie(cookie).json(json!({
            "message": "Logout successful"
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}