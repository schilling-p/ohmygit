use infrastructure::diesel::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}