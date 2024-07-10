use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl, TextExpressionMethods};
use crate::database::conn::establish_connection;
use crate::dto::user::User;
use crate::utils::jwt::decode_jwt_token;

// upgrade to middleware interceptor instead of a manual check on every endpoint.

pub fn check_admin(token: String) -> bool { 
    use crate::database::schema::users::dsl::*;

    let user_id = decode_jwt_token(&token);

    let conn = &mut establish_connection();

    let user: Vec<User> = users
        .filter(id.eq(&user_id))
        .limit(1)
        .load(conn)
        .expect("Error finding user");

    if user.len() == 0 {
        return false;
    }

    if user[0].role == "admin" {
        return true;
    }

    return false;
} 