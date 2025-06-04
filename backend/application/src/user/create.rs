use axum::extract::State;
use axum::Json;
use diesel::prelude::*;
use tracing::debug;

use crate::user::read::retrieve_user_from_db;
use error::AppError;
use domain::ApiResponse;
use domain::models::{NewUser, User};
use domain::schema::users;
use domain::response::auth::SignupResponse;
use domain::request::auth::UserIdentifier;
use shared::crypto::hash_password;


#[tracing::instrument(skip(pool))]
pub async fn create_user(State(pool): State<deadpool_diesel::postgres::Pool>, Json(mut new_user): Json<NewUser>) -> Result<ApiResponse, AppError> {
    debug!("new_user: {:?}", new_user);
    // TODO: optimization of the new_user.email.clone()
    match retrieve_user_from_db(&pool, UserIdentifier::Email(new_user.email.clone())).await {
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