use axum::extract::State;
use axum::Json;
use diesel::prelude::*;
use log::debug;

use crate::user::read::find_user_by_email;
use error::AppError;
use domain::ApiResponse;
use domain::models::{NewUser, User};
use domain::schema::users;
use domain::response::auth::SignupResponse;
use shared::crypto::hash_password;


#[tracing::instrument(skip(pool))]
pub async fn create_user(State(pool): State<deadpool_diesel::postgres::Pool>, Json(mut new_user): Json<NewUser>) -> Result<ApiResponse, AppError> {
    debug!("new_user: {:?}", new_user);
    match find_user_by_email(&pool, &new_user.email).await {
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

    Ok(ApiResponse::Signup(SignupResponse {
        message: "signup_successful",
        user_email: res.email,
    }))
}