use std::env;
use deadpool_diesel::{postgres::{Manager, Pool}, Runtime};
use dotenvy::dotenv;

pub type DbPool = Pool;

pub fn init_pool() -> DbPool {

    assert!(
        dotenv().ok().is_some(),
        ".env file could not be loaded"
    );

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(db_url, Runtime::Tokio1);

    Pool::builder(manager)
        .build()
        .expect("Failed to create pool")
}