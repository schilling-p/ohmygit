use error::AppError;
use infrastructure::diesel::DbPool;

pub async fn write_branch_to_database(pool: &DbPool, branch_name: &str) -> Result<(), AppError> {
    unimplemented!()
}