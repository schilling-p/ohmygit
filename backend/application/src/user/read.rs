use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl};
use diesel::expression_methods::ExpressionMethods;

use domain::models::User;
use infrastructure::diesel::DbPool;
use error::AppError;

// TODO: remove for production
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
pub async fn find_user_by_email(pool: &DbPool, user_email: &str) -> Result<Json<User>, AppError> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let email_owned = user_email.to_owned();
    let res = conn
        .interact(move |conn| users.filter(email.eq(email_owned)).select(User::as_select()).first::<User>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}

#[tracing::instrument(skip(pool))]
pub async fn find_user_by_username(pool: &DbPool, username: &str) -> Result<Json<User>, AppError> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let username_owned = username.to_owned();
    let res = conn
        .interact(move |conn| users.filter(username.eq(username_owned)).select(User::as_select()).first::<User>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}