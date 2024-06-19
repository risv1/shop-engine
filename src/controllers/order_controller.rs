use actix_web::{HttpResponse};
use crate::dto::order::Order;
use crate::dto::product::Product;

pub async fn get_cart() -> HttpResponse {
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

pub async fn add_to_cart() -> HttpResponse {
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

pub async fn delete_from_cart() -> HttpResponse {
    HttpResponse::Ok().json("Product deleted from cart")
}


pub async fn get_orders() -> HttpResponse {
    let orders: Vec<Order> = vec![
        Order {
            id: "1".to_string(),
            user_id: "1".to_string(),
            total: 100.0,
            items: vec![
                Product {
                    id: "1".to_string(),
                    name: "Product 1".to_string(),
                    price: 100.0,
                    stock: 10,
                    created_at: "2021-01-01".to_string(),
                    updated_at: "2021-01-01".to_string(),
                },
            ],
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
        },
        Order {
            id: "2".to_string(),
            user_id: "2".to_string(),
            items: vec![
                Product {
                    id: "2".to_string(),
                    name: "Product 2".to_string(),
                    price: 200.0,
                    stock: 20,
                    created_at: "2021-01-01".to_string(),
                    updated_at: "2021-01-01".to_string(),
                },
            ],
            total: 200.0,
            created_at: "2021-01-01".to_string(),
            updated_at: "2021-01-01".to_string(),
        },
    ];
    
    HttpResponse::Ok().json(orders)
}

pub async fn create_order() -> HttpResponse {
    let order: Order = Order {
        id: "1".to_string(),
        user_id: "1".to_string(),
        items: vec![
            Product {
                id: "1".to_string(),
                name: "Product 1".to_string(),
                price: 100.0,
                stock: 10,
                created_at: "2021-01-01".to_string(),
                updated_at: "2021-01-01".to_string(),
            },
        ],
        total: 100.0,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(order)
}

pub async fn get_order() -> HttpResponse {
    let order: Order = Order {
        id: "1".to_string(),
        user_id: "1".to_string(),
        items: vec![
            Product {
                id: "1".to_string(),
                name: "Product 1".to_string(),
                price: 100.0,
                stock: 10,
                created_at: "2021-01-01".to_string(),
                updated_at: "2021-01-01".to_string(),
            },
        ],
        total: 100.0,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(order)
}

pub async fn update_order() -> HttpResponse {
    let order: Order = Order {
        id: "1".to_string(),
        user_id: "1".to_string(),
        items: vec![
            Product {
                id: "1".to_string(),
                name: "Product 1".to_string(),
                price: 100.0,
                stock: 10,
                created_at: "2021-01-01".to_string(),
                updated_at: "2021-01-01".to_string(),
            },
        ],
        total: 100.0,
        created_at: "2021-01-01".to_string(),
        updated_at: "2021-01-01".to_string(),
    };

    HttpResponse::Ok().json(order)
}

pub async fn delete_order() -> HttpResponse {
    HttpResponse::Ok().json("Order deleted")
}