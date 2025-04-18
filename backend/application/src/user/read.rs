use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper};
use diesel::query_dsl::QueryDsl;
use diesel::expression_methods::ExpressionMethods;
use domain::models::User;
use infrastructure::DbPool;
use error::AppError;
use tracing::debug;

#[debug_handler]
pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, AppError> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await?;
    let res = conn
        .interact(|conn| users.select(User::as_select()).load::<User>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}

#[tracing::instrument(skip(pool))]
pub async fn find_user_by_email(
    pool: &DbPool, user_email: String,
) -> Result<Json<User>, AppError> {
    debug!("LoginRequest: {:?}", user_email);
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let res = conn
        .interact(|conn| users.filter(email.eq(user_email)).select(User::as_select()).first(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}