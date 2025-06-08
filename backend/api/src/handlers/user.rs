use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use domain::user::User;
use shared::state::AppState;
use error::AppError;

#[debug_handler]
pub async fn list_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = app_state.stores.users.list_users().await?;
    Ok(Json(users))
}