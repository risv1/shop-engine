use actix_web::{ HttpRequest, HttpResponse, web::{ Json, Path } };
use crate::dto::product::{ Product, NewProduct };
use crate::database::conn::establish_connection;
use serde_json::json;
use serde::Deserialize;
use diesel::{ RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods };
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewProductDto {
    name: String,
    price: f64,
    stock: i32,
    category_id: String,
    image: String,
    description: String,
}

#[derive(Deserialize)]
pub struct UpdateProductDto {
    name: String,
    price: f64,
    stock: i32,
    category_id: String,
    image: String,
    description: String,
}

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

pub async fn create_product(product_data: Json<NewProductDto>) -> HttpResponse {
    use crate::database::schema::product;

    let conn = &mut establish_connection();

    let new_product = NewProduct {
        id: Uuid::new_v4().to_string(),
        name: product_data.name.clone(),
        price: product_data.price,
        stock: product_data.stock,
        category_id: product_data.category_id.clone(),
        image: product_data.image.clone(),
        description: product_data.description.clone(),
        created_at: chrono::Local::now().naive_local().to_string(),
        updated_at: chrono::Local::now().naive_local().to_string(),
    };

    let insert_product = diesel
        ::insert_into(product::table)
        .values(&new_product)
        .get_result::<Product>(conn)
        .expect("Error inserting product");

    HttpResponse::Ok().json(
        json!({
            "message": "Product created",
            "product": new_product
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

pub async fn update_product(
    path: Path<String>,
    req: HttpRequest,
    update_product_data: Json<UpdateProductDto>
) -> HttpResponse {
    use crate::database::schema::product::dsl::*;

    let conn = &mut establish_connection();
    let product_id = path.into_inner();

    let get_product = product
        .filter(id.eq(product_id.clone()))
        .load::<Product>(conn)
        .expect("Error finding product");

    if get_product.len() == 0 {
        return HttpResponse::NotFound().json(
            json!({
                "message": "Product not found"
            })
        );
    }

    let update_product = diesel
        ::update(product.find(product_id))
        .set((
            name.eq(
                if !update_product_data.name.is_empty() {
                    update_product_data.name.clone()
                } else {
                    get_product[0].name.clone()
                }
            ),
            price.eq(
                if update_product_data.price.to_string().is_empty() {
                    get_product[0].price
                } else {
                    update_product_data.price
                }
            ),
            stock.eq(
                if update_product_data.stock.to_string().is_empty() {
                    update_product_data.stock
                } else {
                    get_product[0].stock
                }
            ),
            category_id.eq(
                if !update_product_data.category_id.is_empty() {
                    update_product_data.category_id.clone()
                } else {
                    get_product[0].category_id.clone()
                }
            ),
            image.eq(
                if !update_product_data.image.is_empty() {
                    update_product_data.image.clone()
                } else {
                    get_product[0].image.clone()
                }
            ),
            description.eq(
                if !update_product_data.description.is_empty() {
                    update_product_data.description.clone()
                } else {
                    get_product[0].description.clone()
                }
            ),
            updated_at.eq(
                chrono::Local::now().naive_local().to_string()
            ),
        ))
        .execute(conn);

    match update_product {
        Ok(_) => println!("Product updated successfully."),
        Err(e) => println!("Error updating product: {:?}", e),
    }

    HttpResponse::Ok().json(json!({
        "message": "Product updated",
    }))
}

pub async fn delete_product(path: Path<String>, req: HttpRequest) -> HttpResponse {
    use crate::database::schema::product::dsl::*;

    let conn = &mut establish_connection();
    let product_id = path.into_inner();

    let get_product = product
        .filter(id.eq(product_id.clone()))
        .load::<Product>(conn)
        .expect("Error finding product");

    if get_product.len() == 0 {
        return HttpResponse::NotFound().json(
            json!({
                "message": "Product not found"
            })
        );
    }

    let delete_product = diesel
        ::delete(product.find(product_id))
        .execute(conn)
        .expect("Error deleting product");

    HttpResponse::Ok().json(json!({
            "message": "Product deleted"
        }))
}
