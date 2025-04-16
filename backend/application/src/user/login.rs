use axum::extract::State;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use domain::models::User;
use axum::http::StatusCode;

pub async fn login_user(pool: deadpool_diesel::postgres::Pool, user: User) -> Result<(), (StatusCode, String)> {
    let user = find_user_by_email(State(pool), user).await?;
    verify_password(&user.hashed_pw, &user.hashed_pw).unwrap();
    Ok(())
}
