use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use domain::models::{NewUser, User};
use domain::schema::users;
use crate::user::internal_error;
use shared::crypto::hash_password;
use error::AppError;

pub async fn create_user(State(pool): State<deadpool_diesel::postgres::Pool>, Json(new_user): Json<NewUser>,) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut user = new_user;

    // TODO: Do better error handling than unwrap() hint: use the new AppError
    user.hashed_pw = hash_password(&user.hashed_pw).unwrap();

    let conn = pool.get().await.map_err(|e|internal_error(e))?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(|e|internal_error(e))?
        .map_err(|e|internal_error(e))?;
    Ok(Json(res))
}