use axum::{extract::State, Json};
use axum_macros::debug_handler;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, OptionalExtension};
use diesel::expression_methods::ExpressionMethods;
use uuid::Uuid;
use domain::models::{User};
use domain::request::auth::UserIdentifier;
use infrastructure::diesel::DbPool;
use error::AppError;
use shared::state::AppState;

// TODO: remove for production
// this is purely for testing purposes
#[debug_handler]
pub async fn list_users(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<User>>, AppError> {
    use domain::schema::users::dsl::*;
    let pool = &app_state.db;
    let conn = pool.get().await?;
    let res = conn
        .interact(|conn| users.select(User::as_select()).load::<User>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(Json(res))
}

#[tracing::instrument(skip(pool))]
pub async fn retrieve_user_from_db(pool: &DbPool, identifier: UserIdentifier) -> Result<User, AppError> {
    use domain::schema::users::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let id_string = match identifier.clone() {
        UserIdentifier::Email(s) => s,
        UserIdentifier::Username(s) => s,
    };

    let user: User = conn
        .interact(move |conn| {
            match identifier {
                UserIdentifier::Email(_) => users.filter(email.eq(&id_string)).select(User::as_select()).first::<User>(conn),
                UserIdentifier::Username(_) => users.filter(username.eq(&id_string)).select(User::as_select()).first::<User>(conn),
            }
        })
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(user)
}

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