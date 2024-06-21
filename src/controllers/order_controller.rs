use actix_web::{web::{Json,Path}, HttpRequest, HttpResponse};
use crate::database::models::{NewItemToCart, Cart, NewOrderItem, OrderItem, NewOrder, Order};
use crate::database::conn::establish_connection;
use crate::utils::jwt::decode_jwt_token;
use uuid::Uuid;
use serde::{Deserialize};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods};
use serde_json::json;
use chrono;

#[derive(Deserialize)]
pub struct NewItem {
    product_id: String,
    quantity: i32,
}

#[derive(Deserialize)]
pub struct DeleteItem {
    item_id: String,
}

pub async fn get_cart(req: HttpRequest) -> HttpResponse {

    use crate::database::schema::cart::dsl::*;

    if let Some(_cookie) = req.cookie("token") {

        let token = req.cookie("token").unwrap();

        let token_user_id = decode_jwt_token(&token.value());

        let conn = &mut establish_connection();

        let user_cart = cart
            .filter(user_id.eq(token_user_id))
            .load(conn)
            .expect("Error finding cart");

        if user_cart.len() == 0 {
            return HttpResponse::NotFound().json(json!({
                "message": "Cart not found"
            }));
        }

        let get_user_cart: &Vec<Cart> = &user_cart;

        HttpResponse::Ok().json(json!({
            "message": "Cart found",
            "cart": get_user_cart
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}   

pub async fn add_to_cart(req: HttpRequest, cart_data: Json<NewItem>) -> HttpResponse {

    use crate::database::schema::cart;

    if let Some(_cookie) = req.cookie("token") {
        let token = req.cookie("token").unwrap();

        let token_user_id = decode_jwt_token(&token.value());

        let conn = &mut establish_connection();

        let new_item = NewItemToCart {
            id: Uuid::new_v4().to_string(),
            user_id: token_user_id,
            product_id: cart_data.product_id.clone(),
            quantity: cart_data.quantity.clone(),
            created_at: chrono::Utc::now().to_string(),
            updated_at: chrono::Utc::now().to_string(),
        };

        diesel::insert_into(cart::table)
            .values(&new_item)
            .get_result::<Cart>(conn)
            .expect("Error saving new item to cart");

        HttpResponse::Ok().json(json!({
            "message": "Item added to cart",
            "data": new_item
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}

pub async fn delete_from_cart(req: HttpRequest, delete_item: Json<DeleteItem>) -> HttpResponse {
    use crate::database::schema::cart::dsl::*;

    if let Some(_cookie) = req.cookie("token") {

        let conn = &mut establish_connection();

        diesel::delete(cart.filter(id.like(delete_item.item_id.clone())))
            .execute(conn)
            .expect("Error deleting item from cart");

        HttpResponse::Ok().json(json!({
            "message": "Item deleted from cart"
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    
    }
}


pub async fn get_orders(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::orders::dsl::*;

    if let Some(_cookie) = req.cookie("token") {
        let token = req.cookie("token").unwrap();
        let conn = &mut establish_connection();

        let user_orders = orders
            .load(conn)
            .expect("Error finding orders");

        if user_orders.len() == 0 {
            return HttpResponse::NotFound().json(json!({
                "message": "Orders not found"
            }));
        }

        let get_user_orders: &Vec<Order> = &user_orders;

        HttpResponse::Ok().json(json!({
            "message": "Orders found",
            "orders": get_user_orders
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}

pub async fn create_order(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::cart::dsl::*;
    use crate::database::schema::{orders, order_items, cart};

    if let Some(_cookie) = req.cookie("token") {
        let token = req.cookie("token").unwrap();
        let conn = &mut establish_connection();

        let token_user_id = decode_jwt_token(&token.value());
        let order_id = Uuid::new_v4().to_string();

        diesel::insert_into(orders::table)
            .values(NewOrder {
                id: &order_id.clone(),
                user_id: &token_user_id,
                status: "pending",
                created_at: &chrono::Utc::now().to_string(),
                updated_at: &chrono::Utc::now().to_string(),
            })
            .get_result::<Order>(conn)
            .expect("Error saving new order");

        let get_cart_items = cart
        .filter(user_id.eq(token_user_id.clone()))
        .load(conn)
        .expect("Error finding cart");

        let cart_items: &Vec<Cart> = &get_cart_items;
        
        for item in cart_items {
            diesel::insert_into(order_items::table)
                .values(NewOrderItem {
                    id: &Uuid::new_v4().to_string(),
                    order_id: &order_id.clone(),
                    product_id: &item.product_id,
                    quantity: item.quantity,
                    created_at: &chrono::Utc::now().to_string(),
                    updated_at: &chrono::Utc::now().to_string(),
                })
                .get_result::<OrderItem>(conn)
                .expect("Error saving new item to order items");
        }

        diesel::delete(cart::table.filter(user_id.eq(token_user_id)))
            .execute(conn)
            .expect("Error deleting items from cart");


        HttpResponse::Ok().json(json!({
            "message": "Order created",
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}

pub async fn get_order(path: Path<String>, req: HttpRequest) -> HttpResponse {
    use crate::database::schema::orders::dsl::*;

    if let Some(_cookie) = req.cookie("token") {
        let get_order_id = path.into_inner();
        let conn = &mut establish_connection();

        let order = orders
            .filter(id.eq(get_order_id))
            .load(conn)
            .expect("Error finding order");

        if order.len() == 0 {
            return HttpResponse::NotFound().json(json!({
                "message": "Order not found"
            }));
        }

        let get_order: &Vec<Order> = &order;

        HttpResponse::Ok().json(json!({
            "message": "Order found",
            "order": get_order
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}

pub async fn get_user_order(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::orders::dsl::*;

    if let Some(_cookie) = req.cookie("token") {
        let token = req.cookie("token").unwrap();
        let conn = &mut establish_connection();

        let token_user_id = decode_jwt_token(&token.value());

        let user_orders = orders
            .filter(user_id.eq(token_user_id))
            .load(conn)
            .expect("Error finding orders");

        if user_orders.len() == 0 {
            return HttpResponse::NotFound().json(json!({
                "message": "Orders not found"
            }));
        }

        let get_user_orders: &Vec<Order> = &user_orders;

        HttpResponse::Ok().json(json!({
            "message": "Orders found",
            "orders": get_user_orders
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}

pub async fn delete_user_order(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::orders::dsl::*;

    if let Some(_cookie) = req.cookie("token") {
        let token = req.cookie("token").unwrap();
        let conn = &mut establish_connection();

        let token_user_id = decode_jwt_token(&token.value());

        let get_order: Vec<Order> = orders
            .filter(user_id.eq(token_user_id.clone()))
            .load(conn)
            .expect("Error finding order");

        let order_to_delete: &Order = &get_order[0];

        if order_to_delete.id.is_empty() {
            return HttpResponse::NotFound().json(json!({
                "message": "Order not found"
            }));
        }

        if order_to_delete.status == "out for delivery" || order_to_delete.status == "delivered" {
            return HttpResponse::BadRequest().json(json!({
                "message": "Order cannot be deleted"
            }));
        }

        diesel::delete(orders.filter(user_id.eq(token_user_id)))
            .execute(conn)
            .expect("Error deleting order");

        HttpResponse::Ok().json(json!({
            "message": "Order deleted"
        }))

    } else {
        HttpResponse::Unauthorized().json(json!({
            "message": "No token found"
        }))
    }
}