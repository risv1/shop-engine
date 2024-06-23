use actix_web::{ HttpRequest, HttpResponse, web::{ Json, Path } };
use crate::dto::product::{ Product, NewProduct };
use crate::database::conn::establish_connection;
use serde_json::json;
use serde::Deserialize;
use diesel::{ RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods };
use uuid::Uuid;

pub async fn get_products(req: HttpRequest) -> HttpResponse {
    use crate::database::schema::product::dsl::*;

    let conn = &mut establish_connection();

    let get_products = product.load(conn).expect("Error finding products");

    let products: &Vec<Product> = &get_products;

    if products.len() == 0 {
        HttpResponse::NotFound().json(
            json!({
                "message": "No products found"
            })
        );
    }

    HttpResponse::Ok().json(
        json!({
            "message": "Products found",
            "products": products
        })
    )
}

pub async fn get_product(path: Path<String>, req: HttpRequest) -> HttpResponse {
    use crate::database::schema::product::dsl::*;

    let conn = &mut establish_connection();
    let get_product_id = path.into_inner();

    let get_product_table = product
        .filter(id.eq(get_product_id))
        .load(conn)
        .expect("Error finding product");

    let get_product: Vec<Product> = get_product_table;

    if get_product.len() == 0 {
        HttpResponse::NotFound().json(
            json!({
                "message": "Product not found"
            })
        );
    }

    HttpResponse::Ok().json(
        json!({
            "message": "Product found",
            "product": get_product
        })
    )
}

