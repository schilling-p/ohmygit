use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use domain::models::Repository;
use domain::request::auth::LoginRequest;
use domain::request::repo::FetchRepoRequest;
use error::AppError;
use crate::user::read::find_user_by_email;
use tracing::debug;

#[debug_handler]
pub async fn list_user_repositories(State(pool): State<deadpool_diesel::postgres::Pool>, Json(fetch_repo_request): Json<FetchRepoRequest>)
    -> Result<Json<Vec<Repository>>, AppError> {
    debug!("listing user repositories for: {:?}", &fetch_repo_request.user_email);
    let user = find_user_by_email(&pool, &fetch_repo_request.user_email).await?.0;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repos = conn
        .interact(move |conn| Repository::belonging_to(&user).select(Repository::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(repos))
}
