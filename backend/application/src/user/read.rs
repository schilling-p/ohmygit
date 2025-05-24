use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl};
use diesel::expression_methods::ExpressionMethods;

use domain::models::User;
use domain::request::auth::UserIdentifier;
use infrastructure::diesel::DbPool;
use error::AppError;

// TODO: remove for production
// this is purely for testing purposes
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
pub async fn retrieve_user_from_db(pool: &DbPool, identifier: UserIdentifier) -> Result<User, AppError> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let id_string = match identifier.clone() {
        UserIdentifier::Email(s) => s,
        UserIdentifier::Username(s) => s,
    };

    let user = conn
        .interact(move |conn| {
            match identifier {
                UserIdentifier::Email(_) => users.filter(email.eq(&id_string)).select(User::as_select()).first::<User>(conn),
                UserIdentifier::Username(_) => users.filter(username.eq(&id_string)).select(User::as_select()).first::<User>(conn),
            }
        })
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(user)
}