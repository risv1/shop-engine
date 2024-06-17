use actix_web::{HttpResponse};
use crate::models::product::Product;

pub async fn get_products() -> HttpResponse {
    let products: Vec<Product> = vec![
        Product {
            id: "1".to_string(),
            name: "Product 1".to_string(),
            price: 100.0,
            stock: 10,
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
        },
        Product {
            id: "2".to_string(),
            name: "Product 2".to_string(),
            price: 200.0,
            stock: 20,
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
        },
    ];

    HttpResponse::Ok().json(products)
}

pub async fn create_product() -> HttpResponse {
    let product: Product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        price: 100.0,
        stock: 10,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(product)
}

pub async fn get_product() -> HttpResponse {
    let product: Product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        price: 100.0,
        stock: 10,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(product)
}


pub async fn update_product() -> HttpResponse {
    let product: Product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        price: 100.0,
        stock: 10,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(product)
}

pub async fn delete_product() -> HttpResponse {
    HttpResponse::Ok().json("Product deleted")
}

