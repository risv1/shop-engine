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
