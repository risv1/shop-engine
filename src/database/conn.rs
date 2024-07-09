use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    pool.get().expect("Failed to get connection")
}