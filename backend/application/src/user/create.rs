use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use diesel::prelude::*;
use domain::models::{NewUser, User};
use domain::schema::users;
use shared::crypto::hash_password;
use error::AppError;
use log::debug;
use crate::user::read::find_user_by_email;

#[tracing::instrument(skip(pool))]
// TODO: change the return type of the function
pub async fn create_user(State(pool): State<deadpool_diesel::postgres::Pool>, Json(mut new_user): Json<NewUser>) -> Result<impl IntoResponse, AppError> {
    debug!("new_user: {:?}", new_user);
    match find_user_by_email(&pool, new_user.email.clone()).await {
        Ok(_) => return Err(AppError::EmailAlreadyExists),
        Err(AppError::NotFound(_)) => {},
        Err(e) => return Err{ 0: e },
    }

    new_user.hashed_pw = hash_password(&new_user.hashed_pw)?;

    let conn = pool.get().await.map_err(AppError::from)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}