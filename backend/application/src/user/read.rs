use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use diesel::SelectableHelper;
use domain::models::User;
use domain::schema::users;
use crate::user::internal_error;

pub async fn list_users(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| users::table.select(User::as_select()).load::<User>(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}