use axum::{extract::State, http::StatusCode, Json};
use diesel::{RunQueryDsl, SelectableHelper};
use diesel::query_dsl::QueryDsl;
use diesel::expression_methods::ExpressionMethods;
use domain::models::User;
use crate::user::internal_error;

pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users.select(User::as_select()).load::<User>(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn find_user_by_email(
    pool: State<deadpool_diesel::postgres::Pool>, user: User,
) -> Result<Json<User>, (StatusCode, String)> {
    use domain::schema::users::dsl::*;

    // TODO: solve this redeclaration in a Rust way
    let user_email = user.email.clone();
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users.filter(email.eq(user_email)).select(User::as_select()).first(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}