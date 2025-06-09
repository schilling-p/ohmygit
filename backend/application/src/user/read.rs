use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension};
use diesel::expression_methods::ExpressionMethods;
use uuid::Uuid;
use domain::user::User;
use domain::request::auth::UserIdentifier;
use infrastructure::diesel::DbPool;
use error::AppError;
use shared::state::AppState;

#[tracing::instrument(skip(pool))]
pub async fn get_user_role_for_repository(pool: &DbPool, id_user: Uuid, repo_id: Uuid) -> Result<Option<String>, AppError> {
    use domain::schema::user_repository_roles::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let user_role: Option<String> = conn
        .interact(move |conn| user_repository_roles
            .filter(repository_id.eq(repo_id))
            .filter(user_id.eq(id_user))
            .select(role)
            .first::<String>(conn)
            .optional())
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))??;
    Ok(user_role)
}