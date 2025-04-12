use std::env;
use std::error::Error;
use diesel::pg::PgConnection;
use deadpool_diesel::{postgres::{Manager, Pool}, Runtime};
use diesel::Connection;
use dotenvy::dotenv;
use diesel_migrations::{embed_migrations, MigrationHarness, EmbeddedMigrations};

pub type DbPool = Pool;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");
pub fn init_pool() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(db_url, Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .expect("Failed to create pool")
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut conn = establish_database_connection();
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

fn establish_database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to: {}", database_url))
}